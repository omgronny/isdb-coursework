use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};
use std::collections::HashMap;
use std::fmt::format;
use std::sync::{Arc, Mutex};
use std::sync::atomic::Ordering;

use actix_web::{App, delete, get, HttpResponse, HttpServer, post, put, Responder, web};
use diesel::{QueryDsl, RunQueryDsl};
use diesel::prelude::*;
use diesel::sql_types::Uuid;

use crate::{establish_connection, VecOfMap};
use crate::models::*;

pub async fn not_found() -> HttpResponse {
    let contents = fs::read_to_string("resources/static/404.html").unwrap();
    HttpResponse::NotFound().body(contents)
}

#[get("/js/{filename}")]
pub async fn get_js(path: web::Path<String>) -> HttpResponse {
    let contents = fs::read_to_string("resources/static/js/".to_owned() + path.into_inner().as_str()).unwrap();
    HttpResponse::Ok().body(contents)
}

#[get("/jedi")]
pub async fn get_all_jedi(data: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    use crate::schema::jedi::dsl::*;

    let conn = &mut establish_connection();
    let result = jedi
        .load::<Jedi>(conn)
        .expect("Error loading");

    HttpResponse::Ok().json(result)
}


pub async fn index(data: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    println!("normal.html");
    let contents = fs::read_to_string("resources/static/normal.html").unwrap();
    HttpResponse::Ok().body(contents)
}

