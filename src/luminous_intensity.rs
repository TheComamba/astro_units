//! Luminous Intensity is a measure of the wavelength-weighted power emitted by a light source in a particular direction per unit solid angle.
//!
//! It is measured in candelas (cd), which is equivalent to lumens per steradian (lm/sr).
//! In contrast to Luminosity, Luminous Intensity is a measure of the brightness of a source as perceived by the human eye.
//!
//! https://en.wikipedia.org/wiki/Luminous_intensity
//!
//! This module provides some functions to convert between absolute/apparent astronomical magnitude and luminous intensity.

use uom::si::{
    f64::{Illuminance, Length, SolidAngle},
    length::parsec,
    solid_angle::steradian,
};

use super::illuminance::{apparent_magnitude_to_illuminance, illuminance_to_apparent_magnitude};

/// Unit re-export.
pub use uom::si::luminous_intensity::candela;

/// Type re-export.
pub use uom::si::f64::LuminousIntensity;

#[inline(always)]
/// The luminous intensity of the Sun.
pub fn solar_luminous_intensity() -> LuminousIntensity {
    LuminousIntensity::new::<candela>(2.98e27)
}

#[inline(always)]
/// Converts luminous intensity to a number of solar luminosities.
pub fn luminous_intensity_to_solar_luminosities(luminous_intensity: LuminousIntensity) -> f64 {
    (luminous_intensity / solar_luminous_intensity()).into()
}

#[inline(always)]
/// Converts an absolute astronomical magnitude to Luminous Intensity.
///
/// https://en.wikipedia.org/wiki/Absolute_magnitude
pub fn absolute_magnitude_to_luminous_intensity(absolute_magnitude: f64) -> LuminousIntensity {
    let ten_pc = Length::new::<parsec>(10.);
    let illuminance = apparent_magnitude_to_illuminance(absolute_magnitude);
    calc_luminous_intensity(illuminance, ten_pc)
}

#[inline(always)]
/// Converts Luminous Intensity to an absolute astronomical magnitude.
///
/// https://en.wikipedia.org/wiki/Absolute_magnitude
pub fn luminous_intensity_to_absolute_magnitude(luminous_intensity: LuminousIntensity) -> f64 {
    let ten_pc = Length::new::<parsec>(10.);
    let illuminance = calc_illuminance(luminous_intensity, ten_pc);
    illuminance_to_apparent_magnitude(illuminance)
}

#[inline(always)]
/// Calculates the Illuminance received by a source with a given Lunimous Intensity at a given distance.
pub fn calc_illuminance(luminous_intensity: LuminousIntensity, distance: Length) -> Illuminance {
    (luminous_intensity * SolidAngle::new::<steradian>(1.) / (distance * distance)).into()
}

#[inline(always)]
/// Using Illuminance received from a source at a given distance, this calculates the Luminous Intensity of the source.
pub fn calc_luminous_intensity(illuminance: Illuminance, distance: Length) -> LuminousIntensity {
    illuminance * (distance * distance) / SolidAngle::new::<steradian>(1.)
}

#[cfg(test)]
mod tests {
    use uom::si::{illuminance::lux, length::meter};

    use super::*;
    use crate::tests::{eq, eq_within};

    const REAL_DATA_TEST_ACCURACY: f64 = 0.05;
    const ILLUMINANCE_AT_UNIT_DISTANCE: f64 = 1.;

    #[test]
    fn illuminance_roundtrip() {
        for i in -10..10 {
            let input = i as f64;
            let luminous_intensity = LuminousIntensity::new::<candela>(input);
            let distance = Length::new::<meter>(1.);
            let illuminance = calc_illuminance(luminous_intensity, distance);
            let output = calc_luminous_intensity(illuminance, distance);
            assert!(eq(input, output.value));
        }
    }

    #[test]
    fn absolute_magnitude_roundtrip() {
        for i in -10..10 {
            let input = i as f64;
            let luminous_intensity = absolute_magnitude_to_luminous_intensity(input);
            let output = luminous_intensity_to_absolute_magnitude(luminous_intensity);
            assert!(eq(input, output));
        }
    }

    #[test]
    fn illuminance_of_1_cd_source_at_1_m() {
        let luminous_intensity = LuminousIntensity::new::<candela>(1.);
        let distance = Length::new::<meter>(1.);
        let illuminance = calc_illuminance(luminous_intensity, distance);
        let actual = illuminance.get::<lux>();
        let expected = ILLUMINANCE_AT_UNIT_DISTANCE;
        assert!(eq(actual, expected));
    }

    #[test]
    fn illuminance_is_proportional_to_luminous_intensity() {
        for i in 1..10 {
            let cd = i as f64;
            let luminous_intensity = LuminousIntensity::new::<candela>(cd);
            let distance = Length::new::<meter>(1.);
            let illuminance = calc_illuminance(luminous_intensity, distance);
            let expected = cd * ILLUMINANCE_AT_UNIT_DISTANCE;
            let actual = illuminance.get::<lux>();
            assert!(eq(actual, expected));
        }
    }

    #[test]
    fn illuminance_is_inversely_proportional_to_distance_squared() {
        for d in 1..10 {
            let distance = Length::new::<meter>(d as f64);
            let luminous_intensity = LuminousIntensity::new::<candela>(1.);
            let illuminance = calc_illuminance(luminous_intensity, distance);
            let expected = ILLUMINANCE_AT_UNIT_DISTANCE / (d * d) as f64;
            let actual = illuminance.get::<lux>();
            assert!(eq(actual, expected));
        }
    }

    #[test]
    fn apparent_and_absolute_magnitude_at_ten_parsecs_are_the_same() {
        let ten_pc = Length::new::<parsec>(10.);
        for i in -10..10 {
            let input = i as f64;
            let luminous_intensity = absolute_magnitude_to_luminous_intensity(input);
            let illuminance = calc_illuminance(luminous_intensity, ten_pc);
            let apparent_magnitude = illuminance_to_apparent_magnitude(illuminance);
            let absolute_magnitude = luminous_intensity_to_absolute_magnitude(luminous_intensity);
            assert!(eq(apparent_magnitude, absolute_magnitude));
        }
    }

    #[test]
    fn test_the_sun() {
        let sun_abs_mag = luminous_intensity_to_absolute_magnitude(solar_luminous_intensity());
        let expected = 4.83;
        assert!(eq_within(sun_abs_mag, expected, REAL_DATA_TEST_ACCURACY));
    }

    #[test]
    fn test_sirius() {
        let sun_abs_mag =
            luminous_intensity_to_absolute_magnitude(22. * solar_luminous_intensity());
        let expected = 1.43;
        assert!(eq_within(sun_abs_mag, expected, REAL_DATA_TEST_ACCURACY));
    }
}
