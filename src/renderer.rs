use askama::Template;

#[derive(Template)]
#[template(path = "main.html")]
pub struct MainPage {
    pub test: String,
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginPage {
}