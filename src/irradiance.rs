//! Irradiance is the power of electromagnetic radiation per unit area (radiative flux) incident on a surface. It is measured in watts per square meter (W/m²).
//!
//! In an idealised world without extinction, the irradiance of a source as seen from a distance is equal to its luminosity divided by the surface area of a sphere with a radius equal to that distance.
//!
//! https://en.wikipedia.org/wiki/Irradiance

/// Unit re-export
pub use uom::si::heat_flux_density::watt_per_square_meter;

/// Irradiance is the power of electromagnetic radiation per unit area (radiative flux) incident on a surface. It is measured in watts per square meter (W/m²).
///
/// In an idealised world without extinction, the irradiance of a source as seen from a distance is equal to its luminosity divided by the surface area of a sphere with a radius equal to that distance.
///
/// https://en.wikipedia.org/wiki/Irradiance
///
/// This type is an alias for uom::si::f64::HeatFluxDensity, which uses the same units.
pub type Irradiance = uom::si::f64::HeatFluxDensity;

#[inline(always)]
/// Returns the approximate irradiance corresponding to a bolometric magnitude of zero.
///
/// This value assumes that the source emits radiation with the same spectral distribution as the Sun.
pub fn irradiance_of_bolometric_zero() -> Irradiance {
    Irradiance::new::<watt_per_square_meter>(2.518e-8)
}
