pub enum Screens {
    Ready,
    Running,
    Over,
}

pub struct GameState {
    pub buffer: Vec<u32>,
    pub game_state: Screens,
    pub player_position: (i32,i32),
    pub player_size : i32,
    pub obstacles: [(i32,i32,i32); 30],
    pub score: u32,
    pub speed: u32,
    pub obstacle_gen_rate: u32
}


pub fn initial_state() -> GameState{
    let game_state = GameState {
        buffer : vec![0; 0 * 0],
        game_state : Screens::Ready,
        player_position : (500,500),
        player_size : 5,
        obstacles : [(0,0,0); 30],
        score : 0,
        speed : 10,
        obstacle_gen_rate : 10
    };
    game_state

}