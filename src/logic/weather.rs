pub struct WeatherReport {
    temp: f64,
    uv: u8,
}

impl WeatherReport {
    fn new(temp: f64, uv: u8) -> Self {
        WeatherReport { temp, uv }
    }
}

pub fn weather() -> WeatherReport {
    let temp: f64 = 63.3;
    let uv: u8 = 4;

    WeatherReport::new(temp, uv)
}
