extern crate piston_window;
extern crate graphics;
extern crate rand;

use piston_window::*;
use graphics::color::*;
use rand::Rng;
use std::path::Path;

struct Point {
    x: f64,
    y: f64,
}

impl Point {

    fn new(x: f64, y: f64) -> Point {
        Point {
            x: x,
            y: y
        }
    }

}

struct Bubble {

    location: Point,
    radius: f64,
    speed: f64,
    direction: f64

}

impl Bubble {

    fn new(x: f64, y: f64, radius: f64, speed: f64, direction: f64) -> Bubble {
        Bubble {
            location: Point::new(x, y),
            radius: radius,
            speed: speed,
            direction: direction
        }
    }

    fn rise(&mut self) -> &mut Bubble {
        self.location.y -= self.speed;
        self.location.x += self.direction;
        self
    }

}

struct Level {

    width: f64,
    height: f64,
    bubbles: Vec<Bubble>

}

impl Level {

    fn new(width: f64, height: f64) -> Level {
        println!("h={}, w={}", height, width);
        Level {
            width: width,
            height: height,
            bubbles: Vec::new()
        }
    }

    fn update(&mut self) -> &mut Level {
        for i in 0..self.bubbles.len() {
            self.bubbles[i].rise();
        }
        let h = self.height;
        let w = self.width;
        self.bubbles.retain(|b| b.location.y > 0.0 && (b.location.y + b.radius) < h);
        self.bubbles.retain(|b| b.location.x > 0.0 && (b.location.x + b.radius) < w);
        self
    }

    fn add_bubble(&mut self) -> &mut Level {
        let radius = rand::thread_rng().gen_range(50.0, 200.0);
        let center = (self.width / 2.0) - radius / 2.0;
        let x = rand::thread_rng().gen_range(center - 100.0, center + 100.0);
        let speed = rand::thread_rng().gen_range(0.2, 1.0);
        let direction = rand::thread_rng().gen_range(-1.0, 1.0);
        self.bubbles.push(Bubble::new(x, self.height - radius, radius, speed, direction));
        self
    }

}

fn main() {
    let window: PistonWindow = WindowSettings::new("Bubbles", [640, 480])
        .exit_on_esc(true)
        .fullscreen(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to start Bubbles: {}", e) });

    let windown_size: Size = window.size();
    let mut level = Level::new(windown_size.width as f64, windown_size.height as f64);

    let background_color = hex("34b5ec");
    let bubble_texture = Texture::from_path(&mut *window.factory.borrow_mut(), Path::new("bubble.png"), Flip::None, &TextureSettings::new()).unwrap();

    for e in window {
        match e.event {
            Some(Event::Render(_)) => {
                e.draw_2d(|_c, g| {
                    clear(background_color, g);
                    let draw_state = default_draw_state();
                    for bubble in &level.bubbles {
                        let shape = rectangle::square(bubble.location.x, bubble.location.y, bubble.radius);
                        Image::new().rect(shape).draw(&bubble_texture, draw_state, _c.transform, g);
                    }
                });
            },
            Some(Event::Update(_)) => {
                level.update();
            },
            Some(Event::Input(input)) => {
                match input {
                    Input::Release(key) => {
                        match key {
                            Button::Keyboard(Key::Space) => {
                                 level.add_bubble();
                            },
                            Button::Keyboard(Key::Return) => {
                                 level.add_bubble();
                            },
                            _ => {}
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }
}
