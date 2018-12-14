extern crate ggez;
use ggez::graphics::{DrawMode, Point2, Rect};
use ggez::*;

const TILE_WIDTH: f32 = 32.0;
const TILE_HEIGHT: f32 = 32.0;

trait Tile {
    fn draw(_ctx: &mut Context, point: (f32, f32));
}

struct Floor;

impl Tile for Floor {
    fn draw(_ctx: &mut Context, (x, y): (f32, f32)) {
        graphics::set_color(
            _ctx,
            graphics::Color {
                r: 0.1,
                g: 0.1,
                b: 0.1,
                a: 1.0,
            },
        );

        graphics::rectangle(
            _ctx,
            DrawMode::Fill,
            Rect {
                x,
                y,
                w: TILE_WIDTH,
                h: TILE_HEIGHT,
            },
        );
    }
}

struct Wall;

impl Tile for Wall {
    fn draw(_ctx: &mut Context, (x, y): (f32, f32)) {
        graphics::set_color(
            _ctx,
            graphics::Color {
                r: 0.8,
                g: 0.8,
                b: 0.8,
                a: 1.0,
            },
        );
        graphics::rectangle(
            _ctx,
            DrawMode::Line(1.0),
            Rect {
                x,
                y,
                w: TILE_WIDTH,
                h: TILE_HEIGHT,
            },
        );
    }
}

struct MainState {
    pos_x: f32,
    // level: Vec<Box<dyn Tile>>,
}

// const level = vec!

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let state = MainState { pos_x: 0.0_f32 };
        Ok(state)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::circle(
            ctx,
            DrawMode::Fill,
            Point2::new(self.pos_x, 380.0),
            100.0,
            2.0,
        )?;

        Wall::draw(ctx, (32.0, 32.0));
        Wall::draw(ctx, (64.0, 64.0));
        Wall::draw(ctx, (128.0, 128.0));

        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    let context_builder = ContextBuilder::new("pacman_clone", "ggez")
        .window_setup(conf::WindowSetup::default().title("Pacman Clone!"))
        .window_mode(conf::WindowMode::default().dimensions(224, 288));

    let ctx = &mut context_builder.build().unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
