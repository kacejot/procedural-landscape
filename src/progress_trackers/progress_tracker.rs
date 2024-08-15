
struct ProgressState {
    old_height: f32,
    height: f32,
    x: u32,
    y: u32,
}

pub trait ProgressTracker {
    fn start(&mut self, height_map: &HeightMap);
    fn update_point(&mut self, state: ProgressState);
    fn update_step(&mut self, step_size: u32);
}