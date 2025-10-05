#![warn(missing_docs)]

// https://www.astro.princeton.edu/~gk/A403/constants.pdf

#[macro_use]
extern crate uom;

pub mod angle;
pub mod illuminance;
pub mod irradiance;
pub mod length;
pub mod luminosity;
pub mod luminous_intensity;
pub mod mass;
pub mod solid_angle;
pub mod time;

#[cfg(test)]
pub(crate) mod tests {
    pub(crate) const TEST_ACCURACY: f64 = 1e-5;

    pub(crate) fn eq_within(actual: f64, expected: f64, accuracy: f64) -> bool {
        if (actual - expected).abs() >= accuracy {
            println!("actual  : {}", actual);
            println!("expected: {}", expected);
            println!("accuracy: {}", accuracy);
            false
        } else {
            true
        }
    }

    pub(crate) fn eq(actual: f64, expected: f64) -> bool {
        eq_within(actual, expected, TEST_ACCURACY)
    }
}
