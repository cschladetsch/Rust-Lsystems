use std::fs;
use std::path::PathBuf;
use minifb::{Key, Window};

#[derive(Debug, Clone)]
pub struct MenuItem {
    pub name: String,
    pub file_path: PathBuf,
    pub hotkey: Option<Key>,
}

pub struct Menu {
    pub items: Vec<MenuItem>,
    pub selected_index: usize,
    pub visible: bool,
    pub rules_directory: PathBuf,
}

impl Menu {
    pub fn new() -> Self {
        let rules_dir = PathBuf::from("rules");
        let mut menu = Self {
            items: Vec::new(),
            selected_index: 0,
            visible: false,
            rules_directory: rules_dir,
        };
        menu.load_items();
        menu
    }
    
    pub fn load_items(&mut self) {
        self.items.clear();
        
        // Add default systems with hotkeys
        let default_systems = vec![
            ("Sierpinski Triangle", "rules/sierpinski.json", Some(Key::Key1)),
            ("3D Plant", "rules/plant.json", Some(Key::Key2)),
            ("Oak Tree", "rules/oak_tree.json", Some(Key::Key3)),
            ("Pine Tree", "rules/pine_tree.json", Some(Key::Key4)),
            ("Cherry Blossom", "rules/cherry_blossom.json", Some(Key::Key5)),
            ("Autumn Maple", "rules/autumn_maple.json", Some(Key::Key6)),
            ("Weeping Willow", "rules/willow_tree.json", Some(Key::Key7)),
            ("Baobab Tree", "rules/baobab_tree.json", Some(Key::Key8)),
            ("Spiral Eucalyptus", "rules/spiral_eucalyptus.json", Some(Key::Key9)),
        ];
        
        for (name, path, key) in default_systems {
            let path_buf = PathBuf::from(path);
            if path_buf.exists() {
                self.items.push(MenuItem {
                    name: name.to_string(),
                    file_path: path_buf,
                    hotkey: key,
                });
            }
        }
        
        // Load additional JSON files from rules directory
        if let Ok(entries) = fs::read_dir(&self.rules_directory) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    if extension == "json" {
                        let file_name = path.file_stem()
                            .and_then(|s| s.to_str())
                            .unwrap_or("Unknown");
                        
                        // Skip if already added as default
                        if !self.items.iter().any(|item| item.file_path == path) {
                            self.items.push(MenuItem {
                                name: file_name.replace('_', " ").to_string(),
                                file_path: path,
                                hotkey: None,
                            });
                        }
                    }
                }
            }
        }
    }
    
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }
    
    pub fn handle_input(&mut self, window: &Window) -> Option<PathBuf> {
        if !self.visible {
            // Handle hotkeys even when menu is not visible
            for item in &self.items {
                if let Some(key) = item.hotkey {
                    if window.is_key_pressed(key, minifb::KeyRepeat::No) {
                        return Some(item.file_path.clone());
                    }
                }
            }
            return None;
        }
        
        // Navigation when menu is visible
        if window.is_key_pressed(Key::Up, minifb::KeyRepeat::No) {
            if self.selected_index > 0 {
                self.selected_index -= 1;
            } else {
                self.selected_index = self.items.len().saturating_sub(1);
            }
        }
        
        if window.is_key_pressed(Key::Down, minifb::KeyRepeat::No) {
            self.selected_index = (self.selected_index + 1) % self.items.len().max(1);
        }
        
        if window.is_key_pressed(Key::Enter, minifb::KeyRepeat::No) {
            if let Some(item) = self.items.get(self.selected_index) {
                self.visible = false;
                return Some(item.file_path.clone());
            }
        }
        
        None
    }
    
    pub fn render_to_buffer(&self, buffer: &mut [u32], width: usize, height: usize) {
        if !self.visible || self.items.is_empty() {
            return;
        }
        
        let menu_width = 300;
        let menu_height = self.items.len() * 30 + 40;
        let menu_x = (width - menu_width) / 2;
        let menu_y = (height - menu_height) / 2;
        
        // Draw menu background
        self.fill_rect(buffer, width, height, 
                      menu_x, menu_y, menu_width, menu_height, 0x404040);
        
        // Draw border
        self.draw_rect(buffer, width, height, 
                      menu_x, menu_y, menu_width, menu_height, 0xFFFFFF);
        
        // Draw title
        self.draw_text(buffer, width, height, 
                      menu_x + 10, menu_y + 10, "L-System Menu", 0xFFFFFF);
        
        // Draw menu items
        for (i, item) in self.items.iter().enumerate() {
            let y = menu_y + 40 + i * 30;
            let color = if i == self.selected_index { 0x00FF00 } else { 0xCCCCCC };
            
            let text = if let Some(key) = item.hotkey {
                format!("{} ({})", item.name, self.key_to_string(key))
            } else {
                item.name.clone()
            };
            
            self.draw_text(buffer, width, height, menu_x + 10, y, &text, color);
        }
        
        // Draw instructions
        let instructions = "Arrow keys: Navigate | Enter: Select | Tab: Toggle Menu | E: Edit";
        self.draw_text(buffer, width, height, 
                      menu_x + 10, menu_y + menu_height - 20, instructions, 0x888888);
    }
    
    fn key_to_string(&self, key: Key) -> &'static str {
        match key {
            Key::Key1 => "1",
            Key::Key2 => "2", 
            Key::Key3 => "3",
            Key::Key4 => "4",
            Key::Key5 => "5",
            Key::Key6 => "6",
            Key::Key7 => "7",
            Key::Key8 => "8",
            Key::Key9 => "9",
            Key::Key0 => "0",
            _ => "?",
        }
    }
    
    fn fill_rect(&self, buffer: &mut [u32], width: usize, height: usize, 
                x: usize, y: usize, w: usize, h: usize, color: u32) {
        for dy in 0..h {
            for dx in 0..w {
                let px = x + dx;
                let py = y + dy;
                if px < width && py < height {
                    buffer[py * width + px] = color;
                }
            }
        }
    }
    
    fn draw_rect(&self, buffer: &mut [u32], width: usize, height: usize,
                x: usize, y: usize, w: usize, h: usize, color: u32) {
        // Top and bottom borders
        for dx in 0..w {
            let px = x + dx;
            if px < width {
                if y < height {
                    buffer[y * width + px] = color;
                }
                if y + h - 1 < height {
                    buffer[(y + h - 1) * width + px] = color;
                }
            }
        }
        
        // Left and right borders
        for dy in 0..h {
            let py = y + dy;
            if py < height {
                if x < width {
                    buffer[py * width + x] = color;
                }
                if x + w - 1 < width {
                    buffer[py * width + (x + w - 1)] = color;
                }
            }
        }
    }
    
    fn draw_text(&self, buffer: &mut [u32], width: usize, height: usize,
                x: usize, y: usize, text: &str, color: u32) {
        // Simple bitmap font rendering - just draw colored pixels
        // This is a basic implementation, could be improved with actual font rendering
        let char_width = 8;
        let char_height = 12;
        
        for (i, _c) in text.chars().enumerate() {
            let char_x = x + i * char_width;
            
            // Draw a simple rectangle for each character
            for dy in 0..char_height {
                for dx in 0..char_width {
                    let px = char_x + dx;
                    let py = y + dy;
                    
                    if px < width && py < height {
                        // Simple pattern to make text visible
                        if (dy == 0 || dy == char_height - 1 || dx == 0 || dx == char_width - 1) && 
                           dy >= 2 && dy < char_height - 2 {
                            buffer[py * width + px] = color;
                        }
                    }
                }
            }
        }
    }
    
    pub fn get_selected_file(&self) -> Option<PathBuf> {
        self.items.get(self.selected_index).map(|item| item.file_path.clone())
    }
}