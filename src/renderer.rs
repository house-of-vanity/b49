use std::sync::Mutex;
use askama::Template;
use crate::config;

#[derive(Template)]
#[template(path = "main.html")]
pub struct MainPage<'a> {
    pub package_name: String,
    pub package_version: String,
    pub package_authors: String,
    pub config: &'a Mutex<config::Config>,
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginPage {
    pub package_name: String,
    pub package_version: String,
    pub package_authors: String,
}