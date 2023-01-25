use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};
use std::collections::HashMap;
use std::fmt::format;
use std::sync::{Arc, Mutex};
use std::sync::atomic::Ordering;

use actix_web::{App, delete, get, HttpRequest, HttpResponse, HttpServer, post, put, Responder, web};
use diesel::{QueryDsl, RunQueryDsl};
use diesel::prelude::*;
use diesel::sql_types::Uuid;

use crate::{establish_connection};
use crate::models::*;

use qstring::QString;

pub async fn not_found() -> HttpResponse {
    let contents = fs::read_to_string("resources/static/404.html").unwrap();
    HttpResponse::NotFound().body(contents)
}

#[get("/")]
pub async fn get_root(data: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    let contents = fs::read_to_string("resources/static/registration.html").unwrap();
    HttpResponse::Ok().body(contents)
}

#[get("/login")]
pub async fn register(req: HttpRequest, data: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    use crate::schema::*;

    println!("{}", req.query_string());

    let query_str = req.query_string();
    let qs = QString::from(query_str);

    // сохранить id в app state
    let user_name = qs.get("name").unwrap();
    let user_type = qs.get("user_type").unwrap();

    println!("{user_type}");

    let conn = &mut establish_connection();
    let contents = if user_type == "normal" {

        let result: Vec<i32> = normals::dsl::normals
            .filter(normals::dsl::name.eq(user_name))
            .select(normals::dsl::id)
            .load(conn)
            .expect("Error loading");

        data.lock().unwrap().id_nor = if result.len() > 0 {
            result[0]
        } else {

            // insert into normals
            let new_normal = NewNormal {
                name: &user_name,
                money: &100,
            };

            let normal: Normal = diesel::insert_into(normals::table)
                .values(&new_normal)
                .get_result(conn)
                .expect("Error saving new data");

            normal.id
        };

        fs::read_to_string("resources/static/normal.html").unwrap()

    } else if user_type == "inquisitor" {

        let result: Vec<i32> = inquisitors::dsl::inquisitors
            .filter(inquisitors::dsl::name.eq(user_name))
            .select(inquisitors::dsl::id)
            .load(conn)
            .expect("Error loading");

        data.lock().unwrap().id_inq = if result.len() > 0 {
            result[0]
        } else {

            // insert into normals
            let new_inquisitor = NewInquisitor {
                name: &user_name,
                power: &100,
                money: &100,
            };

            let inquisitor: Inquisitor = diesel::insert_into(inquisitors::table)
                .values(&new_inquisitor)
                .get_result(conn)
                .expect("Error saving new data");

            inquisitor.id
        };

        fs::read_to_string("resources/static/inquisitor.html").unwrap()
    } else {
        fs::read_to_string("resources/static/hunter.html").unwrap()
    };

    HttpResponse::Ok().body(contents)
}

#[get("/reg.css")]
pub async fn get_reg_style() -> HttpResponse {
    let contents = fs::read_to_string("resources/static/reg.css").unwrap();
    HttpResponse::Ok().body(contents)
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

