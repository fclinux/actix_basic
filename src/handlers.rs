use actix_files as fs;
use actix_utils::mpsc;
use actix_session::{ Session};
use actix_web::http::{header, Method, StatusCode};
use actix_web::{
    error, guard, middleware, web, get, HttpRequest, HttpResponse,
    App, Error, HttpServer
};
// use std::{env, io};




// Simple index handler
#[get("/welcome")]
async pub fn welcome(session: Session, req: HttpRequest) -> Result<HttpResponse>{
    println!("{:?}", req);

    // Session
    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter")?{
        println!("SESSION value: {}", count);
        counter = count +1;
    }

    // set counter to session
    session.set("counter", counter)?;

    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/welcome.html"))
    )
}

#[get("/favicon")]
async pub fn favicon() -> Result<fs::NamedFile>{
    Ok(fs::NamedFile::open("static/favicon.ico")?)
}

//404 handlers
async fn p404() -> Result<fs::NamedFile>{
    Ok(fs::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}

/// response body
async pub fn response_body(path: web::Path<String>) -> HttpResponse {
    let text = format!("Hello {}!", *path);

    let (tx, rx_body) = mpsc::channel();
    let _ = tx.send(Ok::<_, Error>(web::Bytes::from(text)));

    HttpResponse::Ok().streaming(rx_body)
}

/// handler with path parameters like `/user/{name}/`
async pub fn with_param(
    req: HttpRequest,
    web::Path((name,)): web::Path<(String,)>,
) -> HttpResponse {
    println!("{:?}", req);

    HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Hello {}!", name))
}