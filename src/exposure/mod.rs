use regex::Regex;

mod exposure_compensation;

pub use exposure_compensation::*;

#[derive(Debug)]
pub struct Exposure {
    pub camera_name: Option<String>,
    pub camera_maker: Option<String>,
    pub lens_name: Option<String>,
    pub lens_maker: Option<String>,
    pub focal_length: Option<f32>,
    pub date: Option<String>,
    pub iso: Option<i32>,
    pub aperture: Option<f32>,
    pub shutter_speed: Option<String>,
    pub exposure_compensation: Option<ExposureCompensation>,
}

impl Exposure {
    pub fn is_shutter_speed_valid(txt: &str) -> bool {
        let expr = Regex::new(r#"1/\d+|\d+""#).unwrap();
        expr.is_match(txt)
    }
}
