use axum::{
    response::{Html, IntoResponse},
    extract::Form,
};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct RegisterForm {
    pub mobile_no: String
}

pub async fn show_register_form() -> impl IntoResponse {
    let html_content = fs::read_to_string("src/register.html")
        .expect("Failed to read HTML file");
    let toggle_form = "show_form"; // setting css form display (so we can hide it later)
    let current = html_content.replace("{{result_message}}", "");
    let show_form = current.replace("{{class}}", toggle_form);

    Html(show_form)
}


pub async fn process_registration(Form(form): Form<RegisterForm>) -> impl IntoResponse {
    let toggle_form = "hide_form";
    let result_message = format!("Registration succesful with your mobile no: {}", form.mobile_no);

    let html_content = fs::read_to_string("src/register.html")
        .expect("Failed to read HTML file");

    let updated_html = html_content.replace("{{result_message}}", &result_message);
    let hide_form = updated_html.replace("{{class}}", &toggle_form);

    Html(hide_form)
}


