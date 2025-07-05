pub fn compute_position(
    position: &str,
    screen_width: u16,
    screen_height: u16,
    win_width: u16,
    win_height: u16,
    margin_x: i32,
    margin_y: i32,
) -> (i32, i32) {
    let (x, y) = match position {
        "TL" => (0, 0),
        "T" => ((screen_width as i32 - win_width as i32) / 2, 0),
        "TR" => ((screen_width as i32 - win_width as i32), 0),
        "L" => (0, (screen_height as i32 - win_height as i32) / 2),
        "C" => ((screen_width as i32 - win_width as i32) / 2, (screen_height as i32 - win_height as i32) / 2),
        "R" => ((screen_width as i32 - win_width as i32), (screen_height as i32 - win_height as i32) / 2),
        "BL" => (0, (screen_height as i32 - win_height as i32)),
        "B" => ((screen_width as i32 - win_width as i32) / 2, (screen_height as i32 - win_height as i32)),
        "BR" => ((screen_width as i32 - win_width as i32), (screen_height as i32 - win_height as i32)),
        _ => (0, 0),
    };

    (x + margin_x, y + margin_y)
}