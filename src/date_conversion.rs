use std::str::FromStr;

use chrono::NaiveDate;

#[repr(u32)]
enum DirtyMonth{
    Jan = 1,
    Feb = 2,
    Mar = 3,
    Apr = 4,
    May = 5,
    Jun = 6,
    Jul = 7,
    Aug = 8,
    Sep = 9,
    Oct = 10,
    Nov = 11,
    Dec = 12
}

impl FromStr for DirtyMonth {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Jan" => Ok(DirtyMonth::Jan),
            "Feb" => Ok(DirtyMonth::Feb),
            "Mar" => Ok(DirtyMonth::Mar),
            "Apr" => Ok(DirtyMonth::Apr),
            "May" => Ok(DirtyMonth::May),
            "Jun" => Ok(DirtyMonth::Jun),
            "Jul" => Ok(DirtyMonth::Jul),
            "Aug" => Ok(DirtyMonth::Aug),
            "Sep" => Ok(DirtyMonth::Sep),
            "Oct" => Ok(DirtyMonth::Oct),
            "Nov" => Ok(DirtyMonth::Nov),
            "Dec" => Ok(DirtyMonth::Dec),
            _ => Err("Invalid month")
        }
    }
}

impl Into<u32> for DirtyMonth {
    fn into(self) -> u32 {
        self as u32
    }
    
}
    

pub fn converted_date(date: &str) -> Option<(NaiveDate, NaiveDate)>{
    if check_second_part_date(date) {
        let first_part = first_part_date(date);
        let second_part = second_part_date(date).unwrap();
        converted_date_with_to(first_part, second_part)
    } else {
        converted_date_without_to(date)
    }
}

#[inline]
fn converted_date_with_to(first_part: &str, second_part: &str) -> Option<(NaiveDate, NaiveDate)> {
    let (month, day, year) = split_to_yr_mnt_date(first_part)?;
    let month = DirtyMonth::from_str(month).ok()?;
    let day = remove_suffix_comma(day).ok()?;
    let year = str_yr_to_i32(year).ok()?;
    let first_part_result = NaiveDate::from_ymd_opt(year, month.into(), day);
    let (month2, day2, year2) = split_to_yr_mnt_date(second_part)?;
    let month2 = DirtyMonth::from_str(month2).ok()?;
    let day2 = remove_suffix_comma(day2).ok()?;
    let year2 = str_yr_to_i32(year2).ok()?;
    let second_part_result = NaiveDate::from_ymd_opt(year2, month2.into(), day2);
    Some((first_part_result?, second_part_result?))

}
#[inline]
fn converted_date_without_to(first_part: &str) -> Option<(NaiveDate, NaiveDate)> {
    let (month, day, year) = split_to_yr_mnt_date(first_part)?;
    let month = DirtyMonth::from_str(month).ok()?;
    let day = remove_suffix_comma(day).ok()?;
    let year = str_yr_to_i32(year).ok()?;
    let first_part_result = NaiveDate::from_ymd_opt(year, month.into(), day);
    Some((first_part_result?, first_part_result?))
    
}
    
#[inline]
fn first_part_date(date: &str) -> &str {
    date.split("to").collect::<Vec<&str>>()[0].trim()
    
}

#[inline]
fn check_second_part_date(date: &str) -> bool {
    date.contains("to")
}
#[inline]
fn second_part_date(date: &str) -> Option<&str> {
    date.split("to").collect::<Vec<&str>>().get(1).map(|x| x.trim())
}

#[inline]
fn split_to_yr_mnt_date(date: &str) -> Option<(&str, &str, &str)> {
    if date.len() == 9 {
        let month = date.split_whitespace().next()?;
        let year = date.split_whitespace().last()?;
         Some((month, "", year))
    }else {
        
    let mut date = date.split_whitespace();
    let month = date.next()?;
    let day = date.next()?;
    let year = date.next()?;
    Some((month, day, year))
    }
}

#[inline]
fn remove_suffix_comma(day: &str) -> Result<u32, &'static str> {
    let day = day.trim_end_matches(',');
    u32::from_str(day).map_err(|_| "Invalid day")
}

fn str_yr_to_i32(yr: &str) -> Result<i32, &'static str> {
    i32::from_str(yr).map_err(|_| "Invalid year")
    
}





#[cfg(test)]
mod tests {

    use super::*;
    const DATE: &str = "Oct 4, 2015 to Mar 27, 2016";
    const DATE2: &str = "Oct 9, 2017";
    const DATE3: &str = "Mar, 2020";

    #[test]
    fn test_converted_date_with_to() {
        let date = converted_date(DATE);
        assert_eq!(date, Some((NaiveDate::from_ymd(2015, 10, 4), NaiveDate::from_ymd(2016, 3, 27))));
    }
    #[test]
    fn test_converted_date_without_to() {
        let date = converted_date(DATE2);
        assert_eq!(date, Some((NaiveDate::from_ymd(2017, 10, 9), NaiveDate::from_ymd(2017, 10, 9))));
    }

    #[test]
    fn test_remove_prefix() {
        let date = first_part_date(DATE);
        assert_eq!(date, "Oct 4, 2015");
    }
    
    #[test]
    fn test_check_second_part_date() {
        let date = check_second_part_date(DATE);
        assert_eq!(date, true);
    }

    #[test]
    fn test_second_part_date() {
        let date = second_part_date(DATE);
        assert_eq!(date, Some("Mar 27, 2016"));
    }

    #[test]
    fn test_split_to_yr_mnt_date() {
        let date = split_to_yr_mnt_date("Oct 4, 2015");
        assert_eq!(date, Some(("Oct", "4,", "2015")));
    }

    #[test]
    fn test_remove_suffix_comma() {
        let day = remove_suffix_comma("4,");
        assert_eq!(day, Ok(4));
    }

    #[test]
    fn test_str_yr_to_i32() {
        let yr = str_yr_to_i32("2015");
        assert_eq!(yr, Ok(2015));
    }

    #[test]
    fn test_split_to_yr_mnt_date_with_full_date() {
        let date = split_to_yr_mnt_date(DATE);
        assert_eq!(date, Some(("Oct", "4,", "2015")));
    }

    #[test]
    fn test_split_to_yr_mnt_date_with_month_year() {
        let date = split_to_yr_mnt_date(DATE3);
        assert_eq!(date, Some(("Mar,", "", "2020")));
    }
    
}


