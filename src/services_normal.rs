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

#[get("/normal")]
pub async fn get_root_normal(data: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    let contents = fs::read_to_string("resources/static/normal.html").unwrap();
    HttpResponse::Ok().body(contents)
}

#[get("/walker.scss")]
pub async fn get_walker() -> HttpResponse {
    let contents = fs::read_to_string("resources/static/walker.scss").unwrap();
    HttpResponse::Ok().body(contents)
}

#[get("/data")]
pub async fn get_all_normal(data: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    use crate::schema::*;

    let conn = &mut establish_connection();

    // select jedi_data.id, jedi.name, planets.name, jedi_data.team_size, jedi_data.ship_power
    // from jedi_data
    // join jedi on jedi_data.jedi_id=jedi.id
    // join planets on jedi_data.planet_id=planets.id;
    let result: Vec<(String, String, Option<i32>, Option<i32>, i32, Option<i32>)> = jedi_data::dsl::jedi_data
        .inner_join(jedi::dsl::jedi)
        .inner_join(planets::dsl::planets.on(jedi_data::dsl::planet_id.eq(planets::dsl::id)))
        .select((jedi::dsl::name, planets::dsl::name, jedi_data::dsl::team_size, jedi_data::dsl::ship_power, jedi_data::dsl::normal_id, jedi_data::dsl::price))
        .load(conn)
        .expect("Error loading");

    HttpResponse::Ok().json(result)
}

#[get("/data/{id}")]
pub async fn get_one_normal(path: web::Path<usize>, data: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    use crate::schema::*;

    // TODO: code below is inconsistent
    let uid = path.into_inner() as i32;

    let conn = &mut establish_connection();
    let result = normals::dsl::normals
        .filter(normals::dsl::id.eq(uid))
        .limit(1)
        .load::<Normal>(conn)
        .expect("Error loading");

    HttpResponse::Ok().json(result)
}

#[post("/data")]
pub async fn post_data(new_one: web::Json<GetJediData>, data: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    use crate::schema::*;

    let conn = &mut establish_connection();

    let values = new_one.into_inner();
    let return_value = values.clone();

    let name_data = values.name;
    let planet_data = values.planet;
    let team = values.team;
    let ship = values.ship;
    let normal = values.normal_id;
    let prc = values.price;

    let jedi_res = jedi::dsl::jedi
        .filter(jedi::dsl::name.eq(name_data))
        .limit(1)
        .load::<Jedi>(conn)
        .expect("Error loading");

    let jedi_id = jedi_res[0].id;

    let planet_res = planets::dsl::planets
        .filter(planets::dsl::name.eq(planet_data))
        .load::<Planet>(conn)
        .expect("Error loading");

    let planet_id = planet_res[0].id;

    let new_jedi_data = NewJediData {
        jedi_id: &jedi_id,
        planet_id: &planet_id,
        normal_id: &normal,
        team_size: Some(&team),
        ship_power: Some(&ship),
        price: Some(&prc),
    };

    let res_jedi_data: JediData = diesel::insert_into(jedi_data::table)
        .values(&new_jedi_data)
        .get_result(conn)
        .expect("Error saving new data");

    HttpResponse::Ok().json(return_value)
}

#[delete("/data/{id}")]
pub async fn delete_one_data(path: web::Path<String>, data: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    use crate::schema::*;

    let name_data = path.into_inner();

    let conn = &mut establish_connection();
    let jedi_res = jedi::dsl::jedi
        .filter(jedi::dsl::name.eq(name_data))
        .limit(1)
        .load::<Jedi>(conn)
        .expect("Error loading");
    let jedi_id = jedi_res[0].id;

    let conn = &mut establish_connection();
    let result = diesel::delete(jedi_data::dsl::jedi_data.filter(jedi_data::dsl::jedi_id.eq(jedi_id)))
        .execute(conn)
        .expect("Error deleting");

    HttpResponse::Ok().json(result)
}




