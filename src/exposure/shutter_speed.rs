use regex::Regex;
use snafu::prelude::*;

#[derive(Debug)]
pub struct ShutterSpeed(String);

impl ShutterSpeed {
    pub fn try_new(value: &str) -> Result<Self, ShutterSpeedError> {
        if !ShutterSpeed::is_valid(value) {
            return Err(ShutterSpeedError::Invalid {
                value: value.to_string(),
            });
        }

        Ok(ShutterSpeed(value.to_string()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    fn is_valid(value: &str) -> bool {
        let expr = Regex::new(r#"^(?:1/[1-9]\d*|[1-9]\d*(?:\"|s)?)$"#).unwrap();
        expr.is_match(value)
    }
}

#[derive(Debug, Snafu)]
pub enum ShutterSpeedError {
    #[snafu(display("Invalid shutter speed: {}", value))]
    Invalid { value: String },
}
