use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Background color in RGBA hexadecimal number  (0x000000 for black)
    #[arg(long)]
    pub background_color: Option<String>,

    /// TL, T, TR, L, C, R, BL, B, BR
    #[arg(long, default_value = "BR")]
    pub position: String,

    /// Tray width (pixels)
    #[arg(long, default_value_t = 200)]
    pub tray_width: i32,

    /// Tray height (pixels)
    #[arg(long, default_value_t = 24)]
    pub tray_height: i32,

    /// X Margin outisde the tray window (negative values can be used)
    #[arg(long, default_value_t = 0)]
    pub margin_x: i32,

    /// Y Margin outisde the tray window (negative values can be used)
    #[arg(long, default_value_t = 0)]
    pub margin_y: i32,

    /// Padding inside the tray window (pixels)
    #[arg(long, default_value_t = 0)]
    pub padding: i32,

    /// Make tray width match content width (shrink-wrap)
    #[arg(long)]
    pub set_to_item_width: bool,

}

pub fn parse_args() -> Args {
    Args::parse()
}
