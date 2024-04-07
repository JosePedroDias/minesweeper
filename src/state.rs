use std::collections::HashMap;

#[derive(Debug)]
pub struct Cell {
    pub uncovered: bool, // left mouse button action
    pub is_flagged: bool, // right mouse button action
    pub has_mine: bool, // hidden state
    pub hint_number: u8, // hint number to display in case of uncover/flood
}

impl Cell {
    pub fn new(has_mine: bool) -> Self {
        Cell {
            uncovered: false,
            is_flagged: false,
            has_mine,
            hint_number: 0,
        }
    }
}

#[derive(Debug)]
pub struct Board {
    //start_time:
    cells: HashMap<(u8, u8), Cell>,
    game_ended: bool,
    has_won: bool,
}

impl Board {
    pub fn new(size: (u8, u8), num_mines: u16) -> Self {
        Board {
            cells: HashMap::new(),
            game_ended: false,
            has_won: false,
        }
    }
}
