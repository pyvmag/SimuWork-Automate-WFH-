use crate::input_simulator::{KeyboardSimulator, MouseSimulator};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum ActivityLevel {
    Low,
    Medium,
    High,
}

pub struct Scheduler { is_running: Arc<Mutex<bool>> }
impl Scheduler {
    pub fn new() -> Self { Self { is_running: Arc::new(Mutex::new(false)) } }
    pub fn start(
        &self,
        mouse_simulator: Arc<Mutex<MouseSimulator>>,
        keyboard_simulator: Arc<Mutex<KeyboardSimulator>>,
        target_coords: Arc<Mutex<(i32, i32)>>,
        mouse_speed: Arc<Mutex<u64>>, // Now needs mouse speed
    ) {
        let running_clone = self.is_running.clone();
        *running_clone.lock().unwrap() = true;
        thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let sample_texts = vec!["Reviewing the document.", "Checking the latest updates."];
            while *running_clone.lock().unwrap() {
                let action = rng.gen_range(0..10);
                match action {
                    0..=5 => { // 60% chance to move the mouse
                        println!("Scheduler Tick: Moving mouse...");
                        let x = rng.gen_range(100..=800);
                        let y = rng.gen_range(100..=600);

                        // Calculate duration based on speed. Higher speed = less time.
                        let speed = *mouse_speed.lock().unwrap();
                        // Simple formula: max duration 2000ms, min 200ms
                        let duration = 2200 - (speed * 20); 

                        mouse_simulator.lock().unwrap().move_to(x, y, duration);
                    }
                    6..=9 => { // 40% chance to type at the target location
                        println!("Scheduler Tick: Finding a place to type...");
                        let coords = *target_coords.lock().unwrap();
                        let speed = *mouse_speed.lock().unwrap();
                        let duration = 2200 - (speed * 20);
                        mouse_simulator.lock().unwrap().move_to(coords.0, coords.1, duration);
                        thread::sleep(Duration::from_millis(100));
                        mouse_simulator.lock().unwrap().click();
                        thread::sleep(Duration::from_millis(100));
                        println!("Scheduler Tick: Typing...");
                        let text_index = rng.gen_range(0..sample_texts.len());
                        keyboard_simulator.lock().unwrap().type_string(sample_texts[text_index]);
                    }
                    _ => {}
                }
                let sleep_time = rng.gen_range(3..=8);
                println!("Waiting for {} seconds...", sleep_time);
                thread::sleep(Duration::from_secs(sleep_time));
            }
            println!("Scheduler loop has stopped.");
        });
    }
    pub fn stop(&self) {
        let mut is_running = self.is_running.lock().unwrap();
        *is_running = false;
        println!("Scheduler stop signal sent.");
    }
}

pub struct Engine {
    mouse: Arc<Mutex<MouseSimulator>>,
    keyboard: Arc<Mutex<KeyboardSimulator>>,
    scheduler: Scheduler,
    target_coords: Arc<Mutex<(i32, i32)>>,
    mouse_speed: Arc<Mutex<u64>>, // New field for mouse speed
}

impl Engine {
    pub fn new() -> Self {
        Self {
            mouse: Arc::new(Mutex::new(MouseSimulator::new())),
            keyboard: Arc::new(Mutex::new(KeyboardSimulator::new())),
            scheduler: Scheduler::new(),
            target_coords: Arc::new(Mutex::new((960, 540))),
            mouse_speed: Arc::new(Mutex::new(50)), // Default speed
        }
    }

    pub fn set_target_coords(&mut self, x: i32, y: i32) {
        *self.target_coords.lock().unwrap() = (x, y);
    }

    // New function to update mouse speed
    pub fn set_mouse_speed(&mut self, speed: u64) {
        *self.mouse_speed.lock().unwrap() = speed;
    }

    pub fn run(&mut self, level: ActivityLevel) {
        println!("Engine is running with activity level: {:?}", level);
        self.scheduler.start(self.mouse.clone(), self.keyboard.clone(), self.target_coords.clone(), self.mouse_speed.clone());
    }

    pub fn stop(&self) {
        self.scheduler.stop();
    }
}