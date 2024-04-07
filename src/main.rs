mod state;

use state::*;

fn main() {
    println!("Hello, world!");
    let mut b = Board::new((16, 16), 32);
    //println!("{:#?}", b);
    println!("{}---", b);

    b.flag(&(0, 0));
    println!("{}---", b);

    b.uncover(&(1, 0));
    println!("{}---", b);

    println!("won? {}", b.has_won());
    println!("ended? {}", b.game_ended);
}
