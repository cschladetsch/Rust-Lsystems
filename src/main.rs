use clap::{Arg, Command};
use minifb::{Key, Window, WindowOptions};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use glam::Vec2;

mod camera;
mod renderer;
mod turtle3d;
mod menu;
mod editor;
mod gui;
mod main_menu;

use camera::Camera;
use renderer::Renderer;
use turtle3d::Turtle3D;
use menu::Menu;
use editor::Editor;
use gui::GUI;
use main_menu::{MainMenu, MenuAction};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LSystemRule {
    name: String,
    axiom: String,
    angle: f32,
    iterations: u32,
    rules: HashMap<char, String>,
    step_length: Option<f32>,
    start_position: Option<[f32; 3]>,
    start_direction: Option<[f32; 3]>,
    colors: Option<ColorConfig>,
    description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ColorConfig {
    depth_based: Option<bool>,
    palette: Option<Vec<[f32; 3]>>,
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
        
        for ch in self.current_string.chars() {
            if let Some(replacement) = self.rule.rules.get(&ch) {
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

    fn draw_3d(&self, turtle: &mut Turtle3D, renderer: &mut Renderer) {
        turtle.reset();
        
        if let Some(step_length) = self.rule.step_length {
            turtle.set_step_length(step_length);
        }
        
        turtle.set_angle(self.rule.angle);
        
        if let Some(colors) = &self.rule.colors {
            if let Some(depth_based) = colors.depth_based {
                turtle.set_depth_colors(depth_based);
            }
        }
        
        turtle.interpret(&self.current_string, renderer, Some(&self.rule.rules));
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
        .about("3D L-System generator with interactive menu and vim integration")
        .arg(
            Arg::new("rule-file")
                .short('r')
                .long("rule")
                .value_name("FILE")
                .help("JSON file containing L-System rules")
                .default_value("rules/cherry_blossom.json"),
        )
        .get_matches();

    let rule_file = matches.get_one::<String>("rule-file").unwrap();
    
    let mut current_rule = match load_rule_from_file(rule_file) {
        Ok(rule) => rule,
        Err(e) => {
            eprintln!("Error loading rule file {}: {}", rule_file, e);
            std::process::exit(1);
        }
    };

    println!("3D L-System Viewer Started");
    println!("Controls:");
    println!("  Mouse + Drag: Rotate camera");
    println!("  Mouse Wheel: Zoom in/out");
    println!("  M: Toggle main menu");
    println!("  Tab: Toggle tree selection menu");
    println!("  1-9: Load tree species (1=Sierpinski, 2=Plant, 3=Oak, 4=Pine, 5=Cherry, 6=Maple, 7=Willow, 8=Baobab, 9=Eucalyptus)");
    println!("  G: Toggle GUI parameter controls");
    println!("  E: Edit current L-system in vim");
    println!("  R: Reload current L-system");
    println!("  Escape: Exit");

    let mut window = Window::new(
        "3D L-System Viewer - Interactive",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(60);

    let mut camera = Camera::new(WIDTH as f32 / HEIGHT as f32);
    let mut renderer = Renderer::new(WIDTH, HEIGHT);
    let mut turtle = Turtle3D::new();
    let mut menu = Menu::new();
    let mut main_menu = MainMenu::new();
    let editor = Editor::new();
    let mut gui = GUI::new();
    
    let mut current_file_path = std::path::PathBuf::from(rule_file);
    let mut needs_regeneration = true;
    let mut lsystem = LSystem::new(current_rule.clone());
    
    let mut mouse_pressed = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Handle main menu input
        if window.is_key_pressed(Key::M, minifb::KeyRepeat::No) {
            main_menu.toggle();
        }
        
        // Handle main menu actions
        if let Some(action) = main_menu.handle_input(&window) {
            match action {
                MenuAction::ShowTreeSelection => {
                    main_menu.hide();
                    if !menu.visible {
                        menu.toggle();
                    }
                },
                MenuAction::ShowParameters => {
                    main_menu.hide();
                    if !gui.visible {
                        gui.toggle();
                    }
                },
                MenuAction::EditLSystem => {
                    main_menu.hide();
                    match editor.edit_file(Some(&current_file_path)) {
                        Ok(_) => {
                            println!("File edited, reloading...");
                            match load_rule_from_file(current_file_path.to_str().unwrap()) {
                                Ok(new_rule) => {
                                    current_rule = new_rule;
                                    lsystem = LSystem::new(current_rule.clone());
                                    needs_regeneration = true;
                                }
                                Err(e) => eprintln!("Error reloading file: {}", e),
                            }
                        }
                        Err(e) => eprintln!("Error editing file: {}", e),
                    }
                },
                MenuAction::ReloadLSystem => {
                    main_menu.hide();
                    match load_rule_from_file(current_file_path.to_str().unwrap()) {
                        Ok(new_rule) => {
                            current_rule = new_rule;
                            lsystem = LSystem::new(current_rule.clone());
                            needs_regeneration = true;
                            println!("L-system reloaded");
                        }
                        Err(e) => eprintln!("Error reloading file: {}", e),
                    }
                },
                MenuAction::Exit => {
                    break;
                }
            }
        }
        
        // Handle input
        if window.is_key_pressed(Key::Tab, minifb::KeyRepeat::No) {
            menu.toggle();
        }
        
        if window.is_key_pressed(Key::G, minifb::KeyRepeat::No) {
            gui.toggle();
        }
        
        if window.is_key_pressed(Key::E, minifb::KeyRepeat::No) && !menu.visible {
            match editor.edit_file(Some(&current_file_path)) {
                Ok(_) => {
                    println!("File edited, reloading...");
                    match load_rule_from_file(current_file_path.to_str().unwrap()) {
                        Ok(new_rule) => {
                            current_rule = new_rule;
                            lsystem = LSystem::new(current_rule.clone());
                            needs_regeneration = true;
                        }
                        Err(e) => eprintln!("Error reloading file: {}", e),
                    }
                }
                Err(e) => eprintln!("Error editing file: {}", e),
            }
        }
        
        if window.is_key_pressed(Key::R, minifb::KeyRepeat::No) && !menu.visible {
            match load_rule_from_file(current_file_path.to_str().unwrap()) {
                Ok(new_rule) => {
                    current_rule = new_rule;
                    lsystem = LSystem::new(current_rule.clone());
                    needs_regeneration = true;
                    println!("L-system reloaded");
                }
                Err(e) => eprintln!("Error reloading file: {}", e),
            }
        }
        
        // Handle menu input
        if let Some(selected_file) = menu.handle_input(&window) {
            match load_rule_from_file(selected_file.to_str().unwrap()) {
                Ok(new_rule) => {
                    current_rule = new_rule;
                    current_file_path = selected_file;
                    lsystem = LSystem::new(current_rule.clone());
                    needs_regeneration = true;
                    println!("Loaded L-system: {}", current_rule.name);
                }
                Err(e) => eprintln!("Error loading file: {}", e),
            }
        }
        
        // Handle mouse input for camera control
        if let Some(mouse_pos) = window.get_mouse_pos(minifb::MouseMode::Clamp) {
            let mouse_vec = Vec2::new(mouse_pos.0, mouse_pos.1);
            
            if window.get_mouse_down(minifb::MouseButton::Left) {
                if !mouse_pressed {
                    camera.start_rotation(mouse_vec);
                    mouse_pressed = true;
                } else {
                    camera.update_rotation(mouse_vec);
                }
            } else if mouse_pressed {
                camera.stop_rotation();
                mouse_pressed = false;
            }
        }
        
        // Handle mouse wheel for zoom
        if let Some(scroll) = window.get_scroll_wheel() {
            camera.zoom(-scroll.1 * 0.1);
        }
        
        // Handle GUI input and parameter changes
        if gui.handle_input(&window) {
            // Apply GUI parameters to turtle
            if let Some(angle) = gui.get_parameter("Angle") {
                turtle.set_angle(angle);
            }
            if let Some(step_length) = gui.get_parameter("Step Length") {
                turtle.set_step_length(step_length);
            }
            needs_regeneration = true;
        }
        
        // Regenerate L-system if needed
        if needs_regeneration {
            lsystem.generate();
            println!("Generated {}: {} characters", current_rule.name, lsystem.current_string.len());
            needs_regeneration = false;
        }
        
        // Render
        renderer.clear();
        lsystem.draw_3d(&mut turtle, &mut renderer);
        renderer.render(&camera);
        
        // Get buffer from renderer
        let buffer = renderer.get_buffer();
        let mut display_buffer = buffer.to_vec();
        
        // Render menu overlay
        menu.render_to_buffer(&mut display_buffer, WIDTH, HEIGHT);
        
        // Render GUI overlay
        gui.render(&mut display_buffer, WIDTH, HEIGHT);
        
        // Render main menu overlay (on top of everything)
        main_menu.render(&mut display_buffer, WIDTH, HEIGHT, &current_rule.name);
        
        window.update_with_buffer(&display_buffer, WIDTH, HEIGHT).unwrap();
    }
}
