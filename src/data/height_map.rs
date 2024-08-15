#[derive(Debug)]
pub struct HeightMap {
    pub buf: Vec<f32>,
    pub chunk_size: u32,
    pub horizontal_chunk_num: u32,
    pub vertical_chunk_num: u32,
    pub width: u32,
    pub height: u32,
}

impl HeightMap {
    pub fn with_edge_size(chunk_size: u32) -> Self {
        Self {
            chunk_size,
            buf: vec![0.0; (chunk_size * chunk_size) as usize],
            horizontal_chunk_num: 1,
            vertical_chunk_num: 1,
            width: chunk_size,
            height: chunk_size,
        }
    }

    pub fn with_multiple_chunks(chunk_size: u32, horizontal_chunk_num: u32, vertical_chunk_num: u32) -> Self {
        Self {
            chunk_size,
            buf: vec![0.0; (chunk_size * chunk_size * horizontal_chunk_num * vertical_chunk_num) as usize],
            horizontal_chunk_num,
            vertical_chunk_num,
            width: chunk_size * horizontal_chunk_num,
            height: chunk_size * vertical_chunk_num,
        }
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
    }

    pub fn at(&self, x: i32, y: i32) -> Option<f32> {
        if !self.in_bounds(x, y) {
            return None;
        };
        let (x, y) = (x as u32, y as u32);
        Some(self.buf[(x + self.width * y) as usize])
    }

    pub fn at_mut(&mut self, x: u32, y: u32) -> &mut f32 {
        &mut self.buf[(x + self.width * y) as usize]
    }

    pub fn normalize(&mut self) {
        let (current_min, current_max) = self.min_max();
        let range = current_max - current_min;

        for height in &mut self.buf {
            *height = (*height - current_min) / range;
        }
    }

    pub fn min_max(&self) -> (f32, f32) {
        (
            self.buf
                .iter()
                .cloned()
                .fold(f32::INFINITY, f32::min),
            self.buf
                .iter()
                .cloned()
                .fold(f32::NEG_INFINITY, f32::max),
        )
    }
}
