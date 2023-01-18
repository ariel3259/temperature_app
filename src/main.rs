//importing module
mod models;
mod services;

//importing structure
use crate::models::temp::Temperature;
use crate::services::average_weather::AverageWeather;
use std::io;

fn main() {
    let mut temperatures: Vec<Temperature> = vec![];
    let mut tmp: i32 = 0;
    let mut input: String = String::new();
    for _ in 0..6 {
        println!("Insert temperature of {} hours: ", tmp);
        io::stdin()
           .read_line(&mut input)
            .expect("Text");
        let value: i32 = input.trim().parse().expect("A number");
        let temperature: Temperature = Temperature::new(value);
        temperatures.push(temperature);
        input = String::new();
        if tmp > 24 {
            tmp += 6;
        }
        else {
            tmp = 0;
        }
        let average_weather: AverageWeather = AverageWeather::new(temperatures);
        let average: i32 = average_weather.get_average();
    }
}
 
