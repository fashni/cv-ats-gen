use maud::{html, Markup, DOCTYPE};
use crate::sections::CV;

pub fn render_cv(cv: &CV) -> Markup {
  html! {
    (DOCTYPE)
    html lang="en" {
      head {
        meta charset="utf-8";
        meta name="viewport" content="width=device-width,initial-scale=1";
        title { (cv.name) " - CV" }
        link rel="stylesheet" type="text/css" href="/static/style.css" {}
      }
      body id="wrapper" {
        header {
          h1 id="name" { (cv.name) }
          @if let Some(title) = &cv.title{
            h2 id="title" { (title) }
          }
          @if let Some(contact) = &cv.contact {
            div id="contact" {
              p {
                a href={ "mailto:" (contact.email) } { (contact.email) }
                " • " a href={ "tel:" (contact.phone) } { (contact.phone) }
                @if let Some(linkedin) = &contact.linkedin {
                  " • " a href=(linkedin) target="_blank" { (linkedin.replace("https://", "")) }
                }
                @if let Some(website) = &contact.website {
                  " • " a href=(website) target="_blank" { (website.replace("https://", "")) }
                }
              }
              @if let Some(address) = &contact.address {
                p id="address" { (address) }
              }
            }
          }
        }
        
        @if let Some(summary) = &cv.summary {
          section {
            p id="summary" { (summary) }
          }
        }
        
        @if let Some(education) = &cv.education {
          section {
            h2 { "Education" }
            div.items {
              @for edu in education {
                div.item {
                  p.item-text {
                    span { b { (edu.degree) } " - " (edu.institution) }
                    span.time { (edu.year) }
                    @if let Some(gpa) = &edu.gpa {
                      br;
                      span.gpa { "GPA: " (format!("{:.2}", gpa)) }
                    }
                  }
                  @if let Some(description) = &edu.description {
                    ul.description {
                      @for desc in description {
                        li { (desc) }
                      }
                    }
                  }
                }
              }
            }
          }
        }
        
        @if let Some(experience) = &cv.experience {
          section {
            h2 { "Experience" }
            div.items {
              @for exp in experience {
                div.item {
                  p.item-text {
                    span { b { (exp.title) } " - " (exp.company) }
                    span.time { (exp.duration) }
                  }
                  @if let Some(description) = &exp.description {
                    ul.description {
                      @for desc in description {
                        li { (desc) }
                      }
                    }
                  }
                }
              }
            }
          }
        }
        
        @if let Some(certs) = &cv.certifications {
          section {
            h2 { "Certifications" }
            div.items {
              @for cert in certs {
                div.item {
                  p.item-text {
                    span { b {(cert.title)} " - " (cert.issuer) } 
                    span.time { (cert.year) }
                  }
                }
              }
            }
          }
        }
        
        @if let Some(skills) = &cv.skills {
          section {
            h2 { "Skills" }
            ul.list-items {
              @for skill in skills {
                li {
                  (skill)
                }
              }
            }
          }
        }
        
        @if let Some(languages) = &cv.languages {
          section {
            h2 { "Languages" }
            ul.list-items {
              @for language in languages {
                li {
                  (language)
                }
              }
            }
          }
        }
        
        footer {
          p {
            "© 2024 " (cv.name)
            @if let Some(contact) = &cv.contact {
              @if let Some(website) = &contact.website {
                " • " a href=(website) target="_blank" { (website.replace("https://", "")) }
              }
            }
          }
        }
      }
    }
  }
}
