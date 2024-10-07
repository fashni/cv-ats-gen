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
      body {
        header {
          h1 #name { (cv.name) }
          @if let Some(title) = &cv.title{
            h2 #title { (title) }
          }
          @if let Some(contact) = &cv.contact {
            div #contact {
              p .contact-links {
                a href={ "mailto:" (contact.email) } { (contact.email) }
                " • " a href={ "tel:" (contact.phone.replace(" ", "").replace("-", "")) } { (contact.phone) }
                @if let Some(linkedin) = &contact.linkedin {
                  " • " a href=(linkedin) target="_blank" { (linkedin.replace("https://", "").replace("http://", "")) }
                }
                @if let Some(website) = &contact.website {
                  " • " a href=(website) target="_blank" { (website.replace("https://", "").replace("http://", "")) }
                }
              }
              @if let Some(address) = &contact.address {
                p #address { (address) }
              }
            }
          }
        }

        @if let Some(summary) = &cv.summary {
          hr;
          section #summary {
            p { (summary) }
          }
        }

        @if let Some(education) = &cv.education {
          hr;
          section #education {
            h2.section-header { "Education" }
            div.items {
              @for edu in education {
                div.item {
                  p.item-text {
                    span { b { (edu.degree) } " - " (edu.institution) }
                    span.time { (edu.year) }
                  }
                  @if let Some(gpa) = &edu.gpa {
                    p.gpa { "GPA: " (format!("{:.2}", gpa)) }
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
          hr;
          section #experience {
            h2.section-header { "Experience" }
            div.items {
              @for exp in experience {
                div.item {
                  p.item-text {
                    span { b { (exp.title) } }
                    span.time { (exp.duration) }
                    br;
                    span.company { (exp.company) }
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

        @if let Some(projects) = &cv.projects {
          hr;
          section #projects {
            h2.section-header { "Projects" }
            div.items.grid {
              @for project in projects {
                div.item {
                  p.item-text {
                    @if let Some(url) = &project.url {
                      a href=(url) target="_blank" {
                        b {(project.name)}
                      }
                    } @else {
                      span { b {(project.name)} }
                    }
                    " - "
                    span.tools { (project.tools.join(", ")) }
                  }
                  p.description { (project.description) }
                }
              }
            }
          }
        }

        @if let Some(certs) = &cv.certifications {
          hr;
          section #certifications {
            h2.section-header { "Certifications" }
            div.items {
              @for cert in certs {
                div.item {
                  p.item-text {
                    @if let Some(url) = &cert.url {
                      a href=(url) target="_blank" {
                        b {(cert.title)} 
                      }
                    } @else {
                      span { b {(cert.title)} }
                    }
                    span.time { (cert.year) }
                    br;
                    span.issuer { (cert.issuer) }
                  }
                }
              }
            }
          }
        }

        @if let Some(skills) = &cv.skills {
          hr;
          section #skills {
            h2.section-header { "Skills" }
            ul.items.grid {
              @for skill in skills {
                li {
                  (skill)
                }
              }
            }
          }
        }

        @if let Some(languages) = &cv.languages {
          hr;
          section #languages {
            h2.section-header { "Languages" }
            ul.items.grid {
              @for language in languages {
                li {
                  (language)
                }
              }
            }
          }
        }

        hr;
        footer {
          p {
            "© 2024 " (cv.name)
          }
          @if let Some(contact) = &cv.contact {
            @if let Some(website) = &contact.website {
              p .contact-links { a href=(website) target="_blank" { (website.replace("https://", "").replace("http://", "")) } }
            }
          }
        }
      }
    }
  }
}
