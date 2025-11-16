use snafu::prelude::*;

use crate::exposure::ExposureCompensation;

pub fn parse_exposure_compensation(
    value: &str,
) -> Result<ExposureCompensation, ParseExposureCompensationError> {
    let s = value.trim();

    if s == "0" || s == "" {
        return Ok(ExposureCompensation::new(0.));
    }

    let (sign, rest) = if let Some(rest) = s.strip_prefix('+') {
        (1., rest)
    } else if let Some(rest) = s.strip_prefix('-') {
        (-1., rest)
    } else {
        (1., s)
    };

    if !contains_fraction(rest) {
        let value: f32 = rest
            .parse()
            .map_err(|_| ParseExposureCompensationError::InvalidInteger)?;

        return Ok(ExposureCompensation::new(sign * value));
    }

    let parts: Vec<&str> = rest.split("").collect();

    let value = if parts.len() == 3 {
        parse_fraction(parts[1])?
    } else if parts.len() == 4 {
        let integer: f32 = parts[1]
            .parse()
            .map_err(|_| ParseExposureCompensationError::InvalidInteger)?;

        integer + parse_fraction(parts[2])?
    } else {
        return Err(ParseExposureCompensationError::InvalidFormat);
    };

    Ok(ExposureCompensation::new(sign * value))
}

fn parse_fraction(v: &str) -> Result<f32, ParseExposureCompensationError> {
    match v {
        "⅓" => Ok(1. / 3.),
        "⅔" => Ok(2. / 3.),
        "½" => Ok(1. / 2.),
        _ => Err(ParseExposureCompensationError::InvalidFormat),
    }
}

fn contains_fraction(v: &str) -> bool {
    if v.contains('⅓') || v.contains('⅔') || v.contains('½') {
        true
    } else {
        false
    }
}

#[derive(Debug, Snafu)]
pub enum ParseExposureCompensationError {
    #[snafu(display("Invalid Exposure Compensation, the value must be similar to +1⅓"))]
    InvalidFormat,

    #[snafu(display(
        "Invalid Integer in the Exposure Compensation, the first value should be an integer value"
    ))]
    InvalidInteger,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_zero() {
        assert_eq!(parse_exposure_compensation("0").unwrap().value(), 0.);
        assert_eq!(parse_exposure_compensation("").unwrap().value(), 0.);
    }

    #[test]
    fn test_parse_half() {
        assert_eq!(parse_exposure_compensation("+½").unwrap().value(), 0.5);
        assert_eq!(parse_exposure_compensation("-½").unwrap().value(), -0.5);
    }

    #[test]
    fn test_parse_half_with_integer() {
        assert_eq!(parse_exposure_compensation("+1½").unwrap().value(), 1.5);
        assert_eq!(parse_exposure_compensation("-2½").unwrap().value(), -2.5);
    }

    #[test]
    fn test_parse_thirds() {
        assert_eq!(parse_exposure_compensation("+⅓").unwrap().value(), 1. / 3.);
        assert_eq!(parse_exposure_compensation("⅔").unwrap().value(), 2. / 3.);

        assert_eq!(parse_exposure_compensation("-⅓").unwrap().value(), -1. / 3.);
        assert_eq!(parse_exposure_compensation("-⅔").unwrap().value(), -2. / 3.);
    }

    #[test]
    fn test_parse_thirds_with_integer() {
        assert_eq!(
            parse_exposure_compensation("+1⅓").unwrap().value(),
            1. / 3. + 1.
        );
        assert_eq!(
            parse_exposure_compensation("2⅔").unwrap().value(),
            2. / 3. + 2.
        );

        assert_eq!(
            parse_exposure_compensation("-3⅓").unwrap().value(),
            (1. / 3. + 3.) * -1.
        );
        assert_eq!(
            parse_exposure_compensation("-4⅔").unwrap().value(),
            (2. / 3. + 4.) * -1.
        );
    }

    #[test]
    fn test_parse_integers() {
        assert_eq!(parse_exposure_compensation("+1").unwrap().value(), 1.);
        assert_eq!(parse_exposure_compensation("-2").unwrap().value(), -2.);

        assert_eq!(parse_exposure_compensation("+10").unwrap().value(), 10.);
        assert_eq!(parse_exposure_compensation("15").unwrap().value(), 15.);
    }
}
