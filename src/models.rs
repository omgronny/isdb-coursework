#![allow(unused)]
#![allow(clippy::all)]

use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::AtomicI64;
use std::sync::Mutex;

use diesel::r2d2::ConnectionManager;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Status {
    pub status: String,
}

#[derive(Deserialize)]
pub struct Message {
    pub name: String,
    pub money: String,
}

pub struct AppState {
    pub id_inq: i32,
    pub id_nor: i32,
    pub id_hunt: i32,
}

use crate::schema::*;

#[derive(Insertable)]
#[diesel(table_name = normals)]
pub struct NewNormal<'a> {
    pub name: &'a str,
    pub money: &'a i32,
}

#[derive(Queryable, Debug, Serialize)]
pub struct Normal {
    pub id: i32,
    pub name: String,
    pub money: i32,
}

#[derive(Queryable, Debug, Serialize)]
pub struct Jedi {
    pub id: i32,
    pub name: String,
    pub power: i32,
}


#[derive(Queryable, Debug)]
#[diesel(primary_key(event_id, planet_id))]
pub struct EventPlanet {
    pub event_id: i32,
    pub planet_id: i32,
}

#[derive(Queryable, Debug, Identifiable)]
pub struct Event {
    pub id: i32,
    pub reason: Option<String>,
    pub damage: Option<i32>,
}

#[derive(Queryable, Debug, Identifiable)]
pub struct Hunter {
    pub id: i32,
    pub name: Option<String>,
    pub power: i32,
    pub money: i32,
    pub ship_id: i32,
}

#[derive(Queryable, Debug, Identifiable)]
pub struct Inquisitor {
    pub id: i32,
    pub name: String,
    pub power: i32,
    pub money: i32,
}

#[derive(Queryable, Debug, Identifiable)]
#[diesel(primary_key(inquisitor_id, squad_id))]
pub struct InquisitorsSquad {
    pub inquisitor_id: i32,
    pub squad_id: i32,
}

#[derive(Queryable, Debug, Serialize)]
pub struct JediData {
    pub id: i32,
    pub jedi_id: i32,
    pub planet_id: i32,
    pub normal_id: i32,
    pub team_size: Option<i32>,
    pub ship_power: Option<i32>,
    pub price: Option<i32>,
}
#[derive(Insertable)]
#[diesel(table_name = jedi_data)]
pub struct NewJediData<'a> {
    pub jedi_id: &'a i32,
    pub planet_id: &'a i32,
    pub normal_id: &'a i32,
    pub team_size: Option<&'a i32>,
    pub ship_power: Option<&'a i32>,
    pub price: Option<&'a i32>,
}
#[derive(Deserialize, Serialize, Clone)]
pub struct GetJediData {
    pub name: String,
    pub planet: String,
    pub team: i32,
    pub ship: i32,
    pub normal_id: i32,
    pub price: i32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct BuyJediData {
    pub buy_id: i32,
    pub jedi_id: i32,
}

#[derive(Queryable, Debug)]
pub struct JediRequest {
    pub id: i32,
    pub price: i32,
    pub jedi_id: i32,
}
#[derive(Insertable)]
#[diesel(table_name = jedi_requests)]
pub struct NewJediRequest<'a> {
    pub jedi_id: &'a i32,
    pub price: &'a i32,
}
#[derive(Deserialize, Serialize, Clone)]
pub struct GetJediRequest {
    pub name: String,
    pub price: i32,
}

#[derive(Queryable, Debug)]
pub struct Mission {
    pub id: i32,
    pub complexity: i32,
    pub jedi_id: i32,
}

#[derive(Queryable, Debug)]
pub struct Planet {
    pub id: i32,
    pub name: String,
    pub color: String,
    pub size: i32,
    pub race: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct Ship {
    pub id: i32,
    pub model: String,
    pub color: String,
    pub fuel_type: Option<String>,
    pub power: i32,
}

#[derive(Queryable, Debug)]
pub struct Squad {
    pub id: i32,
    pub name: String,
    pub ship_id: i32,
}

#[derive(Queryable, Debug, Serialize)]
#[diesel(primary_key(jedi_data_id, inquisitor_id))]
pub struct JediDataInquisitor {
    pub jedi_data_id: i32,
    pub inquisitor_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = jedi_data_inquisitors)]
pub struct NewJediDataInquisitor<'a> {
    pub jedi_data_id: &'a i32,
    pub inquisitor_id: &'a i32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GetUserType {
    pub name: String,
    pub user_type: String,
}

