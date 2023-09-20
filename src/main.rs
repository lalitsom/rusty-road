use minifb::{Key, Window};
mod config;
mod game_state;
mod game_window;
mod utils;

fn main() {
    let mut window = game_window::initialize_window();
    let mut current_state = game_state::initialize_state();
    let mut event: (i32, i32) = (0, 0);
    let mut is_over: bool = false;
    while window.is_open() && !window.is_key_down(Key::Escape) {
            (window, event) = game_window::captureEvents(window);
            current_state = game_state::update_player_position(current_state);
            (current_state, is_over) = game_state::game_tick(current_state, event);
            window = refresh_screen(&current_state, window);
            if is_over{
                println!("score is {:?}",current_state.score);
                utils::delay_for_miliseconds(500);
                break;
            }

        utils::delay_for_miliseconds(5);
    }
}

fn refresh_screen(current_state: &game_state::GameState, mut window: Window) -> Window {
    // println!("score is {:?}", current_state.score,);
    window
        .update_with_buffer(&current_state.buffer, config::WIDTH, config::HEIGHT)
        .unwrap();
    window
}
