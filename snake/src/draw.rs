/*
 * @Author: bill
 * @Date: 2021-06-18 22:38:10
 * @LastEditors: bill
 * @LastEditTime: 2021-06-21 09:42:58
 * @Description:
 * @FilePath: /rust-exercise/snake/src/draw.rs
 */

use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};
const BLOCK_SIZE: f64 = 25.0;

/// 游戏坐标转换为块坐标
pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

/// 按照给定的游戏坐标绘画正方形块
pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    );
}

/// 绘画正方形画布
pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    weight: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(
        color,
        [x, y, BLOCK_SIZE * (weight as f64), (height as f64)],
        con.transform,
        g,
    )
}
