use minifb::{Key, Window, WindowOptions};
use crate::config;

pub fn initialize_window() -> (Window) {
    let mut window = Window::new(
        "Rusty Road - ESC to exit",
        config::WIDTH,
        config::HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    window
}

pub fn captureEvents(window : Window) -> (Window, (i32,i32)){ // x,y
    let mut event : (i32,i32) = (0,0);
    let step_size=10;
    if window.is_key_down(Key::Up){
        event.1 -= step_size;
    }
    if window.is_key_down(Key::Down){
        event.1 += step_size;
    }
    if window.is_key_down(Key::Right){
        event.0 += step_size;
    }
    if window.is_key_down(Key::Left){
        event.0 -= step_size;
    }
    (window, (event.0, event.1))
}