use std::collections::{HashMap, HashSet};
use rand::Rng;

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
    pub size: (u8, u8),
    pub game_ended: bool,
    pub has_won: bool,
    cells: HashMap<(u8, u8), Cell>,
}

impl Board {
    pub fn new(size: (u8, u8), num_mines: u16) -> Self {
        let mut b = Board {
            size,
            cells: HashMap::new(),
            game_ended: false,
            has_won: false,
        };
        
        let mut rng = rand::thread_rng();
        let mut mine_positions: HashSet<(u8, u8)> = HashSet::new();
        for _ in 0..num_mines {
            loop {
                let x = rng.gen_range(0..size.0);
                let y = rng.gen_range(0..size.1);
                let pos = (x, y);
                if !mine_positions.contains(&pos) {
                    mine_positions.insert(pos);
                    break;
                }
            }
        }
        
        for y in 0..size.1 {
            for x in 0..size.0 {
                let pos = (x, y);
                b.cells.insert(pos, Cell::new(mine_positions.contains(&pos)));
            }
        }
        b
    }
}
