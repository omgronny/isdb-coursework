#[macro_use]
extern crate diesel;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicI64;

use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};
use diesel::r2d2::ConnectionManager;

use crate::db::*;
use crate::models::{AppState, VecOfMap};
use crate::services::*;
use crate::services_normal::*;
use crate::services_inquisitor::*;

mod models;
mod services;
mod db;
pub mod schema;
mod services_inquisitor;
mod services_normal;

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

    // set up database connection pool
    // let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    // let manager = ConnectionManager::<SqliteConnection>::new(conn_spec);
    // let pool = r2d2::Pool::builder()
    //     .build(manager)
    //     .expect("Failed to create pool.");

    let data = Arc::new(Mutex::new(
        AppState {
            vec_of_map: VecOfMap { vec: app_name.clone() },
            id_num: 0,
            // pool,
        }
    ));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(data.clone()))
            .service(
            web::scope("/app")
                .route("/normal.html", web::get().to(index)),
            )
            .service(get_js)
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
            .default_service(
                web::route().to(not_found)
            )
    })    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


