//! Database interface

use crate::models::*;
use crate::schema::*;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::error::Error;

pub enum Query {
    Select(Option<String>),
    Insert(UserNew),
    Delete(String),
}

pub struct Database {
    connection: PgConnection,
}

impl Database {
    /// Initializes database (TODO with supplied parameters).
    pub fn init() -> Database {
        let connection = PgConnection::establish("postgres://postgres:postgres@localhost").unwrap();
        debug!("Connected to database");

        Database { connection }
    }

    /// Used only for cli or debugging purposes. Prints query result to stdout
    pub fn query(&self, query: Query) {
        // NOTE: Is internal user representation here ok?
        match query {
            Query::Select(login) => {
                let result = self.select(login);
                println!("{:?}", result);
            }
            Query::Insert(user) => match self.insert(user) {
                Ok(user) => println!("Created user: {:?}", user),
                Err(e) => println!("Could not create user: {}", e.description()),
            },
            Query::Delete(login) => match self.delete(login.as_str()) {
                Ok(user) => println!("Deleted user '{:?}'", user),
                Err(e) => println!("Could not delete user: '{}'", e.description()),
            },
        }
    }

    pub fn get_user(&self, login: &str) -> Option<User> {
        let mut result = pictures::table
            .filter(pictures::login.eq(login))
            .limit(1)
            .load::<User>(&self.connection)
            .expect("Could not get user");

        result.pop()
    }

    /// Lists all database users if `Option::None` is provided. Else print user with specified
    /// login.
    ///
    /// TODO: Implement search by different fields
    pub fn select(&self, login: Option<String>) -> Vec<User> {
        let mut matches = users::table.into_boxed();
        if let Some(login) = login {
            matches = matches.filter(users::login.eq(login));
        }

        matches
            .load::<User>(&self.connection)
            .expect("Could not get users")
    }

    /// Inserts user into the database.
    pub fn insert(&self, user: UserNew) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(&user)
            .get_result(&self.connection)
    }

    /// Removes user from database with specified login.
    ///
    /// TODO: Think of a way to prevent unauthorized deletions from the database.
    pub fn delete(&self, login: &str) -> QueryResult<User> {
        diesel::delete(users::table.filter(users::login.eq(login))).get_result(&self.connection)
    }
}
