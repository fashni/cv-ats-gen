use std::fs;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Contact {
  pub email: String,
  pub phone: String,
  pub address: Option<String>,
  pub website: Option<String>,
  pub linkedin: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Education {
  pub degree: String,
  pub institution: String,
  pub year: String,
  pub gpa: Option<f32>,
  pub description: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct Experience {
  pub title: String,
  pub company: String,
  pub duration: String,
  pub description: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct Certification {
  pub title: String,
  pub issuer: String,
  pub year: String,
}

#[derive(Deserialize, Debug)]
pub struct Project {
  pub name: String,
  pub description: String,
  pub technologies: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct CV {
  pub name: String,
  pub title: Option<String>,
  pub contact: Option<Contact>,
  pub summary: Option<String>,
  pub education: Option<Vec<Education>>,
  pub experience: Option<Vec<Experience>>,
  pub projects:Option<Vec<Project>>,
  pub certifications: Option<Vec<Certification>>,
  pub skills: Option<Vec<String>>,
  pub languages: Option<Vec<String>>,
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
