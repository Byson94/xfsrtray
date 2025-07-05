// using external modules
use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::protocol::Event;
use x11rb::rust_connection::RustConnection;
use x11rb::{COPY_FROM_PARENT, CURRENT_TIME};
use std::collections::HashMap;
use std::mem::transmute;
use std::sync::atomic::{AtomicBool, Ordering};
// use ctrlc;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use x11rb::protocol::xproto::EventMask;
use std::collections::HashSet;
use std::time::Instant;
use signal_hook::consts::signal::*;
use signal_hook::iterator::Signals;
use x11rb::protocol::xproto::PropMode;

// import local modules
mod args;
mod position_calc;

// using local modules
use args::parse_args;
use position_calc::compute_position;
use args::Args;

fn handle_applet_removal(
    window: Window,
    conn: &RustConnection,
    win_id: Window,
    docked_windows: &mut HashMap<Window, i16>,
    args: &Args,
    next_x: &mut i16,
    screen: &Screen,
    height_u16: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    if docked_windows.remove(&window).is_some() {
        *next_x = args.padding as i16;

        for (win, _) in docked_windows.iter_mut() {
            conn.configure_window(*win, &ConfigureWindowAux::new().x(Some(*next_x as i32)))?;
            *next_x += args.tray_height as i16;
        }

        if args.set_to_item_width {
            let new_width = (*next_x as u32) + (args.padding as u32);
            conn.configure_window(win_id, &ConfigureWindowAux::new().width(Some(new_width)))?;

            let new_width_u16: u16 = new_width.try_into().unwrap_or(200);
            let (x, y) = compute_position(
                &args.position,
                screen.width_in_pixels,
                screen.height_in_pixels,
                new_width_u16,
                height_u16,
                args.margin_x,
                args.margin_y,
            );
            conn.configure_window(win_id, &ConfigureWindowAux::new().x(x).y(y))?;
        }

        conn.flush()?;
    }

    Ok(())
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_args();
    println!("{:?}", args);

    let (conn, screen_num) = RustConnection::connect(None)?;
    let screen = &conn.setup().roots[screen_num];
    let root = screen.root;

    let win_id = conn.generate_id()?;

    // move paddng to u32 for width calculation
    let padding = args.padding as u32;
    let icon_size_u16 = args.tray_height as u16;
    let icon_size_i16 = icon_size_u16 as i16;
    let padding_u16 = args.padding as u16;

    let width_u32 = if args.set_to_item_width {
        // just a starting value
        1 + padding * 2 // 1 is added here to prevent 0*2 happening
    } else {
        args.tray_width as u32 + padding * 2
    };
    let width = width_u32.try_into().unwrap_or(200u16);

    let height = icon_size_u16 + padding_u16 * 2;

    let background_pixel = if let Some(color_str) = &args.background_color {
        u32::from_str_radix(color_str.trim_start_matches("0x"), 16).unwrap_or(screen.white_pixel)
    } else {
        screen.white_pixel
    };

    conn.create_window(
        COPY_FROM_PARENT as u8,
        win_id,
        root,
        0,
        0,
        width,
        height,
        0,
        WindowClass::INPUT_OUTPUT,
        screen.root_visual,
        &CreateWindowAux::new()
            .override_redirect(1)
            .background_pixel(background_pixel),
    )?;

    let width_u16: u16 = width.try_into().expect("Width too large to fit in u16");
    let height_u16: u16 = height.try_into().expect("Height too large to fit in u16");

    let (x, y) = compute_position(
        &args.position,
        screen.width_in_pixels,
        screen.height_in_pixels,
        width_u16,
        height_u16,
        args.margin_x,
        args.margin_y,
    );

    conn.configure_window(win_id, &ConfigureWindowAux::new().x(x).y(y))?;
    conn.map_window(win_id)?;
    conn.flush()?;
    println!("Tray window created: {}", win_id);

    // Set WM_CLASS so compositors and window managers can categorize the window
    let wm_class_atom = conn.intern_atom(false, b"WM_CLASS")?.reply()?.atom;
    let class_str = b"xfsrtray\0xfsrtray\0";
    conn.change_property(
        PropMode::REPLACE,
        win_id,
        wm_class_atom,
        AtomEnum::STRING,
        8,
        class_str.len() as u32,
        class_str,
    )?;

    // Set up atoms
    let tray_atom_str = format!("_NET_SYSTEM_TRAY_S{}", screen_num);
    let tray_atom = conn.intern_atom(false, tray_atom_str.as_bytes())?.reply()?.atom;
    let manager_atom = conn.intern_atom(false, b"MANAGER")?.reply()?.atom;
    let opcode_atom = conn.intern_atom(false, b"_NET_SYSTEM_TRAY_OPCODE")?.reply()?.atom;

    // Claim selection
    let current_owner = conn.get_selection_owner(tray_atom)?.reply()?.owner;
    if current_owner != x11rb::NONE {
        eprintln!("Another systray is already running (owner: {}). Exiting.", current_owner);
        return Ok(());
    }

    conn.set_selection_owner(win_id, tray_atom, CURRENT_TIME)?;
    conn.flush()?;
    println!("Claimed system tray selection.");

    let sel_owner = conn.get_selection_owner(tray_atom)?.reply()?.owner;
    if sel_owner != win_id {
        panic!("Failed to acquire system tray selection! {}", win_id);
    }

    let data_u32: [u32; 5] = [CURRENT_TIME, tray_atom, win_id, 0, 0];
    let data_bytes: [u8; 20] = unsafe { transmute(data_u32) };
    let manager_event = ClientMessageEvent {
        response_type: CLIENT_MESSAGE_EVENT,
        format: 32,
        sequence: 0,
        window: root,
        type_: manager_atom,
        data: ClientMessageData::from(data_bytes),
    };

    conn.send_event(false, root, EventMask::STRUCTURE_NOTIFY, manager_event)?;
    conn.flush()?;
    println!("Sent MANAGER client message.");

    let mut docked_windows = HashMap::new();
    let mut next_x: i16 = args.padding as i16;

    // Close handler (SIGINIT for ^C and SIGTERM for detecting something like `killall`)
    let running = Arc::new(AtomicBool::new(true));
    let mut signals = Signals::new(&[SIGINT, SIGTERM])?;
    let r = running.clone();

    // Spawn a thread to listen for termination signals
    std::thread::spawn(move || {
        for _ in signals.forever() {
            r.store(false, Ordering::SeqCst);
            break;
        }
    });


    let mut last_check = Instant::now();

    // Main event loop
    while running.load(Ordering::SeqCst) {
        if let Ok(Some(event)) = conn.poll_for_event() {
            match event {
                Event::ClientMessage(ev) => {
                    if ev.type_ == opcode_atom && ev.format == 32 {
                        let data = ev.data.as_data32();
                        let opcode = data[1];
                        if opcode == 0 {
                            let icon_window = data[2];
                            println!("Dock request for window: {}", icon_window);

                            if conn.get_window_attributes(win_id).is_err() {
                                eprintln!("Dock request for non-existent window: {win_id}");
                                continue;
                            }

                            conn.reparent_window(icon_window, win_id, next_x, padding_u16 as i16)?;
                            conn.map_window(icon_window)?;

                            sleep(Duration::from_millis(10));

                            let icon_size_u32 = icon_size_u16 as u32;

                            conn.configure_window(
                                icon_window,
                                &ConfigureWindowAux::new().width(icon_size_u32).height(icon_size_u32),
                            )?;

                            docked_windows.insert(icon_window, next_x);

                            // Move next_x for the next icon
                            next_x += icon_size_i16;

                            if args.set_to_item_width {
                                let new_width = (next_x as u32) + (args.padding as u32);
                                conn.configure_window(win_id, &ConfigureWindowAux::new().width(Some(new_width)))?;

                                // reposition with new width
                                let new_width_u16: u16 = new_width.try_into().unwrap_or(width_u16);
                                let (x, y) = compute_position(
                                    &args.position,
                                    screen.width_in_pixels,
                                    screen.height_in_pixels,
                                    new_width_u16,
                                    height_u16,
                                    args.margin_x,
                                    args.margin_y,
                                );
                            
                                conn.configure_window(win_id, &ConfigureWindowAux::new().x(x).y(y))?;
                            }
                            
                            conn.flush()?;
                        }
                    }
                }
                _ => {
                    println!("Other Event: {:?}", event);
                }
            }
        } else {
            sleep(Duration::from_millis(10));
        }

        if last_check.elapsed() >= Duration::from_secs(1) {
            match conn.query_tree(win_id)?.reply() {
                Ok(tree) => {
                    let current_children: HashSet<Window> = tree.children.into_iter().collect();

                    let stale_windows: Vec<Window> = docked_windows
                        .keys()
                        .filter(|win| !current_children.contains(win))
                        .copied()
                        .collect();

                    for win in stale_windows {
                        let _ = handle_applet_removal(
                            win,
                            &conn,
                            win_id,
                            &mut docked_windows,
                            &args,
                            &mut next_x,
                            &screen,
                            height_u16,
                        );
                    }
                }
                Err(e) => {
                    eprintln!("query_tree failed: {:?}", e);
                }
            }

            last_check = Instant::now();
        }
    }

    // Cleanup
    println!("Starting closing systray sequence...");
    conn.set_selection_owner(x11rb::NONE, tray_atom, CURRENT_TIME)?;

    let timestamp = CURRENT_TIME;
    let manager_event = ClientMessageEvent {
        response_type: CLIENT_MESSAGE_EVENT,
        format: 32,
        sequence: 0,
        window: root,
        type_: manager_atom,
        data: ClientMessageData::from([
            timestamp,
            tray_atom,
            0,
            0,
            0,
        ]),
    };

    conn.send_event(false, root, EventMask::STRUCTURE_NOTIFY, manager_event)?;
    conn.flush()?;

    // Unmap and reparent docked icons to root before exiting
    for (win, _) in &docked_windows {
        println!("{}", *win);
        let _ = conn.change_window_attributes(
            *win,
            &ChangeWindowAttributesAux::new().event_mask(EventMask::NO_EVENT),
        );
        let _ = conn.unmap_window(*win);
        std::thread::sleep(std::time::Duration::from_millis(10)); // delay to prevent bugs
        let _ = conn.reparent_window(*win, screen.root, 0, 0);
    }

    conn.flush()?;

    //println!("Destroying tray window...");
    //match conn.destroy_window(win_id) {
    //    Ok(_) => println!("Tray window destroyed"),
    //    Err(e) => println!("Error destroying tray window: {:?}", e),
    //}

    sleep(Duration::from_millis(10));

    conn.flush()?;

    Ok(())
}
