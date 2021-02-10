pub mod parsers {
    use std::{time,fmt};
    use chrono::{Local, DateTime, FixedOffset, ParseError, NaiveTime, NaiveDate, Datelike, TimeZone, Timelike};
    use crate::errors::errors::{TError, TErrorKind};

    pub struct DateParser {
        pub action:String, // 具体类型
        pub clock:String, // 时间日期
        pub day_time:i8, // week = 0~6,month = 1~31
    }

    impl DateParser {
        pub fn new() -> DateParser {
            DateParser{
                action: String::from(""),
                clock: String::from(""),
                day_time: 0
            }
        }

        pub fn parser(&mut self, date_format:&str) -> Result<(),TError> {
            let mut date_split = date_format.split(" ").collect::<Vec<&str>>();
            if date_split.len() < 2 {
                return Err(TError::new(TErrorKind::BadFormat)); // 格式错误
            }

            self.action = String::from(date_split[0].to_lowercase());
            match self.action.as_str() {
                "day" => {
                    self.clock = String::from(date_split[1])
                }
                "week" | "month"  => {
                    if date_split.len() < 3 {
                        return Err(TError::new(TErrorKind::BadFormat)); // 格式错误
                    }

                    self.clock = String::from(date_split[2]);
                    let rv = date_split[1].parse();
                    match rv {
                        Ok(value) => { self.day_time = value }
                        Err(e) => { return Err(TError::new(TErrorKind::Other(e.to_string().clone()))) }  // 格式错误
                    }
                }
                _ => {}
            }
            Ok(())
        }
    }

    // get month days
    pub fn getMonthDay(year:i32,month:i32) -> Option<i32> {
        let m_wday = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        if month == 2 && (year%4) == 0 && ( year % 100 != 0 || year % 400 == 0) {
            Some(29)
        }else {
            let index = (month - 1) as usize;
           let r = m_wday.get(index);
            match r {
                Some(v) => { Some(*v) }
                None => None
            }
        }
    }

    // time 00:00:00 get now time
    pub fn atNowTime(time_format:&str ) -> Result<DateTime<Local>,TError> {
        let now_time = Local::now();
        let time_tt = NaiveTime::parse_from_str(time_format, "%T").unwrap();
        let ndt = Local.ymd(now_time.year(), now_time.month(), now_time.day())
            .and_hms(time_tt.hour(),time_tt.minute(),time_tt.second());
        Ok(ndt)
    }

    /// parser date format [通过一个字符串分析出下一次的运行时间，或间隔TICK]
    ///
    /// support data:
    /// Day 05:00:00  every day at 05::00::00 tick [每日几点]
    /// Week 1 05:00:00 What time of the week, week flag 0~6 [每周几的几点]
    /// Month 1 05:00:00 What time of the Month,month flag 1~31 [每月几日几点，不足跳过]
    /// Skip date if there is no such date in this month [该月如果没这个日期，则跳过该月]
    /// # Examples
    ///
    /// ```
    /// use grapeTimerR::parsers;
    /// let next_day = parsers::parser_next("Day 05::00:00").unwrap();
    /// let next_day2 = parsers::parser_next("Week 1 05:00:00").unwrap();
    /// ```
    pub fn parser_next(timeStr:&str) -> Result<chrono::DateTime<Local>,TError> {
        let mut date_pv = DateParser::new();
        date_pv.parser(timeStr)?; //分析分析数据

        Err(TError::new(TErrorKind::BadFormat))
    }

    /// parser date format [通过一个字符串分析出下一次的运行时间，或间隔TICK]
    /// tick time is sec [时间为秒]
    /// support data:
    /// Day 05:00:00  every day at 05::00::00 tick [每日几点]
    /// Week 1 05:00:00 What time of the week, week flag 0~6 [每周几的几点]
    /// Month 1 05:00:00 What time of the Month,month flag 1~31 [每月几日几点，不足跳过]
    /// Skip date if there is no such date in this month [该月如果没这个日期，则跳过该月]
    /// # Examples
    ///
    /// ```
    /// use grapeTimerR::parsers;
    /// let next_day = parsers::parser_tick("Day 05::00:00").unwrap();
    /// let next_day2 = parsers::parser_tick("Week 1 05:00:00").unwrap();
    /// ```
    pub fn parser_tick(timeStr:&str) -> Result<time::Duration,TError> {
        let mut date_pv = DateParser::new();
        date_pv.parser(timeStr)?; //分析分析数据

        Err(TError::new(TErrorKind::BadFormat))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::parsers::{getMonthDay, atNowTime};

    #[test]
    fn test_parser_date() {
        let mut datep = parsers::DateParser::new();
        datep.parser("Day 00:00:00");
        assert_eq!(datep.action,"day");
        assert_eq!(datep.clock,"00:00:00");

        datep.parser("Week 1 00:00:00");
        assert_eq!(datep.action,"week");
        assert_eq!(datep.day_time,1);
        assert_eq!(datep.clock,"00:00:00");
    }

    #[test]
    fn test_utils() {
        let gDay = getMonthDay(2009,1);
        let timeFtm = atNowTime("05:00:00").unwrap();

        println!("day:{}",gDay.unwrap());
        println!("fmt:{}",timeFtm.format("%Y-%m-%d %H:%M:%S"));
        println!("unix time:{}",timeFtm.timestamp())
    }

    #[test]
    fn test_parser_next() {

    }

    #[test]
    fn test_parser_tick() {

    }
}