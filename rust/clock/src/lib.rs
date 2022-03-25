#[derive(Debug, PartialEq)]
pub struct Clock {
    pub hr :i32,
    pub min :i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let time = 60 * hours + minutes;
        if time.is_negative() {
            if time % 60 == 0 {
                Self { hr: 24 + (time / 60) % 24, min: time % 60 }
            } else {
                Self { hr: 23 + (time / 60) % 24, min: 60 + time % 60 }
            }
        } else {
            Self { hr: (time / 60) % 24, min: time % 60 }
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hr, self.min)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let time = 60 * self.hr + self.min + minutes;
        if time.is_negative() {
            if time % 60 == 0 {
                Self { hr: 24 + (time / 60) % 24, min: time % 60 }
            } else {
                Self { hr: 23 + (time / 60) % 24, min: 60 + time % 60 }
            }
        } else {
            Self { hr: (time / 60) % 24, min: time % 60 }
        }
    }
}
