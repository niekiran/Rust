#![allow(dead_code)]

/// Build a simple weather station application
/// The goal is to record and manage weather data for different cities.
/// Hints: Watch the demo video in the section named 'Vectors'
use std::io::Write;

#[derive(Debug)]
enum WeatherCondition {
    Sunny,
    Cloudy,
    Rainy,
    Snowy,
    Unknown,
}

struct Weather {
    temperature: f32,
    humidity: u32,
    condition: WeatherCondition,
}

impl Weather {
    fn new(temperature: f32, humidity: u32, condition: WeatherCondition) -> Self {
        Self {
            temperature,
            humidity,
            condition,
        }
    }

    fn update_temperature(&mut self, temperature: f32) {
        self.temperature = temperature;
    }

    fn update_humidity(&mut self, humidity: u32) {
        self.humidity = humidity;
    }

    fn update_condition(&mut self, condition: WeatherCondition) {
        self.condition = condition;
    }

    fn display(&self) -> String {
        let symbol: char = match self.condition {
            WeatherCondition::Cloudy => '\u{2601}',  // Cloud emoji
            WeatherCondition::Sunny => '\u{2600}',   // Sun emoji
            WeatherCondition::Snowy => '\u{2603}',   // Snowman emoji
            WeatherCondition::Rainy => '\u{2614}',   // Umbrella with rain drops emoji
            WeatherCondition::Unknown => '\u{2613}', // Saltire
        };

        format!(
            "Temperature: {:<10} Humidity: {:<10} Condition: {:<10?} {symbol}",
            self.temperature.to_string() + "°C",
            self.humidity.to_string() + "%",
            self.condition
        )
    }
}

struct CityWeather {
    city: String,
    weather: Weather,
}

impl CityWeather {
    fn new(city: &str, weather: Weather) -> Self {
        Self {
            city: city.to_string(),
            weather,
        }
    }

    fn update_weather(&mut self, weather: Weather) {
        self.weather = weather;
    }

    fn display(&self) -> String {
        format!("City: {:<20} {}", self.city, self.weather.display())
    }
}

struct WeatherStation {
    cities: Vec<CityWeather>,
}

impl WeatherStation {
    fn new() -> Self {
        Self { cities: Vec::new() }
    }

    fn add_city(&mut self, city_weather: CityWeather) {
        self.cities.push(city_weather);
        println!("✓ Weather report added successfuly")
    }

    fn find_city(&self, city: &str) -> Option<&CityWeather> {
        self.cities.iter().find(|&cw| cw.city == city)
    }

    fn list_cities(&self) {
        let font_red = "\x1b[31m";
        let font_reset = "\x1b[0m";

        if self.cities.is_empty() {
            println!("{}Nothing to display{}", font_red, font_reset)
        }
        for city_weather in &self.cities {
            println!("{}", city_weather.display());
        }
    }
}

fn display_prompt() {
    let font_green = "\x1b[32m";
    let font_reset = "\x1b[0m";

    print!(
        r#"{}
\\\\\\Simple Weather Station\\\\\\\\\\
\\\\\\Display all weather reports -- 0
\\\\\\Add a new weather report    -- 1
\\\\\\Display city weather report -- 2
\\\\\\Update weather report       -- 3
\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\
Enter your choice: {}"#,
        font_green, font_reset
    );

    std::io::stdout().flush().expect("Someting went wrong");
}

fn read_user_input() -> u32 {
    stdin_read_integer()
}

fn display_all_weather_reports(weather_station: &WeatherStation) {
    weather_station.list_cities();
}

fn get_new_weather_report_from_user() -> CityWeather {
    print!("Enter city name: ");
    stdout_flush();
    let city = stdin_read_string();

    print!("Enter temperature(°C): ");
    stdout_flush();
    let temperature = stdin_read_float();

    print!("Enter humidity(%): ");
    stdout_flush();
    let humidity = stdin_read_integer();

    print!("Describe weather Condition(Snowy, Cloudy, Rainy, Sunny): ");
    stdout_flush();
    let weather_desc = stdin_read_string().to_uppercase();

    let weather_condition = if weather_desc.contains("SNOW") {
        WeatherCondition::Snowy
    } else if weather_desc.contains("RAIN") {
        WeatherCondition::Rainy
    } else if weather_desc.contains("CLOUD") {
        WeatherCondition::Cloudy
    } else if weather_desc.contains("SUN") {
        WeatherCondition::Sunny
    } else {
        WeatherCondition::Unknown
    };

    let weather = Weather::new(temperature, humidity, weather_condition);

    CityWeather::new(&city, weather)
}

fn add_new_weather_report(weather_station: &mut WeatherStation) {
    weather_station.add_city(get_new_weather_report_from_user());
}

fn display_city_weather_report(weather_station: &WeatherStation) {
    print!("Enter city name: ");
    stdout_flush();
    let city = stdin_read_string();
    if let Some(cw) = weather_station.find_city(&city) {
        println!("{}", cw.display());
    } else {
        println!("City not found!");
    }
}

#[allow(unused_variables)]
fn update_city_weather_report(weather_station: &mut WeatherStation) {
    todo!();
}

fn main() {
    let mut weather_station: WeatherStation = WeatherStation::new();

    loop {
        display_prompt();

        match read_user_input() {
            0 => display_all_weather_reports(&weather_station),
            1 => add_new_weather_report(&mut weather_station),
            2 => display_city_weather_report(&weather_station),
            3 => update_city_weather_report(&mut weather_station),
            _ => {
                println!("Exiting...");
                break;
            }
        }

        print!("Press Enter key to continue...");
        stdout_flush();
        let _ = stdin_read_string();
    }
}

fn stdout_flush() {
    let _ = std::io::stdout().flush();
}

fn stdin_read_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error reading from stdin");
    input.trim().to_string()
}

fn stdin_read_float() -> f32 {
    let input = stdin_read_string();
    input
        .parse::<f32>()
        .expect("Error while converting string to f32")
}

fn stdin_read_integer() -> u32 {
    let input = stdin_read_string();
    input
        .parse::<u32>()
        .expect("Error while converting string to u32")
}
