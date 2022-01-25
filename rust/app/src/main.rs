mod models; //modelsフォルダの中のmod.rsを参照
mod schema;

#[macro_use]
extern crate diesel;

extern crate app;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

use schema::accounts;
use models::account_models::{PostReqAccount,NewAccount};

use actix_web::{get,post,web,App,HttpResponse,HttpServer,Responder};
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
        App::new()
        .service(index)
        .service(post_index)
    })
    .bind("0.0.0.0:81")?
    .run()
    .await
}
