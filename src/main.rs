use clap::{Arg, Command};
use minifb::{Key, Window, WindowOptions};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

#[derive(Serialize, Deserialize, Debug)]
struct LSystemRule {
    name: String,
    axiom: String,
    angle: f64,
    iterations: u32,
    rules: HashMap<String, String>,
    drawing_rules: HashMap<String, String>,
    step_size: f64,
    start_position: [f64; 2],
    start_angle: f64,
}

#[derive(Clone)]
struct TurtleState {
    x: f64,
    y: f64,
    angle: f64,
}

struct LSystem {
    rule: LSystemRule,
    current_string: String,
}

impl LSystem {
    fn new(rule: LSystemRule) -> Self {
        LSystem {
            current_string: rule.axiom.clone(),
            rule,
        }
    }

    fn iterate(&mut self) {
        let mut new_string = String::new();
        let mut chars = self.current_string.chars().peekable();
        
        while let Some(ch) = chars.next() {
            let ch_str = ch.to_string();
            if let Some(replacement) = self.rule.rules.get(&ch_str) {
                new_string.push_str(replacement);
            } else {
                new_string.push(ch);
            }
        }
        
        self.current_string = new_string;
    }

    fn generate(&mut self) {
        for _ in 0..self.rule.iterations {
            self.iterate();
        }
    }

    fn draw(&self, buffer: &mut [u32]) {
        let mut turtle = TurtleState {
            x: self.rule.start_position[0],
            y: self.rule.start_position[1],
            angle: self.rule.start_angle.to_radians(),
        };
        
        let mut state_stack: Vec<TurtleState> = Vec::new();
        
        for ch in self.current_string.chars() {
            let ch_str = ch.to_string();
            if let Some(action) = self.rule.drawing_rules.get(&ch_str) {
                match action.as_str() {
                    "forward" => {
                        let new_x = turtle.x + self.rule.step_size * turtle.angle.cos();
                        let new_y = turtle.y + self.rule.step_size * turtle.angle.sin();
                        
                        self.draw_line(buffer, turtle.x, turtle.y, new_x, new_y);
                        
                        turtle.x = new_x;
                        turtle.y = new_y;
                    }
                    "turn_right" => {
                        turtle.angle += self.rule.angle.to_radians();
                    }
                    "turn_left" => {
                        turtle.angle -= self.rule.angle.to_radians();
                    }
                    "push_state" => {
                        state_stack.push(turtle.clone());
                    }
                    "pop_state" => {
                        if let Some(state) = state_stack.pop() {
                            turtle = state;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn draw_line(&self, buffer: &mut [u32], x1: f64, y1: f64, x2: f64, y2: f64) {
        let x1 = x1 as i32;
        let y1 = y1 as i32;
        let x2 = x2 as i32;
        let y2 = y2 as i32;
        
        if x1 < 0 || x1 >= WIDTH as i32 || y1 < 0 || y1 >= HEIGHT as i32 ||
           x2 < 0 || x2 >= WIDTH as i32 || y2 < 0 || y2 >= HEIGHT as i32 {
            return;
        }
        
        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx - dy;
        
        let mut x = x1;
        let mut y = y1;
        
        loop {
            if x >= 0 && x < WIDTH as i32 && y >= 0 && y < HEIGHT as i32 {
                let index = y as usize * WIDTH + x as usize;
                if index < buffer.len() {
                    buffer[index] = 0x00FFFFFF;
                }
            }
            
            if x == x2 && y == y2 {
                break;
            }
            
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }
}

fn load_rule_from_file(path: &str) -> Result<LSystemRule, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let rule: LSystemRule = serde_json::from_str(&contents)?;
    Ok(rule)
}

fn main() {
    let matches = Command::new("RustL-System")
        .version("0.1.0")
        .author("Christian")
        .about("L-System generator with JSON rules")
        .arg(
            Arg::new("rule-file")
                .short('r')
                .long("rule")
                .value_name("FILE")
                .help("JSON file containing L-System rules")
                .default_value("rules/sierpinski.json"),
        )
        .get_matches();

    let rule_file = matches.get_one::<String>("rule-file").unwrap();
    
    let rule = match load_rule_from_file(rule_file) {
        Ok(rule) => rule,
        Err(e) => {
            eprintln!("Error loading rule file {}: {}", rule_file, e);
            std::process::exit(1);
        }
    };

    println!("Generating L-System: {}", rule.name);
    println!("Axiom: {}", rule.axiom);
    println!("Iterations: {}", rule.iterations);

    let mut lsystem = LSystem::new(rule);
    lsystem.generate();

    println!("Generated string length: {}", lsystem.current_string.len());

    let mut buffer: Vec<u32> = vec![0x00000000; WIDTH * HEIGHT];

    let mut window = Window::new(
        &format!("L-System: {}", lsystem.rule.name),
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(60);

    lsystem.draw(&mut buffer);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
