use comfy::*;
use comfy_core::{Mesh, SpriteVertex};

simple_game!("draw_mesh", State, config, setup, update);

struct State {}

impl State {
    pub fn new(_c: &EngineState) -> Self {
        Self {}
    }
}

fn config(config: GameConfig) -> GameConfig {
    GameConfig {
        ..config
    }
}

fn setup(_state: &mut State, _c: &mut EngineContext) {
}

fn update(_state: &mut State, _c: &mut EngineContext) {
    let yellow = Color { r: 1.0, g: 1.0, b: 0.0, a: 1.0 };
    let red = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    
    #[rustfmt::skip]
    let verts = vec![
        SpriteVertex::new(vec3( 0.0,  2.0, 0.0), vec2(0.0, 0.0), yellow),
        SpriteVertex::new(vec3( 5.0, -2.0, 0.0), vec2(0.0, 1.0), red),
        SpriteVertex::new(vec3(-5.0, -2.0, 0.0), vec2(1.0, 0.0), red),
    ];
    let indices = vec![0, 1, 2];
    
    // https://docs.rs/comfy-core/latest/comfy_core/struct.Mesh.html
    let mesh = Mesh {
      //origin: Vec2::new(0.0, 0,0),
      vertices: SmallVec::from_slice(&verts),
      indices: SmallVec::from_slice(&indices),
      z_index: 0,
      texture: None
    };
    draw_mesh(mesh);
}
