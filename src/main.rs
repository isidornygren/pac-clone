extern crate ggez;
use ggez::graphics::{DrawMode, Point2, Rect};
use ggez::*;

mod map;
use map::map::Map;

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

struct MainState {
    pos_x: f32,
    level: Map<char>,
    wall_sprite: graphics::Image,
    floor_sprite: graphics::Image,
}

impl MainState {
    fn new(ctx: &mut Context, level: Map<char>) -> GameResult<MainState> {
        let state = MainState {
            pos_x: 0.0_f32,
            level: level,
            wall_sprite: graphics::Image::new(ctx, "/wall_sprite.png")?,
            floor_sprite: graphics::Image::new(ctx, "/wall_sprite.png")?,
        };
        Ok(state)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        for y in 0..self.level.height {
            for x in 0..self.level.width {
                let tile = self.level.tile_at(x, y);
                let marching_value = self.level.marching_square_at(x, y, '#');
                println!("marching value: {}", marching_value);
                if *tile == '#' {
                    let dst = Point2::new((x as f32) * TILE_WIDTH, (y as f32) * TILE_HEIGHT);
                    graphics::draw(ctx, &self.wall_sprite, dst, 0.0)?;
                }
            }
        }

        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    let new_map = map::load::load_map("./map_example.map").expect("Could not open map");

    let width = (new_map.width as u32) * (TILE_WIDTH as u32);
    let height = (new_map.height as u32) * (TILE_HEIGHT as u32);

    let context_builder = ContextBuilder::new("pacman_clone", "ggez")
        .window_setup(conf::WindowSetup::default().title("Pacman Clone!"))
        .window_mode(conf::WindowMode::default().dimensions(width, height));

    let ctx = &mut context_builder.build().unwrap();
    let state = &mut MainState::new(ctx, new_map).unwrap();
    event::run(ctx, state).unwrap();
}
