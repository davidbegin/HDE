#![allow(unused_imports, unused_variables, dead_code, unused_mut)]

extern crate postgres;
extern crate type_printer;
extern crate csv;

mod database_creator;
mod database_cleaner;
mod database_dumper;
mod database_querier;
mod database_seeder;
mod config;
mod tests;
mod control_tower;
use postgres::{Connection, Error, FromSql, SslMode};
use postgres::Result as PgResult;

fn main() {
    control_tower::title();

    let conn = match config::database_connection() {
        Some(conn) => conn,
        None => {
            println!("Looks like we got a nil connection");
            return;
        }
    };

    // control_tower::clear(&conn);
    // control_tower::drop_tables(&conn);
    // control_tower::create_tables(&conn);
    // control_tower::seed_database(&conn);
    // control_tower::copy_database(&conn);
    // control_tower::blue_angels(&conn);
}
