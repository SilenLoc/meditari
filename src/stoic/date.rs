use lazy_static::lazy_static;
use std::time::{Duration, SystemTime};

lazy_static! {
    static ref TWO_HOURS: Duration = Duration::from_secs(3600 * 2);
}

#[derive(Debug, Clone)]
pub struct StoicDate {
    pub year: String,
    pub month: String,
    pub day: String,
    pub rest: String,
}

impl StoicDate {
    pub fn year(&self) -> String {
        self.year.clone()
    }

    pub fn month(&self) -> String {
        self.month.clone()
    }

    pub fn day(&self) -> String {
        self.day.clone()
    }
}

pub fn date() -> StoicDate {
    let now = SystemTime::now();
    let now = now.checked_add(*TWO_HOURS).unwrap();

    let readable = humantime::format_rfc3339(now);

    let x = readable.to_string();
    let split = x.split("T").collect::<Vec<&str>>();
    let rest = split[1].replace(".", "_");

    let year_month_day = split[0];
    let split = year_month_day.split("-").collect::<Vec<&str>>();
    let year = split[0];
    let month = split[1];
    let day = split[2];

    StoicDate {
        day: day.to_string(),
        month: month.to_string(),
        year: year.to_string(),
        rest: rest.to_string(),
    }
}

pub fn file_name(date: &StoicDate) -> String {
    format!("{}_{}_{}_{}.md", date.year, date.month, date.day, date.rest)
}

#[cfg(test)]
mod tests {
    use super::date;

    #[test]
    pub fn should_create_readable_time() {
        let _date = date();
        println!("Should be able to create date")
    }
}
