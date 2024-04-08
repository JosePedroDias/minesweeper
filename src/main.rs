mod state;

use comfy::*;
use state::*;
use std::process::exit;

const W: u8 = 16;
const H: u8 = 16;
const NUM_MINES: u16 = 32;
const SPRITE_W: f32 = 16.0;

// fn main() {
//     println!("Hello, world!");
//     let mut b = Board::new((16, 16), 32);
//     //println!("{:#?}", b);
//     println!("{}---", b);

//     b.flag(&(0, 0));
//     println!("{}---", b);

//     b.uncover(&(1, 0));
//     println!("{}---", b);

//     println!("won? {}", b.has_won());
//     println!("ended? {}", b.game_ended);
// }

struct State {
    pub board: Board,
}

static STATE: Lazy<AtomicRefCell<State>> = Lazy::new(|| {
    AtomicRefCell::new(State {
        board: Board::new((W, H), NUM_MINES),
    })
});

simple_game!("tictactoe", setup, update);

fn setup(c: &mut EngineContext) {
    // c.load_fonts_from_bytes(&[(
    //     "uni",
    //     include_bytes!("../assets/univers-light-normal.ttf"),
    // )]);

    c.load_texture_from_bytes("1", include_bytes!("../sprites/1.png"));
    c.load_texture_from_bytes("2", include_bytes!("../sprites/2.png"));
    c.load_texture_from_bytes("3", include_bytes!("../sprites/3.png"));
    c.load_texture_from_bytes("4", include_bytes!("../sprites/4.png"));
    c.load_texture_from_bytes("5", include_bytes!("../sprites/5.png"));
    c.load_texture_from_bytes("6", include_bytes!("../sprites/6.png"));
    c.load_texture_from_bytes("7", include_bytes!("../sprites/7.png"));
    c.load_texture_from_bytes("8", include_bytes!("../sprites/8.png"));

    c.load_texture_from_bytes("empty", include_bytes!("../sprites/empty.png"));
    c.load_texture_from_bytes("exploded", include_bytes!("../sprites/exploded.png"));
    c.load_texture_from_bytes("flag", include_bytes!("../sprites/flag.png"));
    c.load_texture_from_bytes("mine", include_bytes!("../sprites/mine.png"));
    c.load_texture_from_bytes("unknown", include_bytes!("../sprites/unknown.png"));

    let mut cam = main_camera_mut();
    cam.zoom = 500.0;
    //cam.zoom /= 4.0;
}

fn draw_cell(cell: &Cell, pos: &(u8, u8)) {
    let x = pos.0 as f32;
    let y = pos.1 as f32;
    let vec = Vec2::new((x - W as f32 * 0.5) * SPRITE_W, (y - H as f32 * 0.5) * SPRITE_W);
    let t = if cell.is_uncovered {
        if cell.is_mine {
            if cell.has_exploded {
                "exploded"
            } else {
                "mine"
            }
        } else if cell.hint_number == 1 {
            "1"
        } else if cell.hint_number == 2 {
            "2"
        } else if cell.hint_number == 3 {
            "3"
        } else if cell.hint_number == 4 {
            "4"
        } else if cell.hint_number == 5 {
            "5"
        } else if cell.hint_number == 6 {
            "6"
        } else if cell.hint_number == 7 {
            "7"
        } else if cell.hint_number == 8 {
            "8"
        } else {
            "empty"
        }
    } else {
        if cell.is_flagged {
            "flag"
        } else {
            "unknown"
        }
    };
    draw_sprite(texture_id(t), vec, WHITE, 0, splat(SPRITE_W));
}

fn update(_c: &mut EngineContext) {
    if is_key_down(KeyCode::Escape) {
        exit(0); // TODO
    }
    
    let mut state = STATE.borrow_mut();

    let is_left_down: bool = is_mouse_button_pressed(MouseButton::Left);
    let is_right_down: bool = is_mouse_button_pressed(MouseButton::Right);
    let is_down = is_left_down || is_right_down;
    if is_down {
        let pos = mouse_world();
        let x: i32 = (pos.x / SPRITE_W).floor() as i32 + W as i32 /2;
        let y: i32 = (pos.y / SPRITE_W).floor() as i32 + H as i32 /2;
        if x < 0 || x >= state.board.size.0 as i32 || y < 0 || y >= state.board.size.1 as i32 {
            return;
        }
        println!("pos {:?}", pos);
        println!("mouse {} {}", x, y);
        
        let pos2 = (x as u8, y as u8);
        
        if is_left_down {
            state.board.uncover(&pos2);
        } else if is_right_down {
            state.board.flag(&pos2);
        }
        println!("{}", state.board);
    }

    for y in 0..state.board.size.1 {
        for x in 0..state.board.size.0 {
            let pos = (x, y);
            let cell = state.board.get_cell(&pos).unwrap();
            draw_cell(cell, &pos);
        }
    }
}
