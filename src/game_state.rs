use crate::config;
use rand::Rng;

#[derive(Debug, PartialEq)]
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

pub fn initialize_state() -> GameState {
    let game_state = GameState {
        buffer: vec![0; config::WIDTH * config::HEIGHT],
        game_state: Screens::Ready,
        player_position: (500, 500), // x,y
        player_size: 5,
        obstacles: [(0, 0, 10); 30], // x,y, radius
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

pub fn game_tick(mut current_state: GameState, event: (i32, i32)) -> (GameState, bool) {
    // update player position
    current_state = clear_screen_buffer(current_state);

    current_state.player_position.0 =
        update_normalized_pos(current_state.player_position.0, event.0, config::WIDTH);
    current_state.player_position.1 =
        update_normalized_pos(current_state.player_position.1, event.1, config::HEIGHT);

    current_state = update_player_position(current_state);
    let mut is_collided = false;
    // update obstacles position as per speed
    (current_state, is_collided) = update_obstacles(current_state);
    // println!("obstacle1 : {:?}", current_state.obstacles[0]);
    // println!("player : {:?}", current_state.player_position);
    // update score
    current_state.score += 1;
    // println!("game_state : {:?}", is_collided);
    // current_state.game_state = Screens::Running;
    // check collisions
    if is_collided == true {
        current_state.game_state = Screens::Over;
        // println!("game_state : {:?}", is_collided);
    }
    // update game state

    (current_state, is_collided)
}

pub fn update_player_position(mut current_state: GameState) -> GameState {
    current_state.buffer = put_on_buffer(
        current_state.buffer,
        (
            current_state.player_position.0,
            current_state.player_position.1,
        ),
        current_state.player_size,
        config::PLAYER_COLOR,
    );
    current_state
}

pub fn put_on_buffer(
    mut buffer: Vec<u32>,
    (entity_x, entity_y): (i32, i32),
    radius: i32,
    color: u32,
) -> Vec<u32> {
    for x in -radius..=radius {
        for y in -radius..=radius {
            let index = ((entity_y + y) * (config::WIDTH as i32) + (entity_x + x)) as usize;
            if index >= 0 && index < buffer.len() {
                buffer[index] = color;
            }
        }
    }
    buffer
}

fn update_obstacles(mut current_state: GameState) -> (GameState, bool) {
    let (obstacle_count, obstacle_speed) = get_obstacle_speed_count(current_state.score);
    let max_iterations = std::cmp::min(obstacle_count, current_state.obstacles.len() as u32);
    let mut is_collided = false;
    for i in 0..max_iterations {
        let (x, y, r) = get_obs_new_params(current_state.obstacles[i as usize]);

        current_state.obstacles[i as usize] = (x, y, r);
        is_collided = detect_collisions(
            current_state.obstacles[i as usize],
            current_state.player_position,
        );
        if is_collided == true {
            break;
        }
        current_state.buffer = put_on_buffer(
            current_state.buffer,
            (
                current_state.obstacles[i as usize].0,
                current_state.obstacles[i as usize].1,
            ),
            current_state.obstacles[i as usize].2,
            config::OBSTACLE_COLOR,
        );
    }
    (current_state, is_collided)
}

fn get_obstacle_speed_count(score: u32) -> (u32, u32) {
    (score / 100, (score / 10))
}

fn update_normalized_pos(pos: i32, inc: i32, bound: usize) -> i32 {
    if (pos + inc) < 0 {
        return (bound - 1) as i32;
    }
    if (pos + inc) >= (bound as i32) {
        return 1;
    }
    pos + inc
}

fn get_obs_new_params((x, y, r): (i32, i32, i32)) -> (i32, i32, i32) {
    if x == 0 && y == 0 {
        let random_number = rand::thread_rng().gen_range(3..=25);
        return (
            update_normalized_pos(x, 1, config::WIDTH),
            update_normalized_pos(y, 1, config::HEIGHT),
            random_number,
        );
    }
    let speed = (30 - r) / 2;
    (
        update_normalized_pos(x, speed, config::WIDTH),
        update_normalized_pos(y, speed, config::HEIGHT),
        r,
    )
}

fn detect_collisions((x, y, r): (i32, i32, i32), (px, py): (i32, i32)) -> bool {
    let half_width = r;
    let half_height = r; // Assuming squares, so width and height are the same
    let half_width_p = 5;
    let half_height_p = 5; // Assuming squares, so width and height are the same

    // Calculate the distance between the centers of the two squares
    let dx = (x - px).abs();
    let dy = (y - py).abs();

    // Check for collision by comparing distances with half-widths
    if dx <= half_width + half_width_p && dy <= half_height + half_height_p {
        return true; // Collided
    }

    false // No collision
}
