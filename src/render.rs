use maud::{html, Markup, DOCTYPE, PreEscaped};
use pulldown_cmark::{Parser, Options};
use crate::sections::CV;

fn parse_md(md: &str) -> String {
  let options = Options::empty();
  let parser = Parser::new_ext(md, options);
  let mut out = String::new();
  pulldown_cmark::html::push_html(&mut out, parser);
  out
}

pub fn render_cv(cv: &CV) -> Markup {
  html! {
    (DOCTYPE)
    html lang="en" {
      head {
        meta charset="utf-8";
        meta name="viewport" content="width=device-width,initial-scale=1";
        title { (cv.name) " - Resume" }
        link rel="stylesheet" type="text/css" href="/static/style.css";
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
              @if let Some(location) = &contact.location {
                p #location { (location) }
              }
            }
          }
        }

        @if let Some(summary) = &cv.summary {
          hr;
          section #summary {
            p { (PreEscaped(parse_md(summary))) }
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
                    span.time { (edu.graduation_year) }
                  }
                  @if let Some(gpa) = &edu.gpa {
                    p.gpa {
                      "GPA: " (format!("{:.2}", gpa))
                      @if let Some(max_gpa) = &edu.max_gpa { " / " (format!("{:.2}", max_gpa))}
                    }
                  }
                  @if let Some(description) = &edu.description {
                    ul.description {
                      @for desc in description {
                        li { (PreEscaped(parse_md(desc))) }
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
                    span.time {
                      @if exp.end_date == exp.start_date {
                        (exp.end_date.format_long_name())
                      } @else {
                        (exp.start_date.format_short_name()) " - " (exp.end_date.format_short_name())
                      }
                    }
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
            ul.items {
              @for items in skills {
                li.item {
                  b { (items.category) ": " }
                  (items.skills.join(", "))
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
            "© 2026 " (cv.name)
          }
          @if let Some(contact) = &cv.contact {
            @if let Some(website) = &contact.website {
              p .contact-links { a href={(website) "/resume"} target="_blank" { (website.replace("https://", "").replace("http://", "")) } }
            }
          }
        }
      }
    }
  }
}
