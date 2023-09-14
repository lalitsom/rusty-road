use minifb::{Key, Window, WindowOptions};
mod config;
mod game_state;
mod utils;

fn main() {
    let (mut window, buffer) = initialize_window();
    let mut current_state = game_state::initial_state();
    let mut event : (i32,i32) = (0,0);

    current_state.buffer = buffer;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        (window, event) = captureEvents(window);
        current_state = update_player_position(current_state);
        current_state = game_tick(current_state, event);
        (current_state, window) = refresh_screen(current_state, window);
        // wait();
    }
}

fn wait() {
    utils::delay_for_miliseconds(50000);
}


fn refresh_screen(
    current_state: game_state::GameState,
    mut window: Window,
) -> (game_state::GameState, Window) {
    let x = current_state.player_position.0;
    println!("Refreshing screen {x}");

    window
        .update_with_buffer(&current_state.buffer, config::WIDTH, config::HEIGHT)
        .unwrap();
    (current_state, window)
}

fn initialize_window() -> (Window, Vec<u32>) {
    let buffer: Vec<u32> = vec![0; config::WIDTH * config::HEIGHT];
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
    (window, buffer)
}

fn captureEvents(window : Window) -> (Window, (i32,i32)){ // x,y
    let mut event : (i32,i32) = (0,0);
    if window.is_key_down(Key::Up){
        event.1 -= 1;
    }
    if window.is_key_down(Key::Down){
        event.1 += 1;
    }
    if window.is_key_down(Key::Right){
        event.0 += 1;
    }
    if window.is_key_down(Key::Left){
        event.0 -= 1;
    }
    (window, (event.0, event.1))
}


fn game_tick(mut current_state: game_state::GameState, event : (i32, i32)) -> game_state::GameState {

    // update player position
    current_state = clearScreen(current_state);
    current_state.player_position.0 += event.0;
    current_state.player_position.0 += event.1;

    current_state = update_player_position(current_state);
    // update obstacles position as per speed


    // update score
    current_state.score += 1;

    // check collisions


    // update game state
    current_state.game_state = game_state::Screens::Running;

    current_state
}


fn clearScreen(mut current_state: game_state::GameState) -> game_state::GameState {
    for i in current_state.buffer.iter_mut() {
        *i = config::SKY_BLUE_COLOR;
    }
    current_state
}

fn update_player_position(mut current_state: game_state::GameState) -> game_state::GameState {
    
    for x in -current_state.player_size..=current_state.player_size {
        for y in -current_state.player_size..=current_state.player_size {
            let player_x = current_state.player_position.0 + x;
            let player_y = current_state.player_position.1 + y;
            let index = ((player_y * (config::WIDTH as i32) + player_x)) as usize;
            current_state.buffer[index] = config::PLAYER_COLOR;
        }
    }

    current_state
}