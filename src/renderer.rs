use std::sync::Mutex;
use askama::Template;
use serde::__private::de::TagContentOtherField;
use crate::config;
// use std::fmt;

#[derive(Template)]
#[template(path = "main.html")]
pub struct MainPage<'a> {
    pub package_name: String,
    pub package_version: String,
    pub package_authors: String,
    pub config: &'a Mutex<config::Config>,
    // pub result: Option<String>,
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginPage {
    pub package_name: String,
    pub package_version: String,
    pub package_authors: String,
}