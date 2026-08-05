use std::io::Cursor;
use std::path::Path;
use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{Responder, Response};
use serde::Serialize;
use serde_json::Value;
use tera::{Context, Tera};

use crate::filters::{format_date, tofixed};

#[derive(Debug)]
pub struct Template {
    name: String,
    context: Value,
}

fn walk_and_collect(dir: &Path, base_dir: &Path, templates: &mut Vec<(String, String)>) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                walk_and_collect(&path, base_dir, templates)?;
            } else if path.is_file() {
                if let Ok(rel) = path.strip_prefix(base_dir) {
                    let rel_str = rel.to_string_lossy().to_string();
                    if rel_str.ends_with(".tera") || rel_str.ends_with(".html") {
                        if let Ok(content) = std::fs::read_to_string(&path) {
                            // 1. Original relative path (e.g. "pages/extended/home.html.tera")
                            templates.push((rel_str.clone(), content.clone()));

                            // 2. Alias without .html.tera / .tera extension (e.g. "pages/extended/home" & "base")
                            if rel_str.ends_with(".html.tera") {
                                let no_ext = rel_str.trim_end_matches(".html.tera").to_string();
                                templates.push((no_ext.clone(), content.clone()));
                                let html_ext = format!("{}.html", no_ext);
                                templates.push((html_ext, content.clone()));
                            } else if rel_str.ends_with(".tera") {
                                let no_ext = rel_str.trim_end_matches(".tera").to_string();
                                templates.push((no_ext, content.clone()));
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn load_templates(tera: &mut Tera, dir_path: &str) {
    let base_path = Path::new(dir_path);
    let mut raw_templates = Vec::new();
    if let Err(e) = walk_and_collect(base_path, base_path, &mut raw_templates) {
        eprintln!("Error walking templates directory: {:?}", e);
    }
    if let Err(e) = tera.add_raw_templates(raw_templates) {
        eprintln!("Error loading tera templates: {:?}", e);
    }
}

impl Template {
    pub fn render<S: Into<String>, C: Serialize>(name: S, context: C) -> Template {
        Template {
            name: name.into(),
            context: serde_json::to_value(context).unwrap_or(Value::Null),
        }
    }

    pub fn fairing() -> impl rocket::fairing::Fairing {
        Template::custom(|_| {})
    }

    pub fn custom<F>(f: F) -> impl rocket::fairing::Fairing
    where
        F: FnOnce(&mut Tera) + Send + Sync + 'static,
    {
        rocket::fairing::AdHoc::on_ignite("Tera Templates", |rocket| async move {
            let mut tera = Tera::default();
            tera.register_filter("tofixed", tofixed);
            tera.register_filter("format_date", format_date);
            f(&mut tera);
            load_templates(&mut tera, "templates");
            rocket.manage(tera)
        })
    }
}

impl<'r> Responder<'r, 'static> for Template {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'static> {
        let tera = req.rocket().state::<Tera>().ok_or_else(|| {
            eprintln!("Tera engine state not found in Rocket instance");
            Status::InternalServerError
        })?;

        let name = if tera.get_template_names().any(|t| t == self.name) {
            self.name.clone()
        } else {
            let with_ext = format!("{}.html.tera", self.name);
            if tera.get_template_names().any(|t| t == with_ext) {
                with_ext
            } else {
                self.name.clone()
            }
        };

        let context = Context::from_serialize(&self.context).map_err(|e| {
            eprintln!("Error creating Tera context: {:?}", e);
            Status::InternalServerError
        })?;

        match tera.render(&name, &context) {
            Ok(rendered) => Response::build()
                .header(ContentType::HTML)
                .sized_body(rendered.len(), Cursor::new(rendered))
                .ok(),
            Err(e) => {
                eprintln!("Template rendering error for '{}': {:?}", self.name, e);
                let err_body = format!("<h1>Template Error</h1><pre>{:?}</pre>", e);
                Response::build()
                    .status(Status::InternalServerError)
                    .header(ContentType::HTML)
                    .sized_body(err_body.len(), Cursor::new(err_body))
                    .ok()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_loading_and_inheritance() {
        let mut tera = Tera::default();
        tera.register_filter("tofixed", tofixed);
        tera.register_filter("format_date", format_date);
        load_templates(&mut tera, "templates");

        let names: Vec<_> = tera.get_template_names().collect();
        assert!(names.contains(&"base"));
        assert!(names.contains(&"index"));
        assert!(names.contains(&"pages/home"));
    }
}


