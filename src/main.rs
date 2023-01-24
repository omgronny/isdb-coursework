#[macro_use]
extern crate diesel;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicI64;

use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};
use diesel::r2d2::ConnectionManager;

use crate::db::*;
use crate::models::AppState;
use crate::services::*;
use crate::services_normal::*;
use crate::services_inquisitor::*;
use crate::services_hunter::*;

mod models;
mod services;
mod db;
pub mod schema;
mod services_inquisitor;
mod services_normal;
mod services_hunter;

/*
Всем нужно видеть 2 основные таблицы: jedi_request и jedi_data
Также у всех в углу есть типо баланс

У инквизитора еще:
    - форма добавить request
    - доступ к таблице наемников

У нормала:
    - форма добавить data
    -
У наемника:
    -
 */

/* TODO: наемник
    - инквизитору добавить покупку наемников
    - придумать интерфейс для наемника
        - наемник может принимать заявки ??
        - сам может покупать инфу
        - есть Jedi и Jedi Request
*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let app_name = Vec::<HashMap::<String, String>>::from([]);
    dotenv::dotenv().ok();

    let data = Arc::new(Mutex::new(
        AppState {
            id_nor: 0,
            id_hunt: 0,
            id_inq: 0,
        }
    ));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(data.clone()))
            .service(get_js)
            .service(get_root)
            .service(register)
            .service(get_walker)
            .service(get_root_normal)
            .service(get_root_inquisitor)
            .service(get_all_normal)
            .service(get_money_normal)
            .service(get_one_normal)
            .service(post_data)
            .service(delete_one_data)
            .service(get_all_jedi)
            .service(get_all_requests)
            .service(post_requests)
            .service(get_my_jedi_data)
            .service(get_all_data_for_inquisitor)
            .service(post_inquisitor_data)
            .service(get_money_inquisitor)
            .service(get_root_hunter)
            .service(get_money_hunter)
            .service(get_strooper)
            .service(get_reg_style)
            .default_service(
                web::route().to(not_found)
            )
    })    .bind(("127.0.0.1", 2471))?
    .run()
    .await
}


