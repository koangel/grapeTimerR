pub mod parsers {
    use std::{time,fmt};
    use chrono::{Local, DateTime, FixedOffset, ParseError, NaiveTime, NaiveDate, Datelike, TimeZone, Timelike, Utc, Duration};
    use crate::errors::errors::{TError, TErrorKind, TResult};
    use std::ops::Add;

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

        pub fn parser(&mut self, date_format:&str) -> TResult<()> {
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

    fn next_date_time(year:i32, month:u32, day:u32, time_format:&str) -> TResult<DateTime<Local>> {
        let vt = NaiveTime::parse_from_str(time_format, "%T");
        match vt {
            Ok(time_tt) => {
                let ndt = Local.ymd(year, month,day )
                    .and_hms(time_tt.hour(),time_tt.minute(),time_tt.second());
                Ok(ndt)
            }
            Err(ve) => {
                Err(TError::new(TErrorKind::Other(ve.to_string())))
            }
        }
    }

    fn next_date_timeUtc(year:i32, month:u32, day:u32, time_format:&str) -> TResult<DateTime<Utc>> {
        let vt = NaiveTime::parse_from_str(time_format, "%T");
        match vt {
            Ok(time_tt) => {
                let ndt = Utc.ymd(year, month,day )
                    .and_hms(time_tt.hour(),time_tt.minute(),time_tt.second());
                Ok(ndt)
            }
            Err(ve) => {
                Err(TError::new(TErrorKind::Other(ve.to_string())))
            }
        }
    }

    // time 00:00:00 get now time
    fn atNowTime(time_format:&str) -> TResult<DateTime<Local>> {
        let now_time = Local::now();
        let vt = NaiveTime::parse_from_str(time_format, "%T");
        match vt {
            Ok(time_tt) => {
                let ndt = Local.ymd(now_time.year(), now_time.month(), now_time.day())
                    .and_hms(time_tt.hour(),time_tt.minute(),time_tt.second());
                Ok(ndt)
            }
            Err(ve) => {
                Err(TError::new(TErrorKind::Other(ve.to_string())))
            }
        }
    }

    fn atUtcNowTime(time_format:&str) -> TResult<DateTime<Utc>> {
        let now_time = Utc::now();
        let vt = NaiveTime::parse_from_str(time_format, "%T");
        match vt {
            Ok(time_tt) => {
                let ndt = Utc.ymd(now_time.year(), now_time.month(), now_time.day())
                    .and_hms(time_tt.hour(),time_tt.minute(),time_tt.second());
                Ok(ndt)
            }
            Err(ve) => {
                Err(TError::new(TErrorKind::Other(ve.to_string())))
            }
        }
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
    pub fn parser_next(timeStr:&str) -> TResult<chrono::DateTime<Local>> {
        let mut date_pv = DateParser::new();
        date_pv.parser(timeStr)?; //分析分析数据

        let nowTime = Local::now();

        if "day" == date_pv.action {
            let mut atTime = atNowTime(&date_pv.clock)?;
            if nowTime.timestamp() >= atTime.timestamp() {
                atTime = atTime + Duration::days(1);
            }
            return Ok(atTime);
        }else if "week" == date_pv.action {
            let mut atTime = atNowTime(&date_pv.clock)?;
            if date_pv.day_time >= 7 || date_pv.day_time < 0 {
               return Err(TError::new(TErrorKind::WeekDay));
            }

            let weekDayNow = nowTime.weekday().num_days_from_sunday() as i8;
            let mut weekOffset =  date_pv.day_time - weekDayNow;
            if weekOffset < 0 {
                weekOffset += 7;
            }

            if weekOffset != 0 ||nowTime.timestamp() >= atTime.timestamp() {
                atTime = atTime +Duration::days(weekOffset as i64);
            }

            return Ok(atTime);
        }else if "month" == date_pv.action {
            let maxDay = getMonthDay(nowTime.year(),nowTime.month() as i32).unwrap();
            if date_pv.day_time as i32 > maxDay {
                return Err(TError::new(TErrorKind::DateOverflow));
            }

            // 主动计算下一个日期
            let mut nextTime = next_date_time(nowTime.year(),
                                              nowTime.month(),date_pv.day_time as u32,
                                              &date_pv.clock)?;

            if nowTime.timestamp() >= nextTime.timestamp() {
                nextTime = next_date_time(nowTime.year(),
                                          nowTime.month()+1,date_pv.day_time as u32,
                                          &date_pv.clock)?;
            }

            return Ok(nextTime);
        }else {
            return Err(TError::new(TErrorKind::BadFormat));
        }

        Err(TError::new(TErrorKind::BadFormat))
    }

    /// parser date format [通过一个字符串分析出下一次的运行时间戳]
    /// tick time is sec,unix timestamp [时间为秒，unix时间戳]
    /// support data:
    /// Day 05:00:00  every day at 05::00::00 tick [每日几点]
    /// Week 1 05:00:00 What time of the week, week flag 0~6 [每周几的几点]
    /// Month 1 05:00:00 What time of the Month,month flag 1~31 [每月几日几点，不足跳过]
    /// Skip date if there is no such date in this month [该月如果没这个日期，则跳过该月]
    /// # Examples
    ///
    /// ```
    /// use grapeTimerR::parsers;
    /// let next_day = parsers::parser_timestamp("Day 05::00:00").unwrap();
    /// let next_day2 = parsers::parser_timestamp("Week 1 05:00:00").unwrap();
    /// ```
    pub fn parser_timestamp(timeStr:&str) -> TResult<i64> {
        let date_now = parser_next(timeStr)?;
        Ok(date_now.timestamp())
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
    /// let next_day = parsers::parser_nextUtc("Day 05::00:00").unwrap();
    /// let next_day2 = parsers::parser_nextUtc("Week 1 05:00:00").unwrap();
    /// ```
    pub fn parser_nextUtc(timeStr:&str) -> TResult<chrono::DateTime<Utc>> {
        let mut date_pv = DateParser::new();
        date_pv.parser(timeStr)?; //分析分析数据

        let nowTime = Utc::now();

        if "day" == date_pv.action {
            let mut atTime = atUtcNowTime(&date_pv.clock)?;
            if nowTime.timestamp() >= atTime.timestamp() {
                atTime = atTime + Duration::days(1);
            }
            return Ok(atTime);
        }else if "week" == date_pv.action {
            let mut atTime = atUtcNowTime(&date_pv.clock)?;
            if date_pv.day_time >= 7 || date_pv.day_time < 0 {
                return Err(TError::new(TErrorKind::WeekDay));
            }

            let weekDayNow = nowTime.weekday().num_days_from_sunday() as i8;
            let mut weekOffset =  date_pv.day_time - weekDayNow;
            if weekOffset < 0 {
                weekOffset += 7;
            }

            if weekOffset != 0 ||nowTime.timestamp() >= atTime.timestamp() {
                atTime = atTime +Duration::days(weekOffset as i64);
            }

            return Ok(atTime);
        }else if "month" == date_pv.action {
            let maxDay = getMonthDay(nowTime.year(),nowTime.month() as i32).unwrap();
            if date_pv.day_time as i32 > maxDay {
                return Err(TError::new(TErrorKind::DateOverflow));
            }

            // 主动计算下一个日期
            let mut nextTime = next_date_timeUtc(nowTime.year(),
                                              nowTime.month(),date_pv.day_time as u32,
                                              &date_pv.clock)?;

            if nowTime.timestamp() >= nextTime.timestamp() {
                nextTime = next_date_timeUtc(nowTime.year(),
                                          nowTime.month()+1,date_pv.day_time as u32,
                                          &date_pv.clock)?;
            }

            return Ok(nextTime);
        }else {
            return Err(TError::new(TErrorKind::BadFormat));
        }

        Err(TError::new(TErrorKind::BadFormat))
    }

    /// parser date format [通过一个字符串分析出下一次的运行时间戳]
    /// tick time is sec,unix timestamp [时间为秒，unix时间戳]
    /// support data:
    /// Day 05:00:00  every day at 05::00::00 tick [每日几点]
    /// Week 1 05:00:00 What time of the week, week flag 0~6 [每周几的几点]
    /// Month 1 05:00:00 What time of the Month,month flag 1~31 [每月几日几点，不足跳过]
    /// Skip date if there is no such date in this month [该月如果没这个日期，则跳过该月]
    /// # Examples
    ///
    /// ```
    /// use grapeTimerR::parsers;
    /// let next_day = parsers::parser_timestampUtc("Day 05::00:00").unwrap();
    /// let next_day2 = parsers::parser_timestampUtc("Week 1 05:00:00").unwrap();
    /// ```
    pub fn parser_timestampUtc(timeStr:&str) -> TResult<i64> {
        let date_now = parser_nextUtc(timeStr)?;
        Ok(date_now.timestamp())
    }

    // 内部函数测试
    #[test]
    fn test_parser_date() {
        let mut datep = DateParser::new();
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::parsers::{getMonthDay, parser_next, parser_timestamp, parser_nextUtc, parser_timestampUtc};
    use chrono::Local;

    #[test]
    fn test_parser_next() {
        let next_date = parser_next("Day 05:00:00").unwrap();
        let next_tick = parser_timestamp("Day 05:00:00").unwrap();

        let next_weekDAY = parser_next("WEEK 2 06:00:00").unwrap();
        let next_weekTick = parser_timestamp("WEEK 2 06:00:00").unwrap();

        let next_monthDAY = parser_next("Month 2 06:00:00").unwrap();
        let next_monthTick = parser_timestamp("Month 2 06:00:00").unwrap();

        println!("day {} -  unix:{}",next_date,next_tick);
        println!("week {} -  unix:{}",next_weekDAY,next_weekTick);
        println!("month {} -  unix:{}",next_monthDAY,next_monthTick);
    }

    #[test]
    fn test_parser_utc() {
        let next_date = parser_nextUtc("Day 05:00:00").unwrap();
        let next_tick = parser_timestampUtc("Day 05:00:00").unwrap();

        let next_weekDAY = parser_nextUtc("WEEK 2 06:00:00").unwrap();
        let next_weekTick = parser_timestampUtc("WEEK 2 06:00:00").unwrap();

        let next_monthDAY = parser_nextUtc("Month 2 06:00:00").unwrap();
        let next_monthTick = parser_timestampUtc("Month 2 06:00:00").unwrap();

        println!("utc day {} -  unix:{}",next_date,next_tick);
        println!("utc week {} -  unix:{}",next_weekDAY,next_weekTick);
        println!("utc month {} -  unix:{}",next_monthDAY,next_monthTick);
    }
}