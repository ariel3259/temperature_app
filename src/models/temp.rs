#[derive(Copy, Clone)]
pub struct Temperature {
  value: i32
}

impl Temperature {
    pub fn new(v: i32) -> Temperature {
        Temperature {
            value: v
        }
    }

    pub fn get_temp(&self) -> i32 {
        self.value
    }
}
