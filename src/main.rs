use ggez::event;
use ggez::graphics::{self, Color, DrawMode, Rect};
use ggez::input::keyboard::KeyCode;
use ggez::{Context, GameResult};

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const SQUARE_SIZE: f32 = 50.0;
const SQUARE_SPEED: f32 = 5.0;

struct MainState {
    square_x: f32,
    square_y: f32,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState {
            square_x: WINDOW_WIDTH / 2.0 - SQUARE_SIZE / 2.0,
            square_y: WINDOW_HEIGHT / 2.0 - SQUARE_SIZE / 2.0,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if ctx.keyboard.is_key_pressed(KeyCode::Left) && self.square_x > 0.0 {
            self.square_x -= SQUARE_SPEED;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Right) && self.square_x < WINDOW_WIDTH - SQUARE_SIZE {
            self.square_x += SQUARE_SPEED;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Up) && self.square_y > 0.0 {
            self.square_y -= SQUARE_SPEED;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Down) && self.square_y < WINDOW_HEIGHT - SQUARE_SIZE {
            self.square_y += SQUARE_SPEED;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);

        let square = Rect::new(self.square_x, self.square_y, SQUARE_SIZE, SQUARE_SIZE);
        let draw_mode = DrawMode::fill();
        let mesh = graphics::Mesh::new_rectangle(ctx, draw_mode, square, Color::WHITE)?;
        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("basic_game", "ggez")
        .window_setup(ggez::conf::WindowSetup::default().title("Basic Rust Game Window"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .backend(ggez::conf::Backend::OpenGL); // Explicitly choose OpenGL

    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}
