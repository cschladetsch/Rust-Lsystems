use glam::{Mat4, Vec3, Vec2};

#[derive(Debug, Clone)]
pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
    
    // Rotation state
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
    
    // Mouse interaction
    last_mouse_pos: Option<Vec2>,
    is_rotating: bool,
}

impl Camera {
    pub fn new(aspect: f32) -> Self {
        Self {
            position: Vec3::new(0.0, 5.0, 10.0),
            target: Vec3::ZERO,
            up: Vec3::Y,
            fov: 45.0_f32.to_radians(),
            aspect,
            near: 0.1,
            far: 1000.0,
            yaw: 0.0,
            pitch: -0.3,
            distance: 10.0,
            last_mouse_pos: None,
            is_rotating: false,
        }
    }
    
    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_lh(self.position, self.target, self.up)
    }
    
    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective_lh(self.fov, self.aspect, self.near, self.far)
    }
    
    pub fn update_from_angles(&mut self) {
        let x = self.distance * self.yaw.cos() * self.pitch.cos();
        let y = self.distance * self.pitch.sin();
        let z = self.distance * self.yaw.sin() * self.pitch.cos();
        
        self.position = self.target + Vec3::new(x, y, z);
    }
    
    pub fn start_rotation(&mut self, mouse_pos: Vec2) {
        self.is_rotating = true;
        self.last_mouse_pos = Some(mouse_pos);
    }
    
    pub fn stop_rotation(&mut self) {
        self.is_rotating = false;
        self.last_mouse_pos = None;
    }
    
    pub fn update_rotation(&mut self, mouse_pos: Vec2) {
        if let Some(last_pos) = self.last_mouse_pos {
            if self.is_rotating {
                let delta = mouse_pos - last_pos;
                self.yaw -= delta.x * 0.01;
                self.pitch -= delta.y * 0.01;
                
                // Clamp pitch to prevent flipping
                self.pitch = self.pitch.clamp(-std::f32::consts::FRAC_PI_2 + 0.1, 
                                             std::f32::consts::FRAC_PI_2 - 0.1);
                
                self.update_from_angles();
            }
        }
        self.last_mouse_pos = Some(mouse_pos);
    }
    
    pub fn zoom(&mut self, delta: f32) {
        self.distance *= 1.0 + delta * 0.1;
        self.distance = self.distance.clamp(1.0, 100.0);
        self.update_from_angles();
    }
    
    pub fn set_aspect_ratio(&mut self, aspect: f32) {
        self.aspect = aspect;
    }
}