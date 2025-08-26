use glam::{Mat4, Vec3, Vec4};
use crate::camera::Camera;

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Vec3,
    pub color: Vec3,
}

impl Vertex {
    pub fn new(position: Vec3, color: Vec3) -> Self {
        Self { position, color }
    }
}

#[derive(Debug, Clone)]
pub struct Line {
    pub start: Vertex,
    pub end: Vertex,
    pub thickness: f32,
}

impl Line {
    pub fn new(start: Vertex, end: Vertex) -> Self {
        Self { start, end, thickness: 1.0 }
    }
    
    pub fn new_with_thickness(start: Vertex, end: Vertex, thickness: f32) -> Self {
        Self { start, end, thickness }
    }
}

pub struct Renderer {
    lines: Vec<Line>,
    width: usize,
    height: usize,
    buffer: Vec<u32>,
    depth_buffer: Vec<f32>,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            lines: Vec::new(),
            width,
            height,
            buffer: vec![0; width * height],
            depth_buffer: vec![f32::MAX; width * height],
        }
    }
    
    pub fn clear(&mut self) {
        self.buffer.fill(0x000020); // Dark blue background
        self.depth_buffer.fill(f32::MAX);
        self.lines.clear();
    }
    
    pub fn add_line(&mut self, line: Line) {
        self.lines.push(line);
    }
    
    pub fn render(&mut self, camera: &Camera) {
        let view_proj = camera.projection_matrix() * camera.view_matrix();
        let lines = self.lines.clone(); // Clone to avoid borrow checker issues
        
        for line in &lines {
            self.draw_line_3d(&line.start, &line.end, line.thickness, &view_proj);
        }
    }
    
    fn draw_line_3d(&mut self, start: &Vertex, end: &Vertex, thickness: f32, view_proj: &Mat4) {
        let start_clip = *view_proj * Vec4::new(start.position.x, start.position.y, start.position.z, 1.0);
        let end_clip = *view_proj * Vec4::new(end.position.x, end.position.y, end.position.z, 1.0);
        
        // Perspective divide
        if start_clip.w <= 0.0 || end_clip.w <= 0.0 {
            return; // Behind camera
        }
        
        let start_ndc = Vec3::new(
            start_clip.x / start_clip.w,
            start_clip.y / start_clip.w,
            start_clip.z / start_clip.w,
        );
        
        let end_ndc = Vec3::new(
            end_clip.x / end_clip.w,
            end_clip.y / end_clip.w,
            end_clip.z / end_clip.w,
        );
        
        // Convert to screen space
        let start_screen = Vec3::new(
            (start_ndc.x + 1.0) * 0.5 * self.width as f32,
            (1.0 - start_ndc.y) * 0.5 * self.height as f32,
            start_ndc.z,
        );
        
        let end_screen = Vec3::new(
            (end_ndc.x + 1.0) * 0.5 * self.width as f32,
            (1.0 - end_ndc.y) * 0.5 * self.height as f32,
            end_ndc.z,
        );
        
        self.draw_line_2d(start_screen, end_screen, start.color, end.color, thickness);
    }
    
    fn draw_line_2d(&mut self, start: Vec3, end: Vec3, start_color: Vec3, end_color: Vec3, thickness: f32) {
        let dx = end.x - start.x;
        let dy = end.y - start.y;
        let length = (dx * dx + dy * dy).sqrt();
        
        if length == 0.0 {
            return;
        }
        
        // Perpendicular vector for thickness
        let perp_x = -dy / length * thickness * 0.5;
        let perp_y = dx / length * thickness * 0.5;
        
        let steps = (length as i32).max(1);
        
        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            
            let center_x = start.x + t * dx;
            let center_y = start.y + t * dy;
            let z = start.z + t * (end.z - start.z);
            
            let color = start_color + t * (end_color - start_color);
            let r = (color.x.clamp(0.0, 1.0) * 255.0) as u32;
            let g = (color.y.clamp(0.0, 1.0) * 255.0) as u32;
            let b = (color.z.clamp(0.0, 1.0) * 255.0) as u32;
            let pixel_color = (r << 16) | (g << 8) | b;
            
            // Draw thick line as a series of circles
            let radius = (thickness * 0.5).max(1.0) as i32;
            for dy in -radius..=radius {
                for dx in -radius..=radius {
                    if (dx * dx + dy * dy) as f32 <= radius as f32 * radius as f32 {
                        let px = (center_x as i32 + dx).max(0).min(self.width as i32 - 1);
                        let py = (center_y as i32 + dy).max(0).min(self.height as i32 - 1);
                        
                        if px >= 0 && px < self.width as i32 && py >= 0 && py < self.height as i32 {
                            let idx = py as usize * self.width + px as usize;
                            
                            if z < self.depth_buffer[idx] {
                                self.depth_buffer[idx] = z;
                                self.buffer[idx] = pixel_color;
                            }
                        }
                    }
                }
            }
        }
    }
    
    pub fn get_buffer(&self) -> &[u32] {
        &self.buffer
    }
    
    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.buffer.resize(width * height, 0);
        self.depth_buffer.resize(width * height, f32::MAX);
    }
}