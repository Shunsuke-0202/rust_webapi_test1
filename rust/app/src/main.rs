mod models; //modelsフォルダの中のmod.rsを参照
mod schema;

#[macro_use]
extern crate diesel;

extern crate app;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

use schema::accounts;
use models::account_models::{PostReqAccount,NewAccount};

use actix_cors::Cors;
use actix_web::{get,post,http,web,App,HttpResponse,HttpServer,Responder};
use serde::{Deserialize,Serialize};

#[get("/")]
async fn index()->impl Responder{
    HttpResponse::Ok().body("Hello World!")
}

#[post("/post")]
async fn post_index(user:web::Json<PostReqAccount>)->impl Responder{

    let connection=app::establish_connection();

    let new_account=NewAccount{
        id:&user.id,
        name:&user.name,
        student_number:&user.student_number,
    };

    diesel::insert_into(accounts::table)
        .values(&new_account)
        .execute(&connection)
        .expect("Error");


    println!("{:?}",new_account);
    HttpResponse::Ok().body("ok")
}

#[actix_web::main]
async fn main()->std::io::Result<()>{
    HttpServer::new(||{
        //CORSに対応させるためのオブジェクト作成
        let cors=Cors::default()
                .allowed_origin_fn(|origin,_req_head|{
                    true
                })
                .allowed_methods(vec!["GET","POST"])
                .allowed_headers(vec![http::header::AUTHORIZATION,http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600);

        App::new()
            .wrap(cors)
            .service(index)
            .service(post_index)
    })
    .bind("0.0.0.0:81")?
    .run()
    .await
}