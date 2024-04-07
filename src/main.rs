mod state;

use state::*;

fn main() {
    println!("Hello, world!");
    let b = Board::new((16, 16), 32);
    println!("{:#?}", b);
}
