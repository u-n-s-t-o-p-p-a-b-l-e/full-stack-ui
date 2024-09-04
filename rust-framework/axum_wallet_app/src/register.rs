use axum::{
    response::{Html, IntoResponse},
    extract::Form,
};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct RegisterForm {
    pub mobile_no: String,
}

pub async fn show_register_form() -> impl IntoResponse {
    let html_content = fs::read_to_string("src/register.html")
        .expect("Failed to read HTML file");
    let toggle_form = "show_form"; // setting css form display (so we can hide it later)
    let current = html_content.replace("{{result_message}}", "");
    let show_form = current.replace("{{class}}", toggle_form);

    Html(show_form)
}
