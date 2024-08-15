
struct ProgressState {
    height: f32,
    x: u32,
    y: u32,
    step_size: u32,
    next_step: bool,
}

pub trait ProgressTracker {
    fn start(&mut self, height_map: &HeightMap);
    fn update(&mut self, state: ProgressState);
}