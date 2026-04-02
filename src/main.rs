#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use macroquad::prelude::*;

const SCREEN_WIDTH: f32 = 960.0;
const SCREEN_HEIGHT: f32 = 540.0;
const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT: f32 = 110.0;
const PADDLE_MARGIN: f32 = 40.0;
const PADDLE_SPEED: f32 = 460.0;
const BALL_SIZE: f32 = 18.0;
const BALL_SPEED: f32 = 360.0;
const BALL_SPEED_INCREMENT: f32 = 30.0;
const WINNING_SCORE: u32 = 10;
const CONTROLS_TEXT: &str = "W/S and Up/Down move paddles";
const RESTART_TEXT: &str = "First to 10 wins";
const PLAY_AGAIN_TEXT: &str = "Press Space to play again";
const LEFT_WINNER_TEXT: &str = "Left Player Wins";
const RIGHT_WINNER_TEXT: &str = "Right Player Wins";

#[derive(Clone, Copy)]
struct Paddle {
    rect: Rect,
}

impl Paddle {
    fn new(x: f32) -> Self {
        Self {
            rect: Rect::new(
                x,
                (SCREEN_HEIGHT - PADDLE_HEIGHT) * 0.5,
                PADDLE_WIDTH,
                PADDLE_HEIGHT,
            ),
        }
    }

    fn update(&mut self, up: KeyCode, down: KeyCode, delta: f32) {
        let mut direction = 0.0;

        if is_key_down(up) {
            direction -= 1.0;
        }
        if is_key_down(down) {
            direction += 1.0;
        }

        self.rect.y += direction * PADDLE_SPEED * delta;
        self.rect.y = self.rect.y.clamp(0.0, SCREEN_HEIGHT - self.rect.h);
    }

    fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, WHITE);
    }
}

struct Ball {
    rect: Rect,
    velocity: Vec2,
}

impl Ball {
    fn new() -> Self {
        let mut ball = Self {
            rect: Rect::new(
                (SCREEN_WIDTH - BALL_SIZE) * 0.5,
                (SCREEN_HEIGHT - BALL_SIZE) * 0.5,
                BALL_SIZE,
                BALL_SIZE,
            ),
            velocity: vec2(-BALL_SPEED, BALL_SPEED * 0.55),
        };
        ball.reset(false);
        ball
    }

    fn reset(&mut self, toward_right: bool) {
        self.rect.x = (SCREEN_WIDTH - BALL_SIZE) * 0.5;
        self.rect.y = (SCREEN_HEIGHT - BALL_SIZE) * 0.5;

        let horizontal = if toward_right {
            BALL_SPEED
        } else {
            -BALL_SPEED
        };
        let vertical = if rand::gen_range(0, 2) == 0 {
            BALL_SPEED * 0.55
        } else {
            -BALL_SPEED * 0.55
        };

        self.velocity = vec2(horizontal, vertical);
    }

    fn update(&mut self, delta: f32) {
        self.rect.x += self.velocity.x * delta;
        self.rect.y += self.velocity.y * delta;

        if self.rect.y <= 0.0 {
            self.rect.y = 0.0;
            self.velocity.y = self.velocity.y.abs();
        } else if self.rect.y + self.rect.h >= SCREEN_HEIGHT {
            self.rect.y = SCREEN_HEIGHT - self.rect.h;
            self.velocity.y = -self.velocity.y.abs();
        }
    }

    fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, WHITE);
    }
}

struct GameState {
    left_paddle: Paddle,
    right_paddle: Paddle,
    ball: Ball,
    left_score: u32,
    right_score: u32,
    winner: Option<&'static str>,
}

impl GameState {
    fn new() -> Self {
        Self {
            left_paddle: Paddle::new(PADDLE_MARGIN),
            right_paddle: Paddle::new(SCREEN_WIDTH - PADDLE_MARGIN - PADDLE_WIDTH),
            ball: Ball::new(),
            left_score: 0,
            right_score: 0,
            winner: None,
        }
    }

    fn restart(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        let delta = get_frame_time();

        if self.winner.is_some() {
            if is_key_pressed(KeyCode::Space) {
                self.restart();
            }
            return;
        }

        self.left_paddle.update(KeyCode::W, KeyCode::S, delta);
        self.right_paddle.update(KeyCode::Up, KeyCode::Down, delta);
        self.ball.update(delta);

        self.handle_paddle_collision(self.left_paddle.rect, true);
        self.handle_paddle_collision(self.right_paddle.rect, false);
        self.handle_scoring();
    }

    fn handle_paddle_collision(&mut self, paddle: Rect, is_left_paddle: bool) {
        if !self.ball.rect.overlaps(&paddle) {
            return;
        }

        if is_left_paddle {
            self.ball.rect.x = paddle.x + paddle.w;
        } else {
            self.ball.rect.x = paddle.x - self.ball.rect.w;
        }

        let paddle_center = paddle.y + paddle.h * 0.5;
        let ball_center = self.ball.rect.y + self.ball.rect.h * 0.5;
        let offset = (ball_center - paddle_center) / (paddle.h * 0.5);

        self.ball.velocity.x = if is_left_paddle {
            self.ball.velocity.x.abs() + BALL_SPEED_INCREMENT
        } else {
            -self.ball.velocity.x.abs() - BALL_SPEED_INCREMENT
        };
        self.ball.velocity.y = offset * BALL_SPEED * 1.35;
    }

    fn handle_scoring(&mut self) {
        if self.ball.rect.x + self.ball.rect.w < 0.0 {
            self.right_score += 1;
            self.finish_round();
        } else if self.ball.rect.x > SCREEN_WIDTH {
            self.left_score += 1;
            self.finish_round();
        }
    }

    fn finish_round(&mut self) {
        if self.left_score >= WINNING_SCORE {
            self.winner = Some(LEFT_WINNER_TEXT);
        } else if self.right_score >= WINNING_SCORE {
            self.winner = Some(RIGHT_WINNER_TEXT);
        } else {
            self.ball.reset(rand::gen_range(0, 2) == 0);
        }
    }

    fn draw(&self) {
        clear_background(Color::from_rgba(15, 18, 28, 255));
        self.draw_court();
        self.left_paddle.draw();
        self.right_paddle.draw();
        self.ball.draw();
        self.draw_score();
        self.draw_message();
    }

    fn draw_court(&self) {
        draw_line(
            SCREEN_WIDTH * 0.5,
            0.0,
            SCREEN_WIDTH * 0.5,
            SCREEN_HEIGHT,
            4.0,
            GRAY,
        );

        for index in 0..8 {
            draw_circle_lines(
                SCREEN_WIDTH * 0.5,
                SCREEN_HEIGHT * 0.5,
                18.0 + (index as f32 * 16.0),
                2.0,
                Color::from_rgba(55, 61, 82, 100),
            );
        }
    }

    fn draw_score(&self) {
        let left_score = self.left_score.to_string();
        let right_score = self.right_score.to_string();
        let left_dims = measure_text(&left_score, None, 64, 1.0);
        let right_dims = measure_text(&right_score, None, 64, 1.0);

        draw_text(
            &left_score,
            SCREEN_WIDTH * 0.25 - left_dims.width * 0.5,
            88.0,
            64.0,
            WHITE,
        );
        draw_text(
            &right_score,
            SCREEN_WIDTH * 0.75 - right_dims.width * 0.5,
            88.0,
            64.0,
            WHITE,
        );
    }

    fn draw_message(&self) {
        let controls_dims = measure_text(CONTROLS_TEXT, None, 28, 1.0);
        draw_text(
            CONTROLS_TEXT,
            SCREEN_WIDTH * 0.5 - controls_dims.width * 0.5,
            SCREEN_HEIGHT - 32.0,
            28.0,
            LIGHTGRAY,
        );

        if let Some(winner_text) = self.winner {
            let winner_dims = measure_text(winner_text, None, 56, 1.0);
            let play_again_dims = measure_text(PLAY_AGAIN_TEXT, None, 30, 1.0);

            draw_rectangle(
                0.0,
                0.0,
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                Color::from_rgba(0, 0, 0, 120),
            );
            draw_text(
                winner_text,
                SCREEN_WIDTH * 0.5 - winner_dims.width * 0.5,
                SCREEN_HEIGHT * 0.42,
                56.0,
                YELLOW,
            );
            draw_text(
                PLAY_AGAIN_TEXT,
                SCREEN_WIDTH * 0.5 - play_again_dims.width * 0.5,
                SCREEN_HEIGHT * 0.52,
                30.0,
                WHITE,
            );
        } else {
            let restart_dims = measure_text(RESTART_TEXT, None, 24, 1.0);
            draw_text(
                RESTART_TEXT,
                SCREEN_WIDTH * 0.5 - restart_dims.width * 0.5,
                32.0,
                24.0,
                LIGHTGRAY,
            );
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Rusty Pong".to_owned(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        high_dpi: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = GameState::new();

    loop {
        game.update();
        game.draw();
        next_frame().await;
    }
}
