use crate::config;

pub enum Screens {
    Ready,
    Running,
    Over,
}

pub struct GameState {
    pub buffer: Vec<u32>,
    pub game_state: Screens,
    pub player_position: (i32, i32),
    pub player_size: i32,
    pub obstacles: [(i32, i32, i32); 30],
    pub score: u32,
    pub speed: u32,
    pub obstacle_gen_rate: u32,
}

pub fn initial_state() -> GameState {
    let game_state = GameState {
        buffer: vec![0; config::WIDTH * config::HEIGHT],
        game_state: Screens::Ready,
        player_position: (500, 500),
        player_size: 5,
        obstacles: [(0, 0, 0); 30],
        score: 0,
        speed: 10,
        obstacle_gen_rate: 10,
    };
    game_state
}

pub fn clear_screen_buffer(mut current_state: GameState) -> GameState {
    for i in current_state.buffer.iter_mut() {
        *i = config::SKY_BLUE_COLOR;
    }
    current_state
}

pub fn game_tick(
    mut current_state: GameState,
    event: (i32, i32),
) -> GameState {
    // update player position
    current_state = clear_screen_buffer(current_state);
    current_state.player_position.0 += event.0;
    current_state.player_position.1 += event.1;

    current_state = update_player_position(current_state);
    // update obstacles position as per speed

    // update score
    current_state.score += 1;

    // check collisions

    // update game state
    current_state.game_state = Screens::Running;

    current_state
}


pub fn update_player_position(mut current_state: GameState) -> GameState {
    
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