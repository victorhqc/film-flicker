#[derive(Debug)]
pub struct ExposureCompensation(f32);

impl ExposureCompensation {
    pub fn new(value: f32) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}
