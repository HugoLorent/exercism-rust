// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

// Number of seconds in an Earth year
const SECONDS_IN_EARTH_YEAR: f64 = 31_557_600.0;

#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self(s as f64)
    }
}

pub trait Planet {
    /// Returns the orbital period of the planet in Earth years
    fn orbital_period() -> f64;

    /// Calculates age in planet years for a given duration in seconds
    fn years_during(d: &Duration) -> f64 {
        d.0 / (SECONDS_IN_EARTH_YEAR * Self::orbital_period())
    }
}

// Define a macro that creates a struct for each planet and implements the Planet trait
macro_rules! planet {
    ($planet:ident, $orbital_period:expr) => {
        pub struct $planet;

        impl Planet for $planet {
            fn orbital_period() -> f64 {
                $orbital_period
            }
        }
    };
}

// Use the macro to define all planets with their orbital periods relative to Earth
planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);
