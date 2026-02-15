//! Illuminance is a measure for how much a surface is illuminated.
//!
//! Illuminance is defined as luminous flux per unit area, and measured in lux (lx), which is equivalent to lumens per square meter (lm/m²).
//! In contrast to Irradiance, Illuminance is weighted according to the sensitivity of the human eye to different wavelengths of light.
//! In an idealised world without extinction, the illuminance of a source as seen from a distance is equal to its luminous intensity divided by the surface area of a sphere with a radius equal to that distance.
//!
//! https://en.wikipedia.org/wiki/Illuminance
//!
//! This module provides a typed unit, and some functions to convert between apparent astronomical magnitude and illuminance.

/// Unit re-export
pub use uom::si::illuminance::lux;

/// Type re-export
pub use uom::si::f64::Illuminance;

#[inline(always)]
/// Returns the illuminance corresponding to an apparent visible magnitude of zero.
///
/// Per its original definition, the apparent magnitude zero point corresponds to the perceived brightness of the star Vega.
///
/// https://en.wikipedia.org/wiki/Apparent_magnitude
pub fn aparent_visible_magnitude_zero() -> Illuminance {
    Illuminance::new::<lux>(2.6e-6)
}

#[inline(always)]
/// Converts an apparent astronomical magnitude to Illuminance, a measure of the perceived brightniss.
///
/// Based on the formula:
/// E = E0 * 10^(-m/2.5)
///
/// https://en.wikipedia.org/wiki/Apparent_magnitude
pub fn apparent_magnitude_to_illuminance(apparent_magnitude: f64) -> Illuminance {
    let exponent = apparent_magnitude / -2.5;
    aparent_visible_magnitude_zero() * 10_f64.powf(exponent)
}

#[inline(always)]
/// Converts Illuminance to an apparent astronomical magnitude.
///
/// Based on the formula:
/// m = -2.5 * log10(E / E0)
///
/// https://en.wikipedia.org/wiki/Apparent_magnitude
pub fn illuminance_to_apparent_magnitude(illuminance: Illuminance) -> f64 {
    -2.5 * (illuminance / aparent_visible_magnitude_zero())
        .log10()
        .value
}

#[cfg(test)]
mod tests {
    use uom::si::{
        f64::Length,
        length::{astronomical_unit, light_year},
    };

    use super::*;
    use crate::{
        luminous_intensity::{calc_illuminance, solar_luminous_intensity},
        tests::{eq, eq_within},
    };

    const REAL_DATA_TEST_ACCURACY: f64 = 0.05;

    #[test]
    fn apparent_magnitude_roundtrip() {
        for apparent_magnitude in -10..10 {
            let input = apparent_magnitude as f64;
            let illuminance = apparent_magnitude_to_illuminance(input);
            let output = illuminance_to_apparent_magnitude(illuminance);
            assert!(eq(input, output));
        }
    }

    #[test]
    fn apparent_magnitude_difference_of_1_corresponds_to_factor_of_2_512() {
        let expected = 100_f64.powf(1. / 5.);
        for i in -10..10 {
            let illuminance = apparent_magnitude_to_illuminance(i as f64);
            let illuminance_plus_1 = apparent_magnitude_to_illuminance((i + 1) as f64);
            let ratio = illuminance / illuminance_plus_1;
            println!("i: {}, ratio: {}", i, ratio.value);
            assert!(eq(ratio.value, expected));
        }
    }

    #[test]
    fn test_sunlight() {
        let luminous_intensity = solar_luminous_intensity();
        let distance = Length::new::<astronomical_unit>(1.);
        let illuminance = calc_illuminance(luminous_intensity, distance);
        let apparent_magnitude = illuminance_to_apparent_magnitude(illuminance);
        let expected_app_mag = -26.74;
        assert!(eq_within(apparent_magnitude, expected_app_mag, 0.05));
    }

    #[test]
    fn test_lux_of_sunlight() {
        let apparent_magnitude = -26.72;
        let illuminance = apparent_magnitude_to_illuminance(apparent_magnitude);

        let expected_lux = 107_527.;
        assert!(eq_within(
            illuminance.value,
            expected_lux,
            5. * REAL_DATA_TEST_ACCURACY * expected_lux
        ));
    }

    #[test]
    fn test_sirius() {
        let luminous_intensity = 22. * solar_luminous_intensity();
        let distance = Length::new::<light_year>(8.6);
        let illuminance = calc_illuminance(luminous_intensity, distance);
        let apparent_magnitude = illuminance_to_apparent_magnitude(illuminance);
        let expected_app_mag = -1.46;
        assert!(eq_within(apparent_magnitude, expected_app_mag, 0.05));
    }
}
