use std::fmt;

pub struct WeatherReport {
    temp: f64,
    uv: u8,
}

impl WeatherReport {
    fn new(temp: f64, uv: u8) -> Self {
        WeatherReport { temp, uv }
    }
}

impl fmt::Display for WeatherReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "WeatherReport {{ temp: {}, uv: {} }}",
            self.temp, self.uv
        )
    }
}

pub fn get_weather() -> WeatherReport {
    let temp: f64 = 63.3;
    let uv: u8 = 4;

    WeatherReport::new(temp, uv)
}
