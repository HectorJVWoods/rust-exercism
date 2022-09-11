

#[derive(Debug, PartialEq)]
pub struct Clock(i32);


impl Clock {
    pub fn normalize(&mut self) -> Clock {
        while self.0 < 0 {
            self.0 += 24 * 60;
        }
        self.0 %= 24 * 60;
        Clock(self.0)
    }
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            0: hours * 60 + minutes,
        }.normalize()
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            0: self.0 + minutes,
        }.normalize()
    }
    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.0 / 60, self.0 % 60)
    }
}
