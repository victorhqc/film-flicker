use snafu::prelude::*;

#[derive(Debug)]
pub struct GeoLocation {
    pub latitude: f64,
    pub longitude: f64,
}

impl TryFrom<&str> for GeoLocation {
    type Error = GeoLocationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split(';').collect();
        if parts.len() != 2 {
            return Err(GeoLocationError::InvalidFormat);
        }

        let latitude = parts[0]
            .parse::<f64>()
            .map_err(|_| GeoLocationError::InvalidLatitude)?;
        let longitude = parts[1]
            .parse::<f64>()
            .map_err(|_| GeoLocationError::InvalidLongitude)?;

        if !(-90.0..=90.0).contains(&latitude) {
            return Err(GeoLocationError::InvalidLatitude)?;
        }
        if !(-180.0..=180.0).contains(&longitude) {
            return Err(GeoLocationError::InvalidLongitude)?;
        }

        Ok(GeoLocation {
            latitude,
            longitude,
        })
    }
}

#[derive(Debug, Snafu)]
pub enum GeoLocationError {
    #[snafu(display("Invalid Geo Location format"))]
    InvalidFormat,

    #[snafu(display("Invalid Latitude"))]
    InvalidLatitude,

    #[snafu(display("Invalid Longitude"))]
    InvalidLongitude,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(
            GeoLocation::try_from("52.5173885;13.3951309")
                .unwrap()
                .latitude,
            52.5173885
        );
        assert_eq!(
            GeoLocation::try_from("52.5173885;13.3951309")
                .unwrap()
                .longitude,
            13.3951309
        );
    }

    #[test]
    fn test_negative_values() {
        assert_eq!(
            GeoLocation::try_from("-40.7128;-74.0060").unwrap().latitude,
            -40.7128
        );
        assert_eq!(
            GeoLocation::try_from("-40.7128;-74.0060")
                .unwrap()
                .longitude,
            -74.0060
        );
    }
}
