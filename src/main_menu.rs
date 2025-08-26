use minifb::{Key, Window};

#[derive(Debug, PartialEq)]
pub enum MenuState {
    Main,
    TreeSelection,
    Parameters,
    Settings,
    Help,
    Hidden,
}

#[derive(Debug, Clone)]
pub struct MainMenuItem {
    pub title: String,
    pub description: String,
    pub hotkey: Option<Key>,
}

pub struct MainMenu {
    pub state: MenuState,
    pub main_items: Vec<MainMenuItem>,
    pub selected_index: usize,
}

impl MainMenu {
    pub fn new() -> Self {
        let main_items = vec![
            MainMenuItem {
                title: "Tree Species".to_string(),
                description: "Browse and select different tree species (Tab)".to_string(),
                hotkey: Some(Key::Tab),
            },
            MainMenuItem {
                title: "Parameters".to_string(),
                description: "Adjust L-system parameters in real-time (G)".to_string(),
                hotkey: Some(Key::G),
            },
            MainMenuItem {
                title: "Edit L-system".to_string(),
                description: "Edit current L-system rules in vim (E)".to_string(),
                hotkey: Some(Key::E),
            },
            MainMenuItem {
                title: "Reload".to_string(),
                description: "Reload current L-system from disk (R)".to_string(),
                hotkey: Some(Key::R),
            },
            MainMenuItem {
                title: "Help".to_string(),
                description: "Show controls and usage information (H)".to_string(),
                hotkey: Some(Key::H),
            },
            MainMenuItem {
                title: "Exit".to_string(),
                description: "Exit the application (Escape)".to_string(),
                hotkey: Some(Key::Escape),
            },
        ];

        Self {
            state: MenuState::Hidden,
            main_items,
            selected_index: 0,
        }
    }
    
    pub fn toggle(&mut self) {
        self.state = match self.state {
            MenuState::Hidden => MenuState::Main,
            _ => MenuState::Hidden,
        };
    }
    
    pub fn show_main(&mut self) {
        self.state = MenuState::Main;
        self.selected_index = 0;
    }
    
    pub fn hide(&mut self) {
        self.state = MenuState::Hidden;
    }
    
    pub fn is_visible(&self) -> bool {
        self.state != MenuState::Hidden
    }
    
    pub fn handle_input(&mut self, window: &Window) -> Option<MenuAction> {
        if self.state == MenuState::Hidden {
            return None;
        }
        
        match self.state {
            MenuState::Main => self.handle_main_menu_input(window),
            MenuState::Help => self.handle_help_input(window),
            _ => None,
        }
    }
    
    fn handle_main_menu_input(&mut self, window: &Window) -> Option<MenuAction> {
        // Navigation
        if window.is_key_pressed(Key::Up, minifb::KeyRepeat::No) {
            if self.selected_index > 0 {
                self.selected_index -= 1;
            } else {
                self.selected_index = self.main_items.len() - 1;
            }
        }
        
        if window.is_key_pressed(Key::Down, minifb::KeyRepeat::No) {
            self.selected_index = (self.selected_index + 1) % self.main_items.len();
        }
        
        // Selection
        if window.is_key_pressed(Key::Enter, minifb::KeyRepeat::No) {
            return self.execute_selected_item();
        }
        
        // Direct hotkeys
        if window.is_key_pressed(Key::Tab, minifb::KeyRepeat::No) {
            return Some(MenuAction::ShowTreeSelection);
        }
        
        if window.is_key_pressed(Key::G, minifb::KeyRepeat::No) {
            return Some(MenuAction::ShowParameters);
        }
        
        if window.is_key_pressed(Key::E, minifb::KeyRepeat::No) {
            return Some(MenuAction::EditLSystem);
        }
        
        if window.is_key_pressed(Key::R, minifb::KeyRepeat::No) {
            return Some(MenuAction::ReloadLSystem);
        }
        
        if window.is_key_pressed(Key::H, minifb::KeyRepeat::No) {
            self.state = MenuState::Help;
            return None;
        }
        
        None
    }
    
    fn handle_help_input(&mut self, window: &Window) -> Option<MenuAction> {
        if window.is_key_pressed(Key::Escape, minifb::KeyRepeat::No) ||
           window.is_key_pressed(Key::H, minifb::KeyRepeat::No) ||
           window.is_key_pressed(Key::Enter, minifb::KeyRepeat::No) {
            self.state = MenuState::Main;
        }
        None
    }
    
    fn execute_selected_item(&mut self) -> Option<MenuAction> {
        match self.selected_index {
            0 => Some(MenuAction::ShowTreeSelection),
            1 => Some(MenuAction::ShowParameters),
            2 => Some(MenuAction::EditLSystem),
            3 => Some(MenuAction::ReloadLSystem),
            4 => {
                self.state = MenuState::Help;
                None
            },
            5 => Some(MenuAction::Exit),
            _ => None,
        }
    }
    
    pub fn render(&self, buffer: &mut [u32], width: usize, height: usize, current_tree_name: &str) {
        if self.state == MenuState::Hidden {
            return;
        }
        
        match self.state {
            MenuState::Main => self.render_main_menu(buffer, width, height, current_tree_name),
            MenuState::Help => self.render_help(buffer, width, height),
            _ => {},
        }
    }
    
    fn render_main_menu(&self, buffer: &mut [u32], width: usize, height: usize, current_tree_name: &str) {
        let menu_width = 500;
        let menu_height = 400;
        let menu_x = (width - menu_width) / 2;
        let menu_y = (height - menu_height) / 2;
        
        // Draw menu background with gradient
        self.fill_rect(buffer, width, height, menu_x, menu_y, menu_width, menu_height, 0x1a1a1a);
        self.draw_rect(buffer, width, height, menu_x, menu_y, menu_width, menu_height, 0x444444);
        
        // Draw title bar
        self.fill_rect(buffer, width, height, menu_x, menu_y, menu_width, 40, 0x2d2d2d);
        self.draw_text(buffer, width, height, menu_x + 20, menu_y + 15, "3D L-Systems Main Menu", 0xFFFFFF);
        
        // Draw current tree info
        let info_text = format!("Current: {}", current_tree_name);
        self.draw_text(buffer, width, height, menu_x + 20, menu_y + 50, &info_text, 0x888888);
        
        // Draw menu items
        let start_y = menu_y + 80;
        for (i, item) in self.main_items.iter().enumerate() {
            let y = start_y + i * 45;
            let color = if i == self.selected_index { 0x00FF00 } else { 0xCCCCCC };
            let bg_color = if i == self.selected_index { 0x333333 } else { 0x1a1a1a };
            
            // Highlight selected item
            if i == self.selected_index {
                self.fill_rect(buffer, width, height, menu_x + 10, y - 5, menu_width - 20, 35, bg_color);
            }
            
            // Draw hotkey indicator
            if let Some(key) = item.hotkey {
                let key_text = self.key_to_string(key);
                self.draw_text(buffer, width, height, menu_x + 20, y, &format!("[{}]", key_text), 0x666666);
            }
            
            // Draw title and description
            self.draw_text(buffer, width, height, menu_x + 60, y, &item.title, color);
            self.draw_text(buffer, width, height, menu_x + 60, y + 15, &item.description, 0x888888);
        }
        
        // Draw footer
        let footer_y = menu_y + menu_height - 30;
        self.draw_text(buffer, width, height, menu_x + 20, footer_y, 
                      "Arrow Keys: Navigate | Enter: Select | M: Toggle Menu | Escape: Close", 0x666666);
    }
    
    fn render_help(&self, buffer: &mut [u32], width: usize, height: usize) {
        let menu_width = 600;
        let menu_height = 500;
        let menu_x = (width - menu_width) / 2;
        let menu_y = (height - menu_height) / 2;
        
        // Draw help background
        self.fill_rect(buffer, width, height, menu_x, menu_y, menu_width, menu_height, 0x1a1a1a);
        self.draw_rect(buffer, width, height, menu_x, menu_y, menu_width, menu_height, 0x444444);
        
        // Draw title
        self.fill_rect(buffer, width, height, menu_x, menu_y, menu_width, 40, 0x2d2d2d);
        self.draw_text(buffer, width, height, menu_x + 20, menu_y + 15, "Controls & Help", 0xFFFFFF);
        
        let help_text = vec![
            "Camera Controls:",
            "  Mouse + Drag: Rotate camera around tree",
            "  Mouse Wheel: Zoom in/out",
            "",
            "Tree Selection:",
            "  1-9: Load specific tree species",
            "  Tab: Open tree species menu",
            "",
            "Editing & Parameters:",
            "  G: Toggle parameter sliders (real-time editing)",
            "  E: Edit L-system rules in vim",
            "  R: Reload current L-system from disk",
            "",
            "Interface:",
            "  M: Toggle this main menu",
            "  H: Toggle help screen",
            "  Escape: Exit application",
            "",
            "Tree Species (1-9):",
            "  1=Sierpinski, 2=Plant, 3=Oak, 4=Pine, 5=Cherry",
            "  6=Maple, 7=Willow, 8=Baobab, 9=Eucalyptus",
            "",
            "Press H, Enter, or Escape to return to main menu",
        ];
        
        let mut y = menu_y + 60;
        for line in help_text {
            let color = if line.is_empty() { 
                0x000000 
            } else if line.ends_with(':') { 
                0xFFFFFF 
            } else if line.starts_with("  ") { 
                0x888888 
            } else { 
                0xCCCCCC 
            };
            
            if !line.is_empty() {
                self.draw_text(buffer, width, height, menu_x + 20, y, line, color);
            }
            y += 18;
        }
    }
    
    fn key_to_string(&self, key: Key) -> &'static str {
        match key {
            Key::Tab => "Tab",
            Key::G => "G",
            Key::E => "E", 
            Key::R => "R",
            Key::H => "H",
            Key::Escape => "Esc",
            _ => "?",
        }
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
        // Use same text rendering as menu.rs for consistency
        let char_width = 8;
        let char_height = 12;
        
        for (i, _c) in text.chars().enumerate() {
            let char_x = x + i * char_width;
            
            // Draw a simple rectangle for each character
            for dy in 0..char_height {
                for dx in 0..char_width {
                    let px = char_x + dx;
                    let py = y + dy;
                    
                    if px < buf_width && py < buf_height {
                        // Simple pattern to make text visible
                        if (dy == 0 || dy == char_height - 1 || dx == 0 || dx == char_width - 1) && 
                           dy >= 2 && dy < char_height - 2 {
                            buffer[py * buf_width + px] = color;
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MenuAction {
    ShowTreeSelection,
    ShowParameters,
    EditLSystem,
    ReloadLSystem,
    Exit,
}