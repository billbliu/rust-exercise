/*
 * @Author: bill
 * @Date: 2021-06-19 22:25:38
 * @LastEditors: bill
 * @LastEditTime: 2021-06-21 11:11:21
 * @Description:
 * @FilePath: /rust-exercise/snake/src/game.rs
 */
use piston_window::types::Color;
use piston_window::*;

use rand::{thread_rng, Rng};

use crate::draw::{draw_block, draw_rectangle};
use crate::snake::{Direction, Snake};

/// 食物块的颜色，红色
const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
/// 边界颜色，黑色
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
/// 游戏结束颜色，半透明红色
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

/// 移动周期
const MOVING_PERIOD: f64 = 0.1;
/// 重新开始时间
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    // 贪吃蛇
    snake: Snake,

    // 食物是否cunz
    food_exists: bool,
    // 食物x坐标
    food_x: i32,
    // 食物y坐标
    food_y: i32,
    // 游戏窗口的宽度
    width: i32,
    // 游戏窗口的高度
    height: i32,

    // 游戏是否结束
    game_over: bool,
    waiting_time: f64,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            food_exists: true,
            food_x: 6,
            food_y: 4,
            width,
            height,
            waiting_time: 0.0,
            game_over: false,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        };

        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }

        self.update_snake(dir);
    }

    // 画每个时刻的图
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        // 画贪吃蛇
        self.snake.draw(con, g);

        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }
        // 画边框线
        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);

        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !self.food_exists {
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }

    /// 检查蛇是否存活
    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        // 下一帧，蛇头的位置
        let (next_x, next_y) = self.snake.next_head(dir);

        // 蛇头是否与蛇身重叠
        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }

        // 蛇头是否超过边界
        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    /// 随机添加一个食物
    fn add_food(&mut self) {
        let mut rng = thread_rng();

        let mut new_x = rng.gen_range(1..self.width - 1);
        let mut new_y = rng.gen_range(1..self.height - 1);
        // 食物不要添加到蛇身的位置
        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rng.gen_range(1..self.width - 1);
            new_y = rng.gen_range(1..self.height - 1);
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_x = 6;
        self.food_y = 4;
        self.game_over = false;
    }
}
