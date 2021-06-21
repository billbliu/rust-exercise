extern crate piston_window;
extern crate rand;
extern crate snake;

use piston_window::types::Color;
use piston_window::*;
use snake::draw::to_coord;
use snake::game::Game;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    // 宽40个块，高30个块
    let (width, height) = (40, 30);
    let mut window: PistonWindow =
        WindowSettings::new("snake", [to_coord(width), to_coord(height)])
            .exit_on_esc(true)
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
    println!("Hello, world!");
}
