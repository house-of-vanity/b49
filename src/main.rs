#![allow(unreachable_code)]

mod config;
mod renderer;
mod router;

#[macro_use]
extern crate rouille;

use rouille::Request;
use rouille::Response;
use std::collections::HashMap;
use std::io;
use std::sync::Mutex;
use askama::Template;


#[derive(Debug, Clone)]
struct SessionData {
    login: String,
}
#[derive(Debug, Clone)]
struct PostData {
    message: String,
    providers: String,
}

const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");
const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
const PACKAGE_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn main() {
    let config_data = Mutex::new(config::read_config());
    println!("Now listening on localhost:8000");
    let sessions_storage: Mutex<HashMap<String, SessionData>> = Mutex::new(HashMap::new());
    rouille::start_server("localhost:8000", move |request| {
        rouille::log(&request, io::stdout(), || {
            rouille::session::session(request, "SID", 3600, |session| {
                let mut session_data = if session.client_has_sid() {
                    if let Some(data) = sessions_storage.lock().unwrap().get(session.id()) {
                        Some(data.clone())
                    } else {
                        None
                    }
                } else {
                    None
                };
                let response = handle_route(&request, &mut session_data, &config_data);
                if let Some(d) = session_data {
                    sessions_storage
                        .lock()
                        .unwrap()
                        .insert(session.id().to_owned(), d);
                } else if session.client_has_sid() {
                    sessions_storage.lock().unwrap().remove(session.id());
                }
                response
            })
        })
    });
}

fn handle_route(
    request: &Request,
    session_data: &mut Option<SessionData>,
    config_data: &Mutex<config::Config>,
) -> Response {
    router!(request,
        (POST) (/login) => {
            let data = try_or_400!(post_input!(request, {
                login: String,
                password: String,
            }));
            let password = config_data.lock().unwrap().web.password.clone();
            let user = config_data.lock().unwrap().web.user.clone();
            println!("Login attempt with login {:?}.", data.login);

            if data.password == password && data.login == user {
                *session_data = Some(SessionData { login: data.login });
                return Response::redirect_303("/");
            } else {
                return Response::html("Wrong login/password");
            }
        },
        (POST) (/logout) => {
            *session_data = None;
            return Response::redirect_303("/");
        },
        _ => ()
    );
    if let Some(session_data) = session_data.as_ref() {
        handle_route_logged_in(request, session_data, config_data)
    } else {
        router!(request,
            (GET) (/) => {
                let page = renderer::LoginPage {
                    package_name: PACKAGE_NAME.to_string(),
                    package_authors: PACKAGE_AUTHORS.to_string(),
                    package_version: PACKAGE_VERSION.to_string(),
                };
                Response::html(page.render().unwrap())
            },
            (GET) (/css) => {
                Response::from_data("text/css", include_str!("css/style.css"))
            },
            _ => Response::redirect_303("/")
        )
    }
}

fn handle_route_logged_in(
    request: &Request,
    _session_data: &SessionData,
    config_data: &Mutex<config::Config>,
) -> Response {
    router!(request,
        (GET) (/) => {
            // let result = request.get_param("result");
            let page = renderer::MainPage {
                package_name: PACKAGE_NAME.to_string(),
                package_authors: PACKAGE_AUTHORS.to_string(),
                package_version: PACKAGE_VERSION.to_string(),
                config: config_data,
                // result: result,
            };

            Response::html(page.render().unwrap())
        },
        (GET) (/config) => {
            Response::html(format!("{:?}", config_data.lock().unwrap()))
        },
        (GET) (/css) => {
            Response::from_data("text/css", include_str!("css/style.css"))
        },
        (POST) (/) => {
            let data = try_or_400!(post_input!(request, {
                message: String,
                providers: Vec<String>,
            }));
            println!("{:?}", data);

            return Response::redirect_303("/?result='Done'");
        },
        _ => Response::empty_404()
    )
}
