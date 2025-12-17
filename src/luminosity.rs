//! Luminosity is the total amount of energy radiated by a star, galaxy, or other object per unit time. It is measured in watts (W).
//!
//! The amount of luminosity emitted by an object is an intrinsic property and does not depend on the distance from the observer.
//! In an idealised world without extinction, the luminosity of a source as seen from a distance is equal to its irradiance multiplied by the surface area of a sphere with a radius equal to that distance.
//!
//! https://en.wikipedia.org/wiki/Luminosity

use uom::si::{
    f64::{LuminousIntensity, Power},
    power::watt,
};

use super::luminous_intensity::solar_luminous_intensity;

/// Luminosity is the total amount of energy radiated by a star, galaxy, or other object per unit time. It is measured in watts (W).
///
/// The amount of luminosity emitted by an object is an intrinsic property and does not depend on the distance from the observer.
/// In an idealised world without extinction, the luminosity of a source as seen from a distance is equal to its irradiance multiplied by the surface area of a sphere with a radius equal to that distance.
///
/// https://en.wikipedia.org/wiki/Luminosity
///
/// This is a type alias for uom::si::f64::Power, which uses the same units.
pub type Luminosity = uom::si::f64::Power;

#[inline(always)]
/// The total luminosity of the Sun.
pub fn solar_luminosity() -> Power {
    Power::new::<watt>(3.828e26)
}

/// Approximates the luminosity corresponding to a given luminous intensity.
///
/// This value assumes that the source emits radiation with the same spectral distribution as the Sun.
pub fn luminous_intensity_to_luminosity(luminous_intensity: LuminousIntensity) -> Luminosity {
    (luminous_intensity / solar_luminous_intensity()) * solar_luminosity()
}
