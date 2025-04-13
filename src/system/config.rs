pub type WindowType = i32;
pub const WINDOWED: WindowType = 0;
pub const FULLSCREEN: WindowType = 1;
pub const BORDERLESS: WindowType = 2;

pub struct Config {
    pub window_type: WindowType,
    pub save_resolution: bool,
    pub resolution_x: i32,
    pub resolution_y: i32,
    pub save_window_size: bool,
    pub window_size_x: i32,
    pub window_size_y: i32,
    pub save_window_pos: bool,
    pub window_pos_x: i32,
    pub window_pos_y: i32,
    pub vsync: bool,
    pub max_fps: i32, // if max_fps is 0, fps is unlimited
    pub song_paths: Vec<String>,
    pub table_urls: Vec<String>,
}

impl Config {
    pub fn new() -> Config {
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
