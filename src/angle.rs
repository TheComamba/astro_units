//! Utility functions for working with angles.

use std::f64::consts::PI;

use uom::si::{angle::radian, f64::Angle};

#[inline(always)]
/// Returns an angle of 90 degrees (π/2 radians).
pub fn quarter_circ() -> Angle {
    Angle::new::<radian>(2. * PI / 4.)
}

#[inline(always)]
/// Returns an angle of 180 degrees (π radians).
pub fn half_circ() -> Angle {
    Angle::new::<radian>(2. * PI / 2.)
}

#[inline(always)]
/// Returns an angle of 270 degrees (3π/2 radians).
pub fn three_quarter_circ() -> Angle {
    Angle::new::<radian>(2. * PI * 3. / 4.)
}

#[inline(always)]
/// Returns an angle of 360 degrees (2π radians).
pub fn full_circ() -> Angle {
    Angle::new::<radian>(2. * PI)
}

#[inline(always)]
/// Converts an angle in seconds of arc to an Angle type.
///
/// 1 second of arc = 1/3600 degrees = π/(648000) radians
/// Not to be confused with arcseconds.
pub fn angle_from_second_angle(second_angle: f64) -> Angle {
    second_angle * one_second_angle()
}

#[inline(always)]
/// Converts an Angle type to seconds of arc.
///
/// 1 second of arc = 1/3600 degrees = π/(648000) radians
/// Not to be confused with arcseconds.
pub fn angle_to_second_angle(angle: Angle) -> f64 {
    (angle / one_second_angle()).into()
}

#[inline(always)]
/// Returns the angle equivalent to one second of arc.
///
/// 1 second of arc = 1/3600 degrees = π/(648000) radians
/// Not to be confused with arcseconds.
pub fn one_second_angle() -> Angle {
    Angle::new::<radian>(2. * PI / (24. * 60. * 60.))
}

/// Normalize the angle to a range of −π to +π radians, -180° to 180°.
pub fn normalized_angle(mut angle: Angle) -> Angle {
    angle %= full_circ();
    if angle > half_circ() {
        angle -= full_circ();
    } else if angle < -half_circ() {
        angle += full_circ();
    }
    angle
}

/// Compares two angles for equality within a specified accuracy.
///
/// Returns true if the absolute difference between the two angles is less than the specified accuracy.
pub fn angle_eq_within(actual: Angle, expected: Angle, accuracy: Angle) -> bool {
    let diff = normalized_angle(actual - expected);
    diff.abs() < accuracy.abs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::eq;

    #[test]
    fn second_angle_roundtrip() {
        for i in -10..10 {
            let input = i as f64;
            let angle = angle_from_second_angle(input);
            let output = angle_to_second_angle(angle);
            assert!(eq(input, output));
        }
    }

    #[test]
    fn angle_eq_within_works() {
        for start in [-4., -PI, -PI / 2., 0., PI / 2., PI, 4.] {
            for accuracy in [-1., -0.1, -0.01, 0.01, 0.1, 1.] {
                let start = Angle::new::<radian>(start);
                let accuracy = Angle::new::<radian>(accuracy);
                let within = start + accuracy * 0.9;
                let outside = start + accuracy * 1.1;
                assert!(
                    angle_eq_within(start, within, accuracy),
                    "start: {}, within: {}, accuracy: {}",
                    start.get::<radian>(),
                    within.get::<radian>(),
                    accuracy.get::<radian>()
                );
                assert!(
                    !angle_eq_within(start, outside, accuracy),
                    "start: {}, outside: {}, accuracy: {}",
                    start.get::<radian>(),
                    outside.get::<radian>(),
                    accuracy.get::<radian>()
                );
            }
        }
    }
}
