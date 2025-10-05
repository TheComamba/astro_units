use uom::si::{
    f64::{LuminousIntensity, Power},
    power::watt,
};

pub type Luminosity = uom::si::f64::Power;

use super::luminous_intensity::solar_luminous_intensity;

#[inline(always)]
pub fn solar_luminosity() -> Power {
    Power::new::<watt>(3.828e26)
}

pub fn luminous_intensity_to_luminosity(luminous_intensity: LuminousIntensity) -> Luminosity {
    (luminous_intensity / solar_luminous_intensity()) * solar_luminosity()
}
