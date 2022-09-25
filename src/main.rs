#![allow(unreachable_code)]

mod config;
mod renderer;

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
    config_data: &Mutex<config::Data>,
) -> Response {
    router!(request,
        (POST) (/login) => {
            let data = try_or_400!(post_input!(request, {
                login: String,
                password: String,
            }));
            println!("Login attempt with login {:?} and password {:?}", data.login, data.password);
            if data.password.starts_with("b") {
                *session_data = Some(SessionData { login: data.login });
                return Response::redirect_303("/");
            } else {
                return Response::html("Wrong login/password");
            }
        },
        (POST) (/logout) => {
            *session_data = None;
            return Response::html(r#"Logout successful.
                                     <a href="/">Click here to go to the home</a>"#);
        },
        _ => ()
    );
    if let Some(session_data) = session_data.as_ref() {
        handle_route_logged_in(request, session_data, config_data)
    } else {
        router!(request,
            (GET) (/) => {
                let page = renderer::LoginPage {  };
                Response::html(page.render().unwrap())
            },
            (GET) (/css) => {
                Response::text(include_str!("css/style.css"))
            },
            _ => Response::redirect_303("/")
        )
    }
}

fn handle_route_logged_in(
    request: &Request,
    _session_data: &SessionData,
    config_data: &Mutex<config::Data>,
) -> Response {
    router!(request,
        (GET) (/) => {
            Response::html(r#"You are now logged in. If you close your tab and open it again,
                              you will still be logged in.<br />
                              <a href="/private">Click here for the private area</a>
                              <form action="/logout" method="POST">
                              <button>Logout</button></form>"#)
        },
        (GET) (/private) => {
            let page = renderer::MainPage { test: "world".into() }; // instantiate your struct
            //println!("{}", );
            //Response::html(format!("{:?}", config_data.lock().unwrap()))
            Response::html(page.render().unwrap())
        },
        _ => Response::empty_404()
    )
}
