extern crate rand;
extern crate piston_window;

mod draw;
mod snake;
mod game;

use piston_window::*;
use piston_window::types::Color;

use game::Game;
use draw::to_coord;

const BACK_COLOR: Color = [0.50, 0.50, 0.50, 1.00];

fn main() {
    let (width, height) = (30, 30);

    let mut window: PistonWindow = WindowSettings::new(
        "Rusty Snake",
        [
            to_coord(width),
            to_coord(height)
        ],
    ).exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new(width, height);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
