use rand::Rng;
use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug)]
pub struct Cell {
    pub is_uncovered: bool, // left mouse button action
    pub is_flagged: bool,   // right mouse button action
    pub is_mine: bool,      // hidden state
    pub hint_number: u8,    // hint number to display in case of uncover/flood
}

impl Cell {
    pub fn new(is_mine: bool) -> Self {
        Cell {
            is_uncovered: true,
            is_flagged: false,
            is_mine,
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
                b.cells
                    .insert(pos, Cell::new(mine_positions.contains(&pos)));
            }
        }

        for y in 0..size.1 {
            for x in 0..size.0 {
                let pos = (x, y);
                let count = b.count_neighbors(&pos);
                let cell = b.get_cell_mut(&pos).unwrap();
                if cell.is_mine {
                    continue;
                }
                cell.hint_number = count;
            }
        }

        b
    }
    pub fn get_cell(self: &Self, pos: &(u8, u8)) -> Option<&Cell> {
        self.cells.get(pos)
    }
    pub fn get_cell_mut(self: &mut Self, pos: &(u8, u8)) -> Option<&mut Cell> {
        self.cells.get_mut(pos)
    }
    fn count_neighbors(self: &Self, pos: &(u8, u8)) -> u8 {
        let mut count: u8 = 0;
        for y in -1..=1i8 {
            for x in -1..=1i8 {
                if x == 0 && y == 0 {
                    continue;
                }
                let xx = (x + (pos.0 as i8)) as u8;
                let yy = (y + (pos.1 as i8)) as u8;
                let cell = self.get_cell(&(xx, yy));
                match cell {
                    Some(x) if x.is_mine => {
                        count += 1;
                    }
                    _ => (),
                }
            }
        }
        count
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut st = String::new();
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let pos = (x, y);
                let cell = self.get_cell(&pos).unwrap();
                if cell.is_uncovered {
                    if cell.is_mine {
                        st.push('M');
                    } else if cell.hint_number == 1 {
                        st.push('1');
                    } else if cell.hint_number == 2 {
                        st.push('2');
                    } else if cell.hint_number == 3 {
                        st.push('3');
                    } else if cell.hint_number == 4 {
                        st.push('4');
                    } else if cell.hint_number == 5 {
                        st.push('5');
                    } else if cell.hint_number == 6 {
                        st.push('6');
                    } else if cell.hint_number == 7 {
                        st.push('7');
                    } else if cell.hint_number == 8 {
                        st.push('8');
                    } else {
                        st.push('_');
                    }
                } else {
                    if cell.is_flagged {
                        st.push('F');
                    } else {
                        st.push('?');
                    }
                }
            }
            st.push('\n');
        }
        write!(f, "{}", st)
    }
}
