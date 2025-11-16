use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use snafu::prelude::*;
use std::{fs, path::PathBuf};

#[derive(Debug, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Adapter {
    Lens(LensAdapter),
    Camera(CameraAdapter),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct LensAdapter {
    pub logbook_name: String,
    pub lens_name: String,
    pub lens_maker: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CameraAdapter {
    pub logbook_name: String,
    pub camera_name: String,
    pub camera_maker: String,
}

impl<'de> Deserialize<'de> for Adapter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

        if value.get("lens_name").is_some() {
            let lens_adapter: LensAdapter =
                serde_json::from_value(value).map_err(serde::de::Error::custom)?;
            Ok(Adapter::Lens(lens_adapter))
        } else if value.get("camera_name").is_some() {
            let camera_adapter: CameraAdapter =
                serde_json::from_value(value).map_err(serde::de::Error::custom)?;
            Ok(Adapter::Camera(camera_adapter))
        } else {
            Err(serde::de::Error::custom(
                "Could not determine adapter type from fields",
            ))
        }
    }
}

pub fn read_adapters(path: &PathBuf) -> Result<Vec<Adapter>, AdapterError> {
    let raw_adapter = fs::read_to_string(&path).context(ReadAdapterSnafu)?;
    let adapters: Vec<Adapter> = serde_json::from_str(&raw_adapter).context(ParseAdapterSnafu)?;

    Ok(adapters)
}

pub fn find_adapter<'a>(adapters: &'a Vec<Adapter>, name: &str) -> Option<&'a Adapter> {
    adapters.iter().find(|adapter| match adapter {
        Adapter::Lens(lens) => lens.logbook_name.trim() == name,
        Adapter::Camera(camera) => camera.logbook_name.trim() == name,
    })
}

#[derive(Debug, Snafu)]
pub enum AdapterError {
    #[snafu(display("Failed to read adapter file: {}", source))]
    ReadAdapter { source: std::io::Error },

    #[snafu(display("Failed to parse adapter file: {}", source))]
    ParseAdapter { source: serde_json::Error },
}
