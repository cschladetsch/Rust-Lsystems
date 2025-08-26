use std::process::Command;
use std::path::{Path, PathBuf};
use std::fs;
use std::io::Write;

pub struct Editor {
    editor_command: String,
    rules_directory: PathBuf,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            editor_command: Self::detect_editor(),
            rules_directory: PathBuf::from("rules"),
        }
    }
    
    fn detect_editor() -> String {
        // Check environment variables first
        if let Ok(editor) = std::env::var("EDITOR") {
            return editor;
        }
        
        if let Ok(visual) = std::env::var("VISUAL") {
            return visual;
        }
        
        // Try common editors in order of preference
        let editors = ["nvim", "vim", "nano", "gedit", "code"];
        
        for editor in &editors {
            if Command::new("which")
                .arg(editor)
                .output()
                .map(|output| output.status.success())
                .unwrap_or(false)
            {
                return editor.to_string();
            }
        }
        
        // Fallback
        "nano".to_string()
    }
    
    pub fn edit_file(&self, file_path: Option<&Path>) -> Result<PathBuf, String> {
        let path = if let Some(path) = file_path {
            path.to_path_buf()
        } else {
            self.create_new_file()?
        };
        
        // Ensure the file exists
        if !path.exists() {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create directory: {}", e))?;
            }
            
            // Create a template L-system file
            self.create_template_file(&path)?;
        }
        
        // Launch editor
        let status = Command::new(&self.editor_command)
            .arg(&path)
            .status()
            .map_err(|e| format!("Failed to launch editor '{}': {}", self.editor_command, e))?;
        
        if !status.success() {
            return Err(format!("Editor exited with error code: {:?}", status.code()));
        }
        
        Ok(path)
    }
    
    fn create_new_file(&self) -> Result<PathBuf, String> {
        // Get a unique filename
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| format!("System time error: {}", e))?
            .as_secs();
            
        let filename = format!("custom_{}.json", timestamp);
        let path = self.rules_directory.join(filename);
        
        Ok(path)
    }
    
    fn create_template_file(&self, path: &Path) -> Result<(), String> {
        let template = r#"{
  "name": "Custom L-System",
  "axiom": "F",
  "rules": {
    "F": "F+F-F-F+F"
  },
  "angle": 90.0,
  "iterations": 4,
  "step_length": 10.0,
  "start_position": [0.0, 0.0, 0.0],
  "start_direction": [0.0, 1.0, 0.0],
  "colors": {
    "depth_based": true,
    "palette": [
      [0.0, 1.0, 0.0],
      [0.8, 0.4, 0.0],
      [1.0, 0.0, 0.0]
    ]
  },
  "description": "A simple square-based fractal pattern"
}"#;
        
        let mut file = fs::File::create(path)
            .map_err(|e| format!("Failed to create file: {}", e))?;
            
        file.write_all(template.as_bytes())
            .map_err(|e| format!("Failed to write template: {}", e))?;
            
        Ok(())
    }
    
    pub fn edit_current_file(&self, current_file: Option<&Path>) -> Result<Option<PathBuf>, String> {
        match current_file {
            Some(path) => {
                self.edit_file(Some(path))?;
                Ok(Some(path.to_path_buf()))
            },
            None => {
                // Create and edit a new file
                let path = self.edit_file(None)?;
                Ok(Some(path))
            }
        }
    }
    
    pub fn open_rules_directory(&self) -> Result<(), String> {
        // Ensure directory exists
        fs::create_dir_all(&self.rules_directory)
            .map_err(|e| format!("Failed to create rules directory: {}", e))?;
        
        // Try to open the directory with system file manager
        let status = if cfg!(target_os = "linux") {
            Command::new("xdg-open").arg(&self.rules_directory).status()
        } else if cfg!(target_os = "macos") {
            Command::new("open").arg(&self.rules_directory).status()
        } else if cfg!(target_os = "windows") {
            Command::new("explorer").arg(&self.rules_directory).status()
        } else {
            return Err("Unsupported operating system".to_string());
        };
        
        match status {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err("File manager exited with error".to_string()),
            Err(e) => Err(format!("Failed to open file manager: {}", e)),
        }
    }
    
    pub fn set_editor(&mut self, editor: String) {
        self.editor_command = editor;
    }
    
    pub fn get_editor(&self) -> &str {
        &self.editor_command
    }
}