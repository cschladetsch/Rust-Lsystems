use glam::{Mat3, Vec3};
use crate::renderer::{Renderer, Vertex, Line};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TurtleState {
    pub position: Vec3,
    pub direction: Vec3,
    pub up: Vec3,
    pub color: Vec3,
    pub line_width: f32,
}

impl TurtleState {
    pub fn new() -> Self {
        Self {
            position: Vec3::ZERO,
            direction: Vec3::Y, // Start pointing up
            up: Vec3::Z,        // Up is towards viewer
            color: Vec3::new(0.0, 1.0, 0.0), // Green
            line_width: 1.0,
        }
    }
}

pub struct Turtle3D {
    current_state: TurtleState,
    state_stack: Vec<TurtleState>,
    step_length: f32,
    angle: f32,
    color_palette: Vec<Vec3>,
    current_color_index: usize,
    depth_colors: bool,
}

impl Turtle3D {
    pub fn new() -> Self {
        Self {
            current_state: TurtleState::new(),
            state_stack: Vec::new(),
            step_length: 1.0,
            angle: 25.0_f32.to_radians(),
            color_palette: Self::create_color_palette(),
            current_color_index: 0,
            depth_colors: true,
        }
    }
    
    fn create_color_palette() -> Vec<Vec3> {
        vec![
            Vec3::new(0.0, 1.0, 0.0),   // Green
            Vec3::new(0.8, 0.4, 0.0),   // Brown
            Vec3::new(1.0, 0.0, 0.0),   // Red
            Vec3::new(0.0, 0.0, 1.0),   // Blue
            Vec3::new(1.0, 1.0, 0.0),   // Yellow
            Vec3::new(1.0, 0.0, 1.0),   // Magenta
            Vec3::new(0.0, 1.0, 1.0),   // Cyan
            Vec3::new(1.0, 0.5, 0.0),   // Orange
            Vec3::new(0.5, 0.0, 0.5),   // Purple
            Vec3::new(0.5, 1.0, 0.5),   // Light green
        ]
    }
    
    pub fn set_step_length(&mut self, length: f32) {
        self.step_length = length;
    }
    
    pub fn set_angle(&mut self, angle_degrees: f32) {
        self.angle = angle_degrees.to_radians();
    }
    
    pub fn reset(&mut self) {
        self.current_state = TurtleState::new();
        self.state_stack.clear();
        self.current_color_index = 0;
    }
    
    pub fn interpret(&mut self, commands: &str, renderer: &mut Renderer, custom_rules: Option<&HashMap<char, String>>) {
        for c in commands.chars() {
            match c {
                'F' | 'G' => self.forward(renderer, true),
                'f' | 'g' => self.forward(renderer, false),
                '+' => self.turn_left(),
                '-' => self.turn_right(),
                '&' => self.pitch_down(),
                '^' => self.pitch_up(),
                '\\' => self.roll_left(),
                '/' => self.roll_right(),
                '|' => self.turn_around(),
                '[' => self.push_state(),
                ']' => self.pop_state(),
                '#' => self.increment_color(),
                '!' => self.decrement_line_width(),
                '\'' => self.increment_line_width(),
                _ => {
                    if let Some(rules) = custom_rules {
                        if rules.contains_key(&c) {
                            // Custom rule - could be handled recursively if needed
                            continue;
                        }
                    }
                }
            }
        }
    }
    
    fn forward(&mut self, renderer: &mut Renderer, draw: bool) {
        let new_position = self.current_state.position + self.current_state.direction * self.step_length;
        
        if draw {
            let color = if self.depth_colors {
                self.get_depth_color(self.current_state.position.y)
            } else {
                self.current_state.color
            };
            
            let start = Vertex::new(self.current_state.position, color);
            let end = Vertex::new(new_position, color);
            
            renderer.add_line(Line::new(start, end));
        }
        
        self.current_state.position = new_position;
    }
    
    fn get_depth_color(&self, y: f32) -> Vec3 {
        let depth_factor = (y + 10.0) / 20.0; // Normalize to 0-1 range
        let depth_factor = depth_factor.clamp(0.0, 1.0);
        
        // Interpolate between brown (bottom) and green (top)
        let brown = Vec3::new(0.4, 0.2, 0.0);
        let green = Vec3::new(0.0, 0.8, 0.2);
        
        brown + depth_factor * (green - brown)
    }
    
    fn turn_left(&mut self) {
        let right = self.current_state.direction.cross(self.current_state.up);
        let rotation = Mat3::from_axis_angle(self.current_state.up, self.angle);
        self.current_state.direction = rotation * self.current_state.direction;
    }
    
    fn turn_right(&mut self) {
        let right = self.current_state.direction.cross(self.current_state.up);
        let rotation = Mat3::from_axis_angle(self.current_state.up, -self.angle);
        self.current_state.direction = rotation * self.current_state.direction;
    }
    
    fn pitch_down(&mut self) {
        let right = self.current_state.direction.cross(self.current_state.up);
        let rotation = Mat3::from_axis_angle(right, -self.angle);
        self.current_state.direction = rotation * self.current_state.direction;
        self.current_state.up = rotation * self.current_state.up;
    }
    
    fn pitch_up(&mut self) {
        let right = self.current_state.direction.cross(self.current_state.up);
        let rotation = Mat3::from_axis_angle(right, self.angle);
        self.current_state.direction = rotation * self.current_state.direction;
        self.current_state.up = rotation * self.current_state.up;
    }
    
    fn roll_left(&mut self) {
        let rotation = Mat3::from_axis_angle(self.current_state.direction, self.angle);
        self.current_state.up = rotation * self.current_state.up;
    }
    
    fn roll_right(&mut self) {
        let rotation = Mat3::from_axis_angle(self.current_state.direction, -self.angle);
        self.current_state.up = rotation * self.current_state.up;
    }
    
    fn turn_around(&mut self) {
        self.current_state.direction = -self.current_state.direction;
    }
    
    fn push_state(&mut self) {
        self.state_stack.push(self.current_state.clone());
    }
    
    fn pop_state(&mut self) {
        if let Some(state) = self.state_stack.pop() {
            self.current_state = state;
        }
    }
    
    fn increment_color(&mut self) {
        self.current_color_index = (self.current_color_index + 1) % self.color_palette.len();
        self.current_state.color = self.color_palette[self.current_color_index];
    }
    
    fn increment_line_width(&mut self) {
        self.current_state.line_width = (self.current_state.line_width * 1.2).min(5.0);
    }
    
    fn decrement_line_width(&mut self) {
        self.current_state.line_width = (self.current_state.line_width * 0.8).max(0.1);
    }
    
    pub fn set_depth_colors(&mut self, enabled: bool) {
        self.depth_colors = enabled;
    }
}