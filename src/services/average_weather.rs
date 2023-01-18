
use crate::models::temp::Temperature;

pub struct AverageWeather {
  temperatures: Vec<Temperature>,
  max: i32, 
  min: i32
}

impl AverageWeather {

    pub fn new(temperatures: Vec<Temperature>) -> AverageWeather {
        AverageWeather {
            temperatures,
            max: 0,
            min: 99
        }
    }

    pub fn get_average(&self) -> i32 {
        let mut average: i32 = 0;
        for temperature in self.temperatures.iter() {
            average += temperature.get_temp();
        }
       return average / self.temperatures.len() as i32;
    }
    
    pub fn get_max_temperature() -> Temperature {
        Temperature {
            value: 0
        }
    }

    pub fn get_min_temperature() -> Temperature {
        Temperature {
            value: 0
        }
    }
}