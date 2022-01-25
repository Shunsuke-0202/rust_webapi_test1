extern crate diesel;
extern crate app;

use serde::{Deserialize,Serialize};

use diesel::{Queryable,Insertable};

use super::super::schema::accounts;

//POSTリクエスト受け取り用構造体
#[derive(Debug,Serialize,Deserialize)]
pub struct PostReqAccount{
    pub id:i32,
    pub name:String,
    pub student_number:String,
}

//db接続用構造体
#[derive(Queryable)]
pub struct DataBaseConnectAccount{
    pub id:i32,
    pub name:String,
    pub student_number:String,
}

//insert用構造体
#[derive(Insertable,Debug)]
#[table_name="accounts"]
pub struct NewAccount<'a>{
    pub id:&'a i32,
    pub name:&'a str,
    pub student_number:&'a str,
}
