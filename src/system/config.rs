pub type WindowType = i32;
pub const WINDOWED: WindowType = 0;
pub const FULLSCREEN: WindowType = 1;
pub const BORDERLESS: WindowType = 2;

struct Config {
    window_type: WindowType,
    save_resolution: bool,
    resolution_x: i32,
    resolution_y: i32,
    save_window_size: bool,
    window_size_x: i32,
    window_size_y: i32,
    save_window_pos: bool,
    window_pos_x: i32,
    window_pos_y: i32,
    vsync: bool,
    max_fps: i32, // if max_fps is 0, fps is unlimited

    song_paths: Vec<String>,
    table_urls: Vec<String>,
}

impl Config {
    fn new() -> Config {
        Config {
            window_type: WINDOWED,
            save_resolution: true,
            resolution_x: 1920,
            resolution_y: 1080,
            save_window_size: true,
            window_size_x: 1920,
            window_size_y: 1080,
            save_window_pos: true,
            window_pos_x: 400,
            window_pos_y: 400,
            vsync: false,
            max_fps: 0, // if max_fps is 0, fps is unlimited
            song_paths: Vec::new(),
            table_urls: Vec::new(),
        }
    }
}
