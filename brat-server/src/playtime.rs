use chrono::{Datelike, Timelike};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DayDefinition {
    pub day: u8,
    pub start_time_hour: u8,
    pub start_time_minutes: u8,
    pub end_time_hour: u8,
    pub end_time_minutes: u8,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PlayTime {
    pub days: Vec<DayDefinition>,
}

impl PlayTime {
    pub fn new() -> Self {
        PlayTime { days: Vec::new() }
    }

    /// Add a day to the play time definition.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use whip::playtime::PlayTime;
    ///
    /// let mut play_time = PlayTime::new();
    /// play_time.add_day(1, 8, 30, 17, 45);
    /// ```
    ///
    /// # Arguments
    ///
    /// * `day` - The day of the week (0 = Sunday, 1 = Monday, etc.)
    /// * `start_time_hour` - The start time in hours (0-23)
    /// * `start_time_minute` - The start time in minutes (0-59)
    /// * `end_time_hour` - The end time in hours (0-23)
    /// * `end_time_minute` - The end time in minutes (0-59)
    pub fn add_day(
        &mut self,
        day: u8,
        start_time_hour: u8,
        start_time_minutes: u8,
        end_time_hour: u8,
        end_time_minutes: u8,
    ) {
        self.days.push(DayDefinition {
            day,
            start_time_hour,
            start_time_minutes,
            end_time_hour,
            end_time_minutes,
        });
    }

    #[allow(dead_code)]
    pub fn remove_day(&mut self, day: u8) {
        self.days.retain(|d| d.day != day);
    }

    pub fn is_play_time(&self) -> bool {
        let now = chrono::Local::now();
        let day = now.weekday().num_days_from_sunday() as u8;
        let hour = now.hour() as u8;
        let minute = now.minute() as u8;

        for day_definition in &self.days {
            if day_definition.day == day
                && day_definition.start_time_hour <= hour
                && day_definition.end_time_hour >= hour
            {
                if day_definition.start_time_hour == hour {
                    if day_definition.start_time_minutes < minute {
                        break;
                    }
                } else if day_definition.end_time_hour == hour {
                    if day_definition.end_time_minutes >= minute {
                        return true;
                    }
                } else {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_day() {
        let mut play_time = PlayTime::new();
        play_time.add_day(1, 8, 0, 17, 0);
        assert_eq!(play_time.days.len(), 1);
        assert_eq!(play_time.days[0].day, 1);
        assert_eq!(play_time.days[0].start_time_hour, 8);
        assert_eq!(play_time.days[0].end_time_hour, 17);
    }

    #[test]
    fn test_remove_day() {
        let mut play_time = PlayTime::new();
        play_time.add_day(0, 8, 0, 17, 0);
        play_time.add_day(1, 8, 0, 17, 0);
        play_time.add_day(2, 8, 0, 17, 0);
        play_time.add_day(3, 8, 0, 17, 0);
        play_time.add_day(4, 8, 0, 17, 0);
        play_time.add_day(5, 8, 0, 17, 0);
        play_time.add_day(6, 8, 0, 17, 0);
        assert_eq!(play_time.days.len(), 7);
        play_time.remove_day(0);
        assert_eq!(play_time.days.len(), 6);
        play_time.remove_day(1);
        assert_eq!(play_time.days.len(), 5);
        play_time.remove_day(2);
        assert_eq!(play_time.days.len(), 4);
        play_time.remove_day(3);
        assert_eq!(play_time.days.len(), 3);
        play_time.remove_day(4);
        assert_eq!(play_time.days.len(), 2);
        play_time.remove_day(5);
        assert_eq!(play_time.days.len(), 1);
        play_time.remove_day(6);
        assert_eq!(play_time.days.len(), 0);
    }

    #[test]
    fn test_is_play_time() {
        let mut play_time = PlayTime::new();
        play_time.add_day(0, 8, 0, 17, 21);
        play_time.add_day(1, 8, 0, 17, 0);
        play_time.add_day(2, 8, 0, 17, 0);
        play_time.add_day(3, 8, 0, 17, 0);
        play_time.add_day(4, 8, 0, 17, 0);
        play_time.add_day(5, 8, 0, 17, 0);
        play_time.add_day(6, 8, 0, 17, 0);

        let day = 0;
        let hour = 17;
        let minute = 20;
        let mut test = false;

        for day_definition in &play_time.days {
            if day_definition.day == day
                && day_definition.start_time_hour <= hour
                && day_definition.end_time_hour >= hour
            {
                if day_definition.start_time_hour == hour {
                    if day_definition.start_time_minutes < minute {
                        break;
                    }
                } else if day_definition.end_time_hour == hour {
                    if day_definition.end_time_minutes >= minute {
                        test = true;
                        break;
                    }
                } else {
                    test = true;
                    break;
                }
            }
        }

        assert!(test);

        let hour = 18;
        test = false;

        for day_definition in &play_time.days {
            if day_definition.day == day
                && day_definition.start_time_hour <= hour
                && day_definition.end_time_hour >= hour
            {
                if day_definition.start_time_hour == hour {
                    if day_definition.start_time_minutes < minute {
                        break;
                    }
                } else if day_definition.end_time_hour == hour {
                    if day_definition.end_time_minutes >= minute {
                        test = true;
                        break;
                    }
                } else {
                    test = true;
                    break;
                }
            }
        }

        assert!(!test)
    }
}
