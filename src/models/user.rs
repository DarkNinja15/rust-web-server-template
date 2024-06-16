use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::users;

#[derive(Queryable, Serialize, Deserialize,Insertable, Debug)]
#[table_name = "users"]
pub struct User{
    pub id:i32,
    pub name:String,
    pub email:String
}