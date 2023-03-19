use nannou::prelude::*;
use std::f32::consts::PI;

const BALL_COUNT: usize = 5;
const BALL_SPEED: f32 = 3.0;

struct Ball {
    position: Point2,
    velocity: Vec2,
    color: Rgb,
    color_change_speed: Rgb,
    color_change_direction: Rgb,
}

impl Ball {
    fn new(
        position: Point2,
        velocity: Vec2,
        color: Rgb,
        color_change_speed: Rgb,
        color_change_direction: Rgb,
    ) -> Self {
        Ball {
            position,
            velocity,
            color,
            color_change_speed,
            color_change_direction,
        }
    }

    fn update(&mut self, win_rect: &Rect) {
        self.position += self.velocity;

        if self.position.x < win_rect.left() || self.position.x > win_rect.right() {
            self.velocity.x = -self.velocity.x;
        }

        if self.position.y < win_rect.bottom() || self.position.y > win_rect.top() {
            self.velocity.y = -self.velocity.y;
        }
        // Update the color
        self.color.red += self.color_change_speed.red * self.color_change_direction.red;
        self.color.green += self.color_change_speed.green * self.color_change_direction.green;
        self.color.blue += self.color_change_speed.blue * self.color_change_direction.blue;

        // Reverse color change direction if the color component reaches the minimum or maximum value
        self.color_change_direction.red *= if self.color.red <= 0.0 || self.color.red >= 1.0 {
            -1.0
        } else {
            1.0
        };
        self.color_change_direction.green *= if self.color.green <= 0.0 || self.color.green >= 1.0 {
            -1.0
        } else {
            1.0
        };
        self.color_change_direction.blue *= if self.color.blue <= 0.0 || self.color.blue >= 1.0 {
            -1.0
        } else {
            1.0
        };
    }
}

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    balls: Vec<Ball>,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(800, 600)
        .title("Bouncing Balls")
        .view(view)
        .build()
        .unwrap();

    let balls = (0..BALL_COUNT)
        .map(|_| {
            let position = random_range2(-400.0, 400.0, -300.0, 300.0);
            let angle = random_range(0.0, 2.0 * PI);
            let velocity = Vec2::new(angle.cos() * BALL_SPEED, angle.sin() * BALL_SPEED);

            // Generate a random color for each ball
            let color = rgb(random_f32(), random_f32(), random_f32());
            let color_change_speed = rgb(0.005, 0.005, 0.005);
            let color_change_direction = rgb(
                if random::<bool>() { 1.0 } else { -1.0 },
                if random::<bool>() { 1.0 } else { -1.0 },
                if random::<bool>() { 1.0 } else { -1.0 },
            );

            Ball::new(
                position,
                velocity,
                color,
                color_change_speed,
                color_change_direction,
            )
        })
        .collect();

    Model { balls }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let win_rect = _app.window_rect();
    for ball in &mut model.balls {
        ball.update(&win_rect);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    for ball in &model.balls {
        // Draw the glowing effect
        draw.ellipse()
            .x_y(ball.position.x, ball.position.y)
            .radius(30.0) // Increase the radius to create a glow around the ball
            .color(rgba(ball.color.red, ball.color.green, ball.color.blue, 0.2));

        // Draw the ball
        draw.ellipse()
            .x_y(ball.position.x, ball.position.y)
            .radius(20.0)
            .color(ball.color);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn random_range2(min_x: f32, max_x: f32, min_y: f32, max_y: f32) -> Point2 {
    let x = random_range(min_x, max_x);
    let y = random_range(min_y, max_y);
    pt2(x, y)
}

