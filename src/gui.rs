use minifb::{Key, Window};

#[derive(Debug, Clone)]
pub struct Slider {
    pub name: String,
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub step: f32,
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Slider {
    pub fn new(name: &str, value: f32, min: f32, max: f32, x: usize, y: usize) -> Self {
        Self {
            name: name.to_string(),
            value,
            min,
            max,
            step: (max - min) / 100.0,
            x,
            y,
            width: 200,
            height: 20,
        }
    }
    
    pub fn update(&mut self, window: &Window, mouse_x: f32, mouse_y: f32, mouse_pressed: bool) -> bool {
        if mouse_pressed &&
           mouse_x >= self.x as f32 && mouse_x <= (self.x + self.width) as f32 &&
           mouse_y >= self.y as f32 && mouse_y <= (self.y + self.height) as f32 {
            
            let relative_x = (mouse_x - self.x as f32) / self.width as f32;
            let relative_x = relative_x.clamp(0.0, 1.0);
            let new_value = self.min + relative_x * (self.max - self.min);
            
            if (new_value - self.value).abs() > self.step * 0.1 {
                self.value = new_value;
                return true; // Value changed
            }
        }
        false
    }
    
    pub fn render(&self, buffer: &mut [u32], width: usize, height: usize) {
        // Draw slider background
        self.fill_rect(buffer, width, height, self.x, self.y, self.width, self.height, 0x404040);
        
        // Draw slider track
        let track_y = self.y + self.height / 2 - 2;
        self.fill_rect(buffer, width, height, self.x + 5, track_y, self.width - 10, 4, 0x606060);
        
        // Draw slider handle
        let handle_pos = ((self.value - self.min) / (self.max - self.min) * (self.width - 20) as f32) as usize;
        let handle_x = self.x + 10 + handle_pos;
        let handle_y = self.y + 2;
        self.fill_rect(buffer, width, height, handle_x - 5, handle_y, 10, self.height - 4, 0x00FF00);
        
        // Draw label
        self.draw_text(buffer, width, height, self.x, self.y - 15, 
                      &format!("{}: {:.2}", self.name, self.value), 0xFFFFFF);
    }
    
    fn fill_rect(&self, buffer: &mut [u32], buf_width: usize, buf_height: usize, 
                x: usize, y: usize, w: usize, h: usize, color: u32) {
        for dy in 0..h {
            for dx in 0..w {
                let px = x + dx;
                let py = y + dy;
                if px < buf_width && py < buf_height {
                    buffer[py * buf_width + px] = color;
                }
            }
        }
    }
    
    fn draw_text(&self, buffer: &mut [u32], buf_width: usize, buf_height: usize,
                x: usize, y: usize, text: &str, color: u32) {
        // Simple bitmap font rendering
        let char_width = 6;
        let char_height = 8;
        
        for (i, _c) in text.chars().enumerate() {
            let char_x = x + i * char_width;
            
            // Draw a simple rectangle pattern for each character
            for dy in 0..char_height {
                for dx in 0..char_width {
                    let px = char_x + dx;
                    let py = y + dy;
                    
                    if px < buf_width && py < buf_height {
                        // Simple pattern to make text visible
                        if (dy == 1 || dy == char_height - 2) && dx > 0 && dx < char_width - 1 {
                            buffer[py * buf_width + px] = color;
                        }
                        if (dx == 1 || dx == char_width - 2) && dy > 1 && dy < char_height - 2 {
                            buffer[py * buf_width + px] = color;
                        }
                    }
                }
            }
        }
    }
}

pub struct GUI {
    pub sliders: Vec<Slider>,
    pub visible: bool,
    pub mouse_pressed: bool,
    pub last_mouse_pos: (f32, f32),
}

impl GUI {
    pub fn new() -> Self {
        let mut sliders = Vec::new();
        
        // Create parameter sliders
        sliders.push(Slider::new("Angle", 25.0, 5.0, 90.0, 20, 50));
        sliders.push(Slider::new("Step Length", 1.0, 0.1, 3.0, 20, 100));
        sliders.push(Slider::new("Trunk Width", 5.0, 1.0, 20.0, 20, 150));
        sliders.push(Slider::new("Branch Taper", 0.8, 0.3, 1.0, 20, 200));
        
        Self {
            sliders,
            visible: false,
            mouse_pressed: false,
            last_mouse_pos: (0.0, 0.0),
        }
    }
    
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }
    
    pub fn handle_input(&mut self, window: &Window) -> bool {
        if !self.visible {
            return false;
        }
        
        let mut changed = false;
        
        // Handle mouse input
        if let Some(mouse_pos) = window.get_mouse_pos(minifb::MouseMode::Clamp) {
            let mouse_pressed = window.get_mouse_down(minifb::MouseButton::Left);
            
            for slider in &mut self.sliders {
                if slider.update(window, mouse_pos.0, mouse_pos.1, mouse_pressed) {
                    changed = true;
                }
            }
            
            self.last_mouse_pos = mouse_pos;
            self.mouse_pressed = mouse_pressed;
        }
        
        changed
    }
    
    pub fn render(&self, buffer: &mut [u32], width: usize, height: usize) {
        if !self.visible {
            return;
        }
        
        // Draw GUI background panel
        self.fill_rect(buffer, width, height, 10, 10, 250, 300, 0x202020);
        self.draw_rect(buffer, width, height, 10, 10, 250, 300, 0x606060);
        
        // Draw title
        self.draw_text(buffer, width, height, 20, 25, "L-System Parameters", 0xFFFFFF);
        
        // Render all sliders
        for slider in &self.sliders {
            slider.render(buffer, width, height);
        }
        
        // Draw instructions
        self.draw_text(buffer, width, height, 20, 280, "G: Toggle GUI | Click sliders to adjust", 0xCCCCCC);
    }
    
    pub fn get_parameter(&self, name: &str) -> Option<f32> {
        self.sliders.iter()
            .find(|s| s.name == name)
            .map(|s| s.value)
    }
    
    fn fill_rect(&self, buffer: &mut [u32], buf_width: usize, buf_height: usize, 
                x: usize, y: usize, w: usize, h: usize, color: u32) {
        for dy in 0..h {
            for dx in 0..w {
                let px = x + dx;
                let py = y + dy;
                if px < buf_width && py < buf_height {
                    buffer[py * buf_width + px] = color;
                }
            }
        }
    }
    
    fn draw_rect(&self, buffer: &mut [u32], buf_width: usize, buf_height: usize,
                x: usize, y: usize, w: usize, h: usize, color: u32) {
        // Top and bottom borders
        for dx in 0..w {
            let px = x + dx;
            if px < buf_width {
                if y < buf_height {
                    buffer[y * buf_width + px] = color;
                }
                if y + h - 1 < buf_height {
                    buffer[(y + h - 1) * buf_width + px] = color;
                }
            }
        }
        
        // Left and right borders
        for dy in 0..h {
            let py = y + dy;
            if py < buf_height {
                if x < buf_width {
                    buffer[py * buf_width + x] = color;
                }
                if x + w - 1 < buf_width {
                    buffer[py * buf_width + (x + w - 1)] = color;
                }
            }
        }
    }
    
    fn draw_text(&self, buffer: &mut [u32], buf_width: usize, buf_height: usize,
                x: usize, y: usize, text: &str, color: u32) {
        let char_width = 6;
        let char_height = 8;
        
        for (i, _c) in text.chars().enumerate() {
            let char_x = x + i * char_width;
            
            for dy in 0..char_height {
                for dx in 0..char_width {
                    let px = char_x + dx;
                    let py = y + dy;
                    
                    if px < buf_width && py < buf_height {
                        if (dy == 1 || dy == char_height - 2) && dx > 0 && dx < char_width - 1 {
                            buffer[py * buf_width + px] = color;
                        }
                        if (dx == 1 || dx == char_width - 2) && dy > 1 && dy < char_height - 2 {
                            buffer[py * buf_width + px] = color;
                        }
                    }
                }
            }
        }
    }
}