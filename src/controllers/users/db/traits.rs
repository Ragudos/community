use rocket_db_pools::Connection;

use crate::helpers::db::DbConn;

pub trait Token {
    fn is_expired(&self) -> bool;
}

pub trait Persistence {
    fn get_by_id(db: &mut Connection<DbConn>, id: i32) -> Result<Self, sqlx::Error>
    where
        Self: Sized;
    fn delete_by_id(db: &mut Connection<DbConn>, id: i32) -> Result<(), sqlx::Error>;
}
