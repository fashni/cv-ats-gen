use std::fmt;
use std::fs;
use std::str::FromStr;
use serde::de::{self, Deserialize, Deserializer};

const MONTH_NAMES_SHORT: [&str; 12] = [
  "Jan", "Feb", "Mar", "Apr", "May", "Jun",
  "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

const MONTH_NAMES_LONG: [&str; 12] = [
  "January", "February", "March", "April", "May", "June",
  "July", "August", "September", "October", "November", "December",
];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct YearMonth {
  pub year: i32,
  pub month: u8,
}

#[derive(Debug)]
pub enum ParseYearMonthError {
  InvalidFormat,
  InvalidYear,
  InvalidMonth,
}

impl std::error::Error for ParseYearMonthError {}

impl fmt::Display for ParseYearMonthError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      ParseYearMonthError::InvalidFormat => write!(f, "invalid format (expected YYYY-MM)"),
      ParseYearMonthError::InvalidYear => write!(f, "invalid year"),
      ParseYearMonthError::InvalidMonth => write!(f, "invalid month"),
    }
  }
}

impl FromStr for YearMonth {
  type Err = ParseYearMonthError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let (year_str, month_str) = s
      .split_once('-')
      .ok_or(ParseYearMonthError::InvalidFormat)?;

    let year: i32 = year_str.parse()
      .map_err(|_| ParseYearMonthError::InvalidYear)?;
    let month: u8 = month_str.parse()
      .map_err(|_| ParseYearMonthError::InvalidMonth)?;

    if !(1..=12).contains(&month) {
      return Err(ParseYearMonthError::InvalidMonth);
    }

    Ok(YearMonth { year, month })
  }
}

impl fmt::Display for YearMonth {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:04}-{:02}", self.year, self.month)
  }
}

impl<'de> Deserialize<'de> for YearMonth {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'de> {
    {
      let s = String::deserialize(deserializer)?;
      YearMonth::from_str(&s).map_err(de::Error::custom)
    }
  }
}

impl YearMonth {
  pub fn format_short_name(&self) -> String {
    format!("{} {}", MONTH_NAMES_SHORT[(self.month - 1) as usize], self.year)
  }
  
  pub fn format_long_name(&self) -> String {
    format!("{} {}", MONTH_NAMES_LONG[(self.month - 1) as usize], self.year)
  }
}

#[derive(serde::Deserialize, Debug)]
pub struct Contact {
  pub email: String,
  pub phone: String,
  pub location: Option<String>,
  pub website: Option<String>,
  pub linkedin: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Education {
  pub degree: String,
  pub institution: String,
  pub graduation_year: String,
  pub gpa: Option<f32>,
  pub max_gpa: Option<f32>,
  pub description: Option<Vec<String>>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Experience {
  pub title: String,
  pub company: String,
  pub start_date: YearMonth,
  pub end_date: YearMonth,
  pub description: Option<Vec<String>>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Certification {
  pub title: String,
  pub issuer: String,
  pub year: String,
  pub url: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Project {
  pub name: String,
  pub url: Option<String>,
  pub description: String,
  pub tools: Vec<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Skill {
  pub category: String,
  pub skills: Vec<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Language {
  pub name: String,
  pub fluency: Option<String>
}

#[derive(serde::Deserialize, Debug)]
pub struct CV {
  pub name: String,
  pub title: Option<String>,
  pub contact: Option<Contact>,
  pub summary: Option<String>,
  pub education: Option<Vec<Education>>,
  pub experience: Option<Vec<Experience>>,
  pub projects:Option<Vec<Project>>,
  pub certifications: Option<Vec<Certification>>,
  pub skills: Option<Vec<Skill>>,
  pub languages: Option<Vec<Language>>,
}

impl CV {
  pub fn from_yaml(file_path: &str) -> Result<Option<Self>, serde_yaml::Error> {
    match fs::read_to_string(file_path) {
      Ok(yaml_str) => {
        match serde_yaml::from_str::<Self>(&yaml_str) {
          Ok(cv) => Ok(Some(cv)),
          Err(e) => Err(e),
        }
      },
      Err(_) => Ok(None),
    }
  }
}
