use diesel;
//use diesel::prelude::*;
use diesel::pg::PgConnection;

use self::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub email: String
}

impl User {
    /*pub fn login(username: String, password: String, conn: PgConnection) -> bool {
        let mut query = String::from("SELECT * FROM users WHERE username = ");
        query.push_str(&username);
        query.push_str(" AND password = ");
        query.push_str(&password);
        diesel::sql_query(query).load(&conn).is_ok()
    }*/
    pub fn insert(user: NewUser, conn: &PgConnection) -> bool {
        diesel::insert_into("users")
            .values(&user)
            .execute(&conn)
            .is_ok()
    }
}