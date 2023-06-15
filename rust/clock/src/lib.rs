#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

fn pad_zero(num: i32) -> String {
    match num {
        x if x >= 0 && x <= 9 => {
            format!("0{num}")
        }
        x if x < 0 && x >= -9 => {
            format!("-0{}", x.abs())
        }
        _ => num.to_string(),
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { hours, minutes }
    }

    pub fn add_minutes(mut self, minutes: i32) -> Self {
        self.minutes += minutes;

        self
    }

    pub fn to_string(&self) -> String {
        let mut hours = self.hours;

        let minutes = {
            if self.minutes >= 0 {
                let remainder = self.minutes % 60;

                let quotant = self.minutes / 60;

                hours += quotant;

                pad_zero(remainder)
            } else {
                let remainder = self.minutes % 60;
                let quotant = (-(self.minutes / 60)) + 1;

                hours -= quotant;

                let mut result = 60 - -remainder;

                if result == 60 {
                    hours += 1;

                    result = 0;
                }

                pad_zero(result)
            }
        };

        let final_hours = {
            let temp_hours = if hours >= 0 {
                hours
            } else {
                24 - -(hours % 24)
            };

            let result = temp_hours % 24;

            pad_zero(result)
        };

        format!("{final_hours}:{minutes}")
    }
}
