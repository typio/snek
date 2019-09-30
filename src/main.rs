#![allow(unused)]

use std::collections::VecDeque;
use std::process;
use ggez::conf;
use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::graphics;
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};
use nalgebra as na;
use rand;

type Point2 = na::Point2<f32>;

const SCREEN_WIDTH: f32 = 950.0;
const SCREEN_HEIGHT: f32 = 530.0;

const PADDING: f32 = 25.0;

const GRID_WIDTH: u8 = 82;
const GRID_HEIGHT: u8 = 43;

struct MainState {
    score: u32,
    dir: Direction,
    snake_pos_x: VecDeque<f32>,
    snake_pos_y: VecDeque<f32>,
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState {
            score: 0,
            dir: Direction::Right,
            snake_pos_x: VecDeque::from(vec![2.0]),
            snake_pos_y: VecDeque::from(vec![2.0]),
        };

        Ok(s)
    }
}

fn draw_background(ctx: &mut Context) -> GameResult {
    let outside_rect = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::stroke(1.0),
        graphics::Rect::new(0.0, 0.0, 10.0, 10.0),
        graphics::Color::from_rgb_u32(0xB9C0A3)
    )?;

    let inside_rect = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect::new(0.0, 0.0, 6.0, 6.0),
        graphics::Color::from_rgb_u32(0xB9C0A3)
    )?;

    let square_size = 11.0;
    let mut i = PADDING;
    while i < (SCREEN_WIDTH - PADDING) {
        let mut j = PADDING;
        while j < (SCREEN_HEIGHT - PADDING) {
            graphics::draw(ctx, &outside_rect, (na::Point2::new(i, j),))?;
            graphics::draw(ctx, &inside_rect, (na::Point2::new(i + 2.0, j + 2.0),))?;
            j += square_size;
        }
        i += square_size;
    }

    Ok(())
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);

            match &self.dir {
                Direction::Up => {
                    self.snake_pos_y.push_back(*self.snake_pos_y
                        .get(0)
                        .unwrap_or_else(|| {
                            println!("Index out of bounds");
                            process::exit(1);
                        }) - 1.0);
                    self.snake_pos_x.push_back(*self.snake_pos_x
                        .get(0)
                        .unwrap_or_else(|| {
                            println!("Index out of bounds");
                            process::exit(1);
                        }));
                }
                Direction::Right => {
                    self.snake_pos_y.push_back(*self.snake_pos_y
                        .get(0)
                        .unwrap_or_else(|| {
                            println!("Index out of bounds");
                            process::exit(1);
                        }));
                    self.snake_pos_x.push_back(*self.snake_pos_x
                        .get(0)
                        .unwrap_or_else(|| {
                            println!("Index out of bounds");
                            process::exit(1);
                        }) + 1.0);
                }
                Direction::Down => {
                    self.snake_pos_y.push_back(*self.snake_pos_y
                        .get(0)
                        .unwrap_or_else(|| {
                            println!("Index out of bounds");
                            process::exit(1);
                        }) + 1.0);
                    self.snake_pos_x.push_back(*self.snake_pos_x
                        .get(0)
                        .unwrap_or_else(|| {
                            println!("Index out of bounds");
                            process::exit(1);
                        }));
                }
                Direction::Left => {
                    self.snake_pos_y.push_back(*self.snake_pos_y
                        .get(0)
                        .unwrap_or_else(|| {
                            println!("Index out of bounds");
                            process::exit(1);
                        }));
                    self.snake_pos_x.push_back(*self.snake_pos_x
                        .get(0)
                        .unwrap_or_else(|| {
                            println!("Index out of bounds");
                            process::exit(1);
                        }) - 1.0);
                }
                _ => (),
            }

        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::from_rgb_u32(0xCED6B5));

        draw_background(ctx);

        for n in 0..self.snake_pos_x.len() {
            let outside_rect = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::stroke(1.0),
                graphics::Rect::new(0.0, 0.0, 10.0, 10.0),
                graphics::Color::from_rgb_u32(0x000000)
            )?;

            let inside_rect = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(0.0, 0.0, 6.0, 6.0),
                graphics::Color::from_rgb_u32(0x000000)
            )?;

            graphics::draw(ctx, &outside_rect, (na::Point2::new(self.snake_pos_x.get(n).unwrap() * 11.0 + PADDING,
            self.snake_pos_y.get(n).unwrap() * 11.0 + PADDING
            ),))?;
            graphics::draw(ctx, &inside_rect, (na::Point2::new(self.snake_pos_x.get(n).unwrap() * 11.0 + PADDING + 2.0,
            self.snake_pos_y.get(n).unwrap() * 11.0 + PADDING + 2.0
            ),))?;
        }

        graphics::present(ctx)?;

        timer::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Up => {
                self.dir = Direction::Up;
            }
            KeyCode::Left => {
                self.dir = Direction::Left;
            }
            KeyCode::Right => {
                self.dir = Direction::Right;
            }
            KeyCode::Down => {
                self.dir = Direction::Down;
            }
            KeyCode::Escape => event::quit(ctx),
            _ => (),
        }
    }
}

fn main() -> GameResult {
    let cb = ContextBuilder::new("snek", "Thomas Huber")
        .window_setup(conf::WindowSetup::default().title("snek"))
        .window_mode(conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT));

    let (ctx, events_loop) = &mut cb.build()?;

    let game = &mut MainState::new(ctx)?;

    event::run(ctx, events_loop, game)
}
