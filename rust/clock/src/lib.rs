use std::fmt::{Debug, Display, Formatter};

pub struct Clock {
    hours: i32,
    minutes: i32,
}
impl Display for Clock {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
impl Debug for Clock {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        println!("HOURS : {}", hours);
        println!("MINUTES: {}", minutes);

        if hours < 24 && minutes <= 59 && hours >= 0 && minutes >= 0 {
            let clock = Clock {
                hours,
                minutes,
            };
            clock
        } else {
            if hours > 24 && minutes <= 59 && minutes >= 0 {
                let new_hour = hours - 24;
                Self::new(new_hour, minutes)
            } else if minutes < 0 {
                let positive_minutes = minutes * -1;
                if positive_minutes <= 59 && hours != 0 {
                    let hour_in_minutes = hours * 60;
                    let hour_minus_minutes = hour_in_minutes - positive_minutes;
                    let new_hour = hour_minus_minutes / 60;
                    let new_minutes = hour_minus_minutes % 60;
                    Self::new(new_hour, new_minutes)
                } else {
                    let total_minutes = hours * 60 + minutes;

                    let adjusted_minutes = ((total_minutes % 1440) + 1440) % 1440;


                    let new_hours = adjusted_minutes / 60;
                    let new_minutes = adjusted_minutes % 60;

                    let final_clock = Clock {
                        hours: new_hours,
                        minutes: new_minutes,
                    };
                    final_clock
                }


            } else if hours < 0 && minutes <= 59 && minutes >= 0 {
                let new_hour = 24 + hours;
                Self::new(new_hour, minutes)
            } else if minutes > 59 {
                let added_hour = minutes / 60;
                let added_minutes = minutes % 60;
                let new_hour = hours + added_hour;
                Self::new(new_hour, added_minutes)
            } else {
                return Clock {
                    hours: 0,
                    minutes,
                };
            }
        }
    }

    pub fn to_string(&self) -> String {
        if self.hours >= 0 && self.minutes >= 0 && self.minutes <= 59 {
            format!("{:02}:{:02}", self.hours, self.minutes)
        } else {
            let clock = Self::new(self.hours, self.minutes);
            format!("{:02}:{:02}", clock.hours, clock.minutes)
        }
    }


    pub fn add_minutes(&self, minutes: i32) -> Self {
      Clock{
          hours:self.hours,
          minutes:self.minutes + minutes
      }
    }

    

}
