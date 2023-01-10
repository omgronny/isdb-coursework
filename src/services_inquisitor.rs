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

#[get("/inquisitor")]
pub async fn get_root_inquisitor(data: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    let contents = fs::read_to_string("resources/static/inquisitor.html").unwrap();
    HttpResponse::Ok().body(contents)
}


#[get("/request")]
pub async fn get_all_requests(data: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    use crate::schema::*;

    let conn = &mut establish_connection();

    // select jedi.name, jedi_requests.price
    // from jedi_requests
    // join jedi on jedi_requests.jedi_id=jedi.id
    let result: Vec<(String, i32)> = jedi_requests::dsl::jedi_requests
        .inner_join(jedi::dsl::jedi)
        .select((jedi::dsl::name, jedi_requests::dsl::price))
        .load(conn)
        .expect("Error loading");

    HttpResponse::Ok().json(result)
}


#[post("/request")]
pub async fn post_requests(new_one: web::Json<GetJediRequest>, data: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    use crate::schema::*;

    let conn = &mut establish_connection();

    let values = new_one.into_inner();
    let return_value = values.clone();

    let name_data = values.name;
    let price = values.price;

    let jedi_res = jedi::dsl::jedi
        .filter(jedi::dsl::name.eq(name_data))
        .limit(1)
        .load::<Jedi>(conn)
        .expect("Error loading");

    let jedi_id = jedi_res[0].id;

    let new_jedi_request = NewJediRequest {
        jedi_id: &jedi_id,
        price: &price,
    };

    let res_jedi_request: JediRequest = diesel::insert_into(jedi_requests::table)
        .values(&new_jedi_request)
        .get_result(conn)
        .expect("Error saving new request");

    HttpResponse::Ok().json(return_value)
}


/*
Чтобы инквизитору понять, какие данные можно покупать, надо сделать
ему табличку, в которой просто выжимать из имеющейся даты только имя и цену (!!!)
 */
#[get("/my_jedi_data/{id}")]
pub async fn get_my_jedi_data(path: web::Path<usize>, data: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    use crate::schema::*;

    let conn = &mut establish_connection();
    let uid = path.into_inner() as i32;

    let result: Vec<(String, String, Option<i32>, Option<i32>, i32)> = jedi_data_inquisitors::dsl::jedi_data_inquisitors
        .filter(jedi_data_inquisitors::dsl::inquisitor_id.eq(uid))
        .inner_join(jedi_data::dsl::jedi_data)
        .inner_join(jedi::dsl::jedi.on(jedi_data::dsl::jedi_id.eq(jedi::dsl::id)))
        .inner_join(planets::dsl::planets.on(jedi_data::dsl::planet_id.eq(planets::dsl::id)))
        .select((jedi::dsl::name, planets::dsl::name, jedi_data::dsl::team_size, jedi_data::dsl::ship_power, jedi_data::dsl::normal_id))
        .load(conn)
        .expect("Error loading");

    HttpResponse::Ok().json(result)
}

#[get("/inquisitor/data")]
pub async fn get_all_data_for_inquisitor(data: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    use crate::schema::*;

    let conn = &mut establish_connection();

    // select jedi_data.id, jedi.name
    // from jedi_data
    // join jedi on jedi_data.jedi_id=jedi.id
    // join planets on jedi_data.planet_id=planets.id;
    let result: Vec<(String, i32, i32, Option<i32>)> = jedi_data::dsl::jedi_data
        .inner_join(jedi::dsl::jedi)
        .inner_join(planets::dsl::planets.on(jedi_data::dsl::planet_id.eq(planets::dsl::id)))
        .select((jedi::dsl::name, jedi_data::dsl::normal_id, jedi_data::dsl::id, jedi_data::price))
        .load(conn)
        .expect("Error loading");

    HttpResponse::Ok().json(result)
}

#[post("/inquisitor/data")]
pub async fn post_inquisitor_data(new_one: web::Json<BuyJediData>, data: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    use crate::schema::*;

    let conn = &mut establish_connection();
    let values = new_one.into_inner();
    let jedi_buy_id = values.buy_id;
    let jedi_jedi_id = values.jedi_id;

    let new_buy = NewJediDataInquisitor { inquisitor_id: &jedi_jedi_id, jedi_data_id: &jedi_buy_id };

    let res_buy_jedi: JediDataInquisitor = diesel::insert_into(jedi_data_inquisitors::table)
        .values(&new_buy)
        .get_result(conn)
        .expect("Error saving new request");

    HttpResponse::Ok().json(res_buy_jedi)
}