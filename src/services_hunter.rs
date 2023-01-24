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

use crate::{establish_connection};
use crate::models::*;
use crate::schema::jedi_data::price;


#[get("/hunter")]
pub async fn get_root_hunter(data: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    let contents = fs::read_to_string("resources/static/hunter.html").unwrap();
    HttpResponse::Ok().body(contents)
}

#[get("/hunter-id/{id}")]
pub async fn get_money_hunter(path: web::Path<usize>, data: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    use crate::schema::*;

    let conn = &mut establish_connection();
    let uid = path.into_inner() as i32;

    let result: Vec<i32> = hunters::dsl::hunters
        .filter(hunters::dsl::id.eq(uid))
        .select(hunters::dsl::money)
        .load(conn)
        .expect("Error loading");

    HttpResponse::Ok().json(result)

}

#[get("/strooper.scss")]
pub async fn get_strooper() -> HttpResponse {
    let contents = fs::read_to_string("resources/static/strooper.scss").unwrap();
    HttpResponse::Ok().body(contents)
}

















