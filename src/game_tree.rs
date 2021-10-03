use crate::board::{*, self};

pub enum udlr {
    u,
    d,
    l,
    r
}
pub struct game_state {
    factor: udlr,
    cost: u8,
    state: board::board
}

pub struct game_tree {
    
}

fn alg(start: game_state, goal: game_state) {
    let mut tree = game_state {factor: udlr::d, cost: 3, state: board::board::init(3)};
}