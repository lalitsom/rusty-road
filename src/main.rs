use minifb::{Key, Window, WindowOptions};
mod config;
mod game_state;
mod game_window;
mod utils;

fn main() {
    let mut window = game_window::initialize_window();
    let mut current_state = game_state::initial_state();
    let mut event: (i32, i32) = (0, 0);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        (window, event) = game_window::captureEvents(window);
        current_state = game_state::update_player_position(current_state);
        current_state = game_state::game_tick(current_state, event);
        window = refresh_screen(&current_state, window);
        utils::delay_for_miliseconds(5);
    }
}

fn refresh_screen(current_state: &game_state::GameState, mut window: Window) -> Window {
    window
        .update_with_buffer(&current_state.buffer, config::WIDTH, config::HEIGHT)
        .unwrap();
    window
}
