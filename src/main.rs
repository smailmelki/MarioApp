use macroquad::prelude::*;

const SCREEN_W: f32 = 960.0;
const SCREEN_H: f32 = 540.0;
const WORLD_W: f32 = 3200.0;
const GROUND_Y: f32 = 450.0;
const GRAVITY: f32 = 1450.0;
const PLAYER_SPEED: f32 = 285.0;
const JUMP_SPEED: f32 = 610.0;

#[derive(Clone, Copy)]
struct Rect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl Rect {
    fn overlaps(self, other: Rect) -> bool {
        self.x < other.x + other.w
            && self.x + self.w > other.x
            && self.y < other.y + other.h
            && self.y + self.h > other.y
    }
}

struct Player {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    w: f32,
    h: f32,
    grounded: bool,
    facing: f32,
    lives: i32,
    invulnerable: f32,
}

impl Player {
    fn new() -> Self {
        Self {
            x: 80.0, y: GROUND_Y - 48.0, vx: 0.0, vy: 0.0,
            w: 34.0, h: 48.0, grounded: false, facing: 1.0,
            lives: 3, invulnerable: 0.0,
        }
    }

    fn bounds(&self) -> Rect {
        Rect { x: self.x, y: self.y, w: self.w, h: self.h }
    }
}

struct Coin {
    x: f32,
    y: f32,
    collected: bool,
    phase: f32,
}

struct Enemy {
    x: f32,
    y: f32,
    start_x: f32,
    end_x: f32,
    dir: f32,
    alive: bool,
}

#[derive(PartialEq)]
enum GameState {
    Playing,
    Won,
    Lost,
}

struct Game {
    player: Player,
    platforms: Vec<Rect>,
    coins: Vec<Coin>,
    enemies: Vec<Enemy>,
    score: u32,
    camera_x: f32,
    state: GameState,
}

impl Game {
    fn new() -> Self {
        let platforms = vec![
            Rect { x: 0.0, y: GROUND_Y, w: 650.0, h: 120.0 },
            Rect { x: 760.0, y: GROUND_Y, w: 520.0, h: 120.0 },
            Rect { x: 1390.0, y: GROUND_Y, w: 560.0, h: 120.0 },
            Rect { x: 2070.0, y: GROUND_Y, w: 430.0, h: 120.0 },
            Rect { x: 2620.0, y: GROUND_Y, w: 580.0, h: 120.0 },
            Rect { x: 250.0, y: 350.0, w: 130.0, h: 22.0 },
            Rect { x: 480.0, y: 285.0, w: 120.0, h: 22.0 },
            Rect { x: 850.0, y: 350.0, w: 140.0, h: 22.0 },
            Rect { x: 1080.0, y: 285.0, w: 130.0, h: 22.0 },
            Rect { x: 1510.0, y: 350.0, w: 150.0, h: 22.0 },
            Rect { x: 1760.0, y: 290.0, w: 130.0, h: 22.0 },
            Rect { x: 2180.0, y: 350.0, w: 140.0, h: 22.0 },
            Rect { x: 2390.0, y: 285.0, w: 130.0, h: 22.0 },
            Rect { x: 2740.0, y: 350.0, w: 150.0, h: 22.0 },
        ];

        let coin_positions = [
            (285.0, 310.0), (330.0, 310.0), (515.0, 245.0), (565.0, 245.0),
            (900.0, 310.0), (1130.0, 245.0), (1180.0, 245.0), (1550.0, 310.0),
            (1800.0, 250.0), (1845.0, 250.0), (2220.0, 310.0), (2440.0, 245.0),
            (2790.0, 310.0), (2840.0, 310.0), (2920.0, 400.0),
        ];
        let coins = coin_positions.iter().map(|&(x, y)| Coin {
            x, y, collected: false, phase: x * 0.03,
        }).collect();

        let enemies = vec![
            Enemy { x: 410.0, y: GROUND_Y - 30.0, start_x: 390.0, end_x: 610.0, dir: 1.0, alive: true },
            Enemy { x: 930.0, y: GROUND_Y - 30.0, start_x: 800.0, end_x: 1200.0, dir: -1.0, alive: true },
            Enemy { x: 1600.0, y: GROUND_Y - 30.0, start_x: 1450.0, end_x: 1880.0, dir: 1.0, alive: true },
            Enemy { x: 2200.0, y: GROUND_Y - 30.0, start_x: 2110.0, end_x: 2440.0, dir: -1.0, alive: true },
            Enemy { x: 2780.0, y: GROUND_Y - 30.0, start_x: 2650.0, end_x: 3090.0, dir: 1.0, alive: true },
        ];

        Self {
            player: Player::new(), platforms, coins, enemies,
            score: 0, camera_x: 0.0, state: GameState::Playing,
        }
    }

    fn restart(&mut self) {
        *self = Game::new();
    }

    fn update(&mut self, dt: f32) {
        if self.state != GameState::Playing {
            if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::R) {
                self.restart();
            }
            return;
        }

        let left = is_key_down(KeyCode::Left) || is_key_down(KeyCode::A)
            || (is_mouse_button_down(MouseButton::Left) && mouse_position().0 < 145.0);
        let right = is_key_down(KeyCode::Right) || is_key_down(KeyCode::D)
            || (is_mouse_button_down(MouseButton::Left) && mouse_position().0 > screen_width() - 145.0);
        let jump = is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Up)
            || is_key_pressed(KeyCode::W)
            || (is_mouse_button_pressed(MouseButton::Left)
                && mouse_position().0 > screen_width() - 145.0
                && mouse_position().1 < screen_height() - 105.0);

        self.player.vx = 0.0;
        if left { self.player.vx -= PLAYER_SPEED; self.player.facing = -1.0; }
        if right { self.player.vx += PLAYER_SPEED; self.player.facing = 1.0; }
        if jump && self.player.grounded {
            self.player.vy = -JUMP_SPEED;
            self.player.grounded = false;
        }

        self.player.vy += GRAVITY * dt;
        self.player.vy = self.player.vy.min(900.0);

        // الحركة الأفقية والتصادم مع الجدران.
        self.player.x += self.player.vx * dt;
        for p in &self.platforms {
            if self.player.bounds().overlaps(*p) {
                if self.player.vx > 0.0 { self.player.x = p.x - self.player.w; }
                if self.player.vx < 0.0 { self.player.x = p.x + p.w; }
            }
        }
        self.player.x = self.player.x.clamp(0.0, WORLD_W - self.player.w);

        // الحركة العمودية والهبوط فوق المنصات.
        let old_bottom = self.player.y + self.player.h;
        self.player.y += self.player.vy * dt;
        self.player.grounded = false;
        for p in &self.platforms {
            if self.player.bounds().overlaps(*p) {
                if self.player.vy >= 0.0 && old_bottom <= p.y + 12.0 {
                    self.player.y = p.y - self.player.h;
                    self.player.vy = 0.0;
                    self.player.grounded = true;
                } else if self.player.vy < 0.0 {
                    self.player.y = p.y + p.h;
                    self.player.vy = 0.0;
                }
            }
        }

        if self.player.invulnerable > 0.0 {
            self.player.invulnerable -= dt;
        }

        for coin in &mut self.coins {
            coin.phase += dt * 5.0;
            let coin_rect = Rect { x: coin.x - 11.0, y: coin.y - 13.0, w: 22.0, h: 26.0 };
            if !coin.collected && self.player.bounds().overlaps(coin_rect) {
                coin.collected = true;
                self.score += 10;
            }
        }

        for enemy in &mut self.enemies {
            if !enemy.alive { continue; }
            enemy.x += enemy.dir * 75.0 * dt;
            if enemy.x < enemy.start_x { enemy.x = enemy.start_x; enemy.dir = 1.0; }
            if enemy.x > enemy.end_x { enemy.x = enemy.end_x; enemy.dir = -1.0; }
            let enemy_rect = Rect { x: enemy.x, y: enemy.y, w: 34.0, h: 30.0 };
            if self.player.bounds().overlaps(enemy_rect) && self.player.invulnerable <= 0.0 {
                if self.player.vy > 80.0 && self.player.y + self.player.h - enemy.y < 23.0 {
                    enemy.alive = false;
                    self.player.vy = -JUMP_SPEED * 0.58;
                    self.score += 50;
                } else {
                    self.player.lives -= 1;
                    self.player.invulnerable = 1.6;
                    self.player.vy = -JUMP_SPEED * 0.55;
                    self.player.x -= self.player.facing * 35.0;
                    if self.player.lives <= 0 { self.state = GameState::Lost; }
                }
            }
        }

        if self.player.y > screen_height() + 180.0 {
            self.player.lives -= 1;
            if self.player.lives <= 0 { self.state = GameState::Lost; }
            else {
                self.player.x = (self.player.x - 100.0).max(30.0);
                self.player.y = 100.0;
                self.player.vy = 0.0;
                self.player.invulnerable = 1.6;
            }
        }

        if self.player.x > WORLD_W - 115.0 {
            self.state = GameState::Won;
        }

        let target = (self.player.x - screen_width() * 0.36).max(0.0);
        self.camera_x += (target - self.camera_x) * (dt * 5.0).min(1.0);
    }

    fn draw(&self) {
        clear_background(Color::from_rgba(117, 203, 248, 255));
        self.draw_background();

        let cam = self.camera_x;
        for p in &self.platforms {
            let x = p.x - cam;
            if x + p.w < -30.0 || x > screen_width() + 30.0 { continue; }
            draw_rectangle(x, p.y, p.w, p.h, Color::from_rgba(145, 88, 48, 255));
            draw_rectangle(x, p.y, p.w, 9.0, Color::from_rgba(71, 181, 65, 255));
            for bx in (0..(p.w / 48.0) as i32) {
                draw_line(x + bx as f32 * 48.0, p.y + 12.0, x + bx as f32 * 48.0, p.y + p.h, 1.5, Color::from_rgba(105, 60, 38, 255));
            }
        }

        for coin in &self.coins {
            if coin.collected { continue; }
            let x = coin.x - cam;
            if x < -30.0 || x > screen_width() + 30.0 { continue; }
            let width = 7.0 + coin.phase.sin().abs() * 7.0;
            draw_ellipse(x, coin.y, width, 13.0, GOLD);
            draw_ellipse_lines(x, coin.y, width, 13.0, 2.0, Color::from_rgba(190, 120, 0, 255));
            draw_circle(x - 2.0, coin.y - 4.0, 2.0, Color::from_rgba(255, 250, 180, 255));
        }

        for enemy in &self.enemies {
            if !enemy.alive { continue; }
            let x = enemy.x - cam;
            draw_ellipse(x + 17.0, enemy.y + 27.0, 19.0, 5.0, Color::from_rgba(50, 90, 60, 70));
            draw_rounded_rectangle(x, enemy.y + 5.0, 34.0, 24.0, 8.0, 8.0, Color::from_rgba(151, 76, 48, 255));
            draw_circle(x + 10.0, enemy.y + 15.0, 4.0, WHITE);
            draw_circle(x + 24.0, enemy.y + 15.0, 4.0, WHITE);
            draw_circle(x + 11.0, enemy.y + 16.0, 1.8, BLACK);
            draw_circle(x + 23.0, enemy.y + 16.0, 1.8, BLACK);
            draw_line(x + 7.0, enemy.y + 28.0, x + 27.0, enemy.y + 28.0, 2.0, BLACK);
        }

        self.draw_player();
        self.draw_goal();

        draw_rectangle(0.0, 0.0, screen_width(), 54.0, Color::from_rgba(20, 45, 75, 190));
        draw_text(&format!("SCORE  {}", self.score), 22.0, 35.0, 25.0, WHITE);
        draw_text(&format!("LIVES  {}", self.player.lives), 205.0, 35.0, 25.0, WHITE);
        draw_text("RUST PLATFORM ADVENTURE", screen_width() - 330.0, 34.0, 20.0, WHITE);

        self.draw_touch_controls();

        if self.state != GameState::Playing {
            draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::from_rgba(8, 20, 38, 190));
            let title = if self.state == GameState::Won { "LEVEL COMPLETE!" } else { "GAME OVER" };
            let title_w = measure_text(title, None, 54, 1.0).width;
            draw_text(title, (screen_width() - title_w) / 2.0, screen_height() / 2.0 - 20.0, 54.0, YELLOW);
            let msg = format!("Final score: {}   -   Press ENTER or R to play again", self.score);
            let msg_w = measure_text(&msg, None, 22, 1.0).width;
            draw_text(&msg, (screen_width() - msg_w) / 2.0, screen_height() / 2.0 + 30.0, 22.0, WHITE);
        }
    }

    fn draw_player(&self) {
        let p = &self.player;
        if p.invulnerable > 0.0 && (get_time() * 12.0) as i32 % 2 == 0 { return; }
        let x = p.x - self.camera_x;
        let y = p.y;
        // شخصية مغامر أصلية بأسلوب ألعاب المنصات.
        draw_ellipse(x + 17.0, y + 47.0, 19.0, 5.0, Color::from_rgba(40, 70, 80, 70));
        draw_rectangle(x + 4.0, y + 24.0, 26.0, 20.0, Color::from_rgba(36, 92, 190, 255));
        draw_rectangle(x + 3.0, y + 39.0, 12.0, 8.0, Color::from_rgba(75, 48, 35, 255));
        draw_rectangle(x + 19.0, y + 39.0, 12.0, 8.0, Color::from_rgba(75, 48, 35, 255));
        draw_rectangle(x + 2.0, y + 5.0, 30.0, 8.0, Color::from_rgba(214, 65, 48, 255));
        draw_rectangle(x + 7.0, y + 11.0, 22.0, 16.0, Color::from_rgba(244, 183, 125, 255));
        draw_rectangle(x + 5.0, y + 2.0, 25.0, 6.0, Color::from_rgba(183, 45, 38, 255));
        let eye_x = if p.facing > 0.0 { x + 24.0 } else { x + 10.0 };
        draw_circle(eye_x, y + 17.0, 2.3, BLACK);
        draw_rectangle(x + 10.0, y + 24.0, 14.0, 4.0, Color::from_rgba(245, 210, 165, 255));
    }

    fn draw_goal(&self) {
        let x = WORLD_W - 95.0 - self.camera_x;
        draw_rectangle(x, GROUND_Y - 112.0, 8.0, 112.0, Color::from_rgba(235, 235, 225, 255));
        draw_triangle(vec2(x + 8.0, GROUND_Y - 110.0), vec2(x + 62.0, GROUND_Y - 92.0),
            vec2(x + 8.0, GROUND_Y - 72.0), Color::from_rgba(230, 65, 65, 255));
        draw_rectangle(x - 12.0, GROUND_Y - 5.0, 32.0, 5.0, DARKBROWN);
    }

    fn draw_background(&self) {
        let w = screen_width();
        let h = screen_height();
        draw_circle(w - 130.0, 115.0, 42.0, Color::from_rgba(255, 231, 125, 255));
        for i in 0..9 {
            let base = i as f32 * 440.0 - (self.camera_x * 0.28) % 440.0;
            draw_ellipse(base + 100.0, GROUND_Y - 8.0, 145.0, 92.0, Color::from_rgba(82, 177, 111, 255));
            draw_ellipse(base + 270.0, GROUND_Y - 2.0, 180.0, 112.0, Color::from_rgba(64, 158, 100, 255));
        }
        for i in 0..7 {
            let x = (i as f32 * 230.0 - (self.camera_x * 0.55) % 230.0) + 30.0;
            draw_circle(x, 145.0 + (i % 3) as f32 * 38.0, 20.0, Color::from_rgba(255, 255, 255, 190));
            draw_circle(x + 22.0, 140.0 + (i % 3) as f32 * 38.0, 26.0, Color::from_rgba(255, 255, 255, 190));
            draw_circle(x + 47.0, 148.0 + (i % 3) as f32 * 38.0, 18.0, Color::from_rgba(255, 255, 255, 190));
        }
        let _ = h;
    }

    fn draw_touch_controls(&self) {
        let y = screen_height() - 72.0;
        for (x, label) in [(62.0, "<"), (137.0, ">"), (screen_width() - 75.0, "JUMP")] {
            let radius = if label == "JUMP" { 42.0 } else { 30.0 };
            draw_circle(x, y, radius, Color::from_rgba(25, 40, 65, 105));
            draw_circle_lines(x, y, radius, 2.0, Color::from_rgba(255, 255, 255, 150));
            let size = if label == "JUMP" { 17.0 } else { 34.0 };
            let tw = measure_text(label, None, size, 1.0).width;
            draw_text(label, x - tw / 2.0, y + size as f32 / 3.0, size as f32, WHITE);
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "MarioApp - Rust Platform Adventure".to_owned(),
        window_width: SCREEN_W as i32,
        window_height: SCREEN_H as i32,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();
    loop {
        let dt = get_frame_time().min(1.0 / 30.0);
        game.update(dt);
        game.draw();
        next_frame().await;
    }
}
