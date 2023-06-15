// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s as f64)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        let earth_years = 0.2408467;

        let year_sec = 31557600.0 * earth_years;

        d.0 / year_sec
    }
}

impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        let earth_years = 0.61519726;

        let year_sec = 31557600.0 * earth_years;

        d.0 / year_sec
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        let earth_years = 1.0;

        let year_sec = 31557600.0 * earth_years;

        d.0 / year_sec
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        let earth_years = 1.8808158;

        let year_sec = 31557600.0 * earth_years;

        d.0 / year_sec
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        let earth_years = 11.862615;

        let year_sec = 31557600.0 * earth_years;

        d.0 / year_sec
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        let earth_years = 29.447498;

        let year_sec = 31557600.0 * earth_years;

        d.0 / year_sec
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        let earth_years = 84.016846;

        let year_sec = 31557600.0 * earth_years;

        d.0 / year_sec
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        let earth_years = 164.79132;

        let year_sec = 31557600.0 * earth_years;

        d.0 / year_sec
    }
}
