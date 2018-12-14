extern crate ggez;
use ggez::graphics::{DrawMode, Point2};
use ggez::*;

mod map;

struct MainState {
    pos_x: f32,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState>{
        let state = MainState{ pos_x: 0.0_f32 };
        Ok(state)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::circle(ctx,
                         DrawMode::Fill,
                         Point2::new(self.pos_x, 380.0),
                         100.0,
                         2.0)?;
        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    map::load::load_map("./map_example.map").expect("Could not open map");

    let context_builder = ContextBuilder::new("pacman_clone", "ggez")
        .window_setup(conf::WindowSetup::default().title("Pacman Clone!"))
        .window_mode(conf::WindowMode::default().dimensions(224, 288));

    let ctx = &mut context_builder.build().unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}