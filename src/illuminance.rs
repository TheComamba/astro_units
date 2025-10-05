use uom::si::heat_flux_density::watt_per_square_meter;

use crate::irradiance::Irradiance;

pub type Illuminance = uom::si::f64::Luminance; // Hack until https://github.com/iliekturtles/uom/pull/512 is merged
#[allow(non_camel_case_types)]
pub type lux = uom::si::luminance::candela_per_square_meter;

#[inline(always)]
pub fn aparent_visible_magnitude_zero() -> Illuminance {
    Illuminance::new::<lux>(2.6e-6)
}

#[inline(always)]
pub fn irradiance_of_bolometric_zero() -> Irradiance {
    Irradiance::new::<watt_per_square_meter>(2.518e-8)
}

#[inline(always)]
pub fn apparent_magnitude_to_illuminance(apparent_magnitude: f64) -> Illuminance {
    let exponent = apparent_magnitude / -2.5;
    aparent_visible_magnitude_zero() * 10_f64.powf(exponent)
}

#[inline(always)]
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
        luminous_intensity::{luminous_intensity_to_illuminance, solar_luminous_intensity},
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
        let illuminance = luminous_intensity_to_illuminance(luminous_intensity, distance);
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
        let illuminance = luminous_intensity_to_illuminance(luminous_intensity, distance);
        let apparent_magnitude = illuminance_to_apparent_magnitude(illuminance);
        let expected_app_mag = -1.46;
        assert!(eq_within(apparent_magnitude, expected_app_mag, 0.05));
    }
}
