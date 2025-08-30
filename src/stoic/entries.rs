use std::collections::HashMap;
use std::fs;
use chrono::{Datelike, Utc};

use crate::stoic::init::stoic_shell;
use crate::stoic::date::StoicDate;


pub type Years = HashMap<i32, Months>;
pub type Months = HashMap<i32, Vec<Days>>;
pub type Days = HashMap<i32, Vec<Entry>>;

#[derive(Debug, Clone)]
pub struct Entry {
    pub stoic_date: StoicDate,
    pub date: chrono::NaiveDate,
    pub filename: String,
    pub content: String,
    pub path: String,
}

impl Entry{
    pub fn stoic_date(&self) -> &StoicDate {
        &self.stoic_date
    }
    pub fn date(&self) -> &chrono::NaiveDate {
        &self.date
    }
    pub fn filename(&self) -> &str {
        &self.filename
    }
    pub fn content(&self) -> &str {
        &self.content
    }
    pub fn path(&self) -> &str {
        &self.path
    }
    
}

pub struct EntryCollection(Vec<Entry>);

impl EntryCollection {
    pub fn new(entries: Vec<Entry>) -> Self {
        EntryCollection(entries)
    }

    pub fn entries(&self) -> &Vec<Entry> {
        &self.0
    }
    
    
    
}


pub fn collect_all_entries(owner: String) -> EntryCollection {
    let stoic_shell = stoic_shell(owner);
    let mut all_entries = Vec::new();

    let base_path = stoic_shell.current_dir();

    if let Ok(years) = fs::read_dir(&base_path) {
        for year_entry in years.flatten() {
            if year_entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                if let Some(year_name) = year_entry.file_name().to_str() {
                    if year_name.parse::<u32>().is_ok() {
                        let year_path = year_entry.path();

                        if let Ok(months) = fs::read_dir(&year_path) {
                            for month_entry in months.flatten() {
                                if month_entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                                    if let Some(month_name) = month_entry.file_name().to_str() {
                                        if month_name.parse::<u32>().is_ok() {
                                            let month_path = month_entry.path();

                                            if let Ok(days) = fs::read_dir(&month_path) {
                                                for day_entry in days.flatten() {
                                                    if day_entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                                                        if let Some(day_name) = day_entry.file_name().to_str() {
                                                            if day_name.parse::<u32>().is_ok() {
                                                                let day_path = day_entry.path();

                                                                let stoic_date = StoicDate {
                                                                    year: year_name.to_string(),
                                                                    month: month_name.to_string(),
                                                                    day: day_name.to_string(),
                                                                    rest: String::new(),
                                                                };

                                                                if let Ok(files) = fs::read_dir(&day_path) {
                                                                    for file_entry in files.flatten() {
                                                                        if let Some(filename) = file_entry.file_name().to_str() {
                                                                            if filename.ends_with(".md") {
                                                                                let file_path = file_entry.path();
                                                                                if let Ok(content) = fs::read_to_string(&file_path) {
                                                                                    all_entries.push(Entry {
                                                                                        stoic_date: stoic_date.clone(),
                                                                                        filename: filename.to_string(),
                                                                                        content,
                                                                                        path: file_path.to_string_lossy().to_string(),
                                                                                        date: stoic_date.as_system_time(),
                                                                                    });
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    all_entries.sort_by(|a, b| {a.date.cmp(&b.date)});

    EntryCollection::new(all_entries)
}

