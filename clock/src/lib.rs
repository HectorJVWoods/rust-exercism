#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}


impl Clock {
    fn normalize(&mut self) -> Clock {
        while self.minutes < 0 {
            self.hours -= 1;
            self.minutes += 60;
        }
        while self.hours < 0 {
            self.hours += 24;
        }
        self.hours += self.minutes / 60;
        self.minutes %= 60;
        self.hours %= 24;
        Clock {
            hours: self.hours,
            minutes: self.minutes,
        }
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hours,
            minutes,
        }.normalize()
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            hours: self.hours,
            minutes: self.minutes + minutes,
        }.normalize()
    }
    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }
}
