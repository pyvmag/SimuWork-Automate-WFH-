use enigo::{Enigo, Key, Keyboard, Mouse, Settings, Direction, Axis, Button}; // Added Axis and Button
use std::{thread, time::Duration};
use kurbo::{CubicBez, ParamCurve, Point};
use rand::Rng;

pub trait InputController {}

pub struct MouseSimulator {
    enigo: Enigo,
}

impl MouseSimulator {
    pub fn new() -> Self {
        Self {
            enigo: Enigo::new(&Settings::default()).unwrap(),
        }
    }

    pub fn move_to(&mut self, end_x: i32, end_y: i32, duration_ms: u64) {
        let (start_x, start_y) = self.enigo.location().unwrap_or((0, 0));
        let mut rng = rand::thread_rng();
        let start = Point::new(start_x as f64, start_y as f64);
        let end = Point::new(end_x as f64, end_y as f64);
        let mid_x = (start_x + end_x) / 2;
        let mid_y = (start_y + end_y) / 2;
        let x_variance = (start_x - end_x).abs() / 2;
        let y_variance = (start_y - end_y).abs() / 2;
        let control1 = Point::new(
            rng.gen_range((mid_x - x_variance)..=(mid_x + x_variance)) as f64,
            rng.gen_range((mid_y - y_variance)..=(mid_y + y_variance)) as f64,
        );
        let control2 = Point::new(
            rng.gen_range((mid_x - x_variance)..=(mid_x + x_variance)) as f64,
            rng.gen_range((mid_y - y_variance)..=(mid_y + y_variance)) as f64,
        );
        let curve = CubicBez::new(start, control1, control2, end);
        let steps = 50;
        let sleep_duration = Duration::from_millis(duration_ms / steps);
        for i in 1..=steps {
            let t = i as f64 / steps as f64;
            let point = curve.eval(t);
            self.enigo.move_mouse(point.x as i32, point.y as i32, enigo::Coordinate::Abs).unwrap();
            thread::sleep(sleep_duration);
        }
    }

    pub fn click(&mut self) {
        println!("Simulating mouse click.");
        self.enigo.button(Button::Left, Direction::Click).unwrap();
    }

    // --- NEW FUNCTION ---
    pub fn scroll_wheel(&mut self, direction: &str) {
        let (axis, length) = match direction {
            "down" => (Axis::Vertical, 5), // Scroll 5 "units" down
            "up" => (Axis::Vertical, -5), // Scroll 5 "units" up
            _ => (Axis::Vertical, 0),
        };
        println!("Simulating mouse scroll {}.", direction);
        self.enigo.scroll(length, axis).unwrap();
    }
}

impl InputController for MouseSimulator {}

pub struct KeyboardSimulator {
    enigo: Enigo,
}

impl KeyboardSimulator {
    pub fn new() -> Self {
        Self {
            enigo: Enigo::new(&Settings::default()).unwrap(),
        }
    }

    pub fn type_string(&mut self, text: &str) {
        println!("Simulating typing: {}", text);
        self.enigo.text(text).unwrap();
    }

    pub fn switch_application(&mut self) {
        println!("Simulating application switch...");
        #[cfg(target_os = "macos")]
        {
            self.enigo.key(Key::Meta, Direction::Press).unwrap();
            self.enigo.key(Key::Tab, Direction::Click).unwrap();
            self.enigo.key(Key::Meta, Direction::Release).unwrap();
        }
        #[cfg(not(target_os = "macos"))]
        {
            self.enigo.key(Key::Alt, Direction::Press).unwrap();
            self.enigo.key(Key::Tab, Direction::Click).unwrap();
            self.enigo.key(Key::Alt, Direction::Release).unwrap();
        }
    }
}

impl InputController for KeyboardSimulator {}