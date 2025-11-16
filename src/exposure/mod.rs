use regex::Regex;

mod camera;
mod exposure_compensation;
mod lens;
mod shutter_speed;

pub use camera::*;
pub use exposure_compensation::*;
pub use lens::*;
pub use shutter_speed::*;

#[derive(Debug)]
pub struct Exposure {
    pub camera: Option<Camera>,
    pub lens: Option<Lens>,
    pub date: Option<String>,
    pub iso: Option<i32>,
    pub aperture: Option<f32>,
    pub shutter_speed: Option<ShutterSpeed>,
    pub exposure_compensation: Option<ExposureCompensation>,
}

impl Exposure {
    pub fn is_shutter_speed_valid(txt: &str) -> bool {
        let expr = Regex::new(r#"1/\d+|\d+""#).unwrap();
        expr.is_match(txt)
    }
}
