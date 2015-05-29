#![allow(unused_imports, unused_variables, dead_code)]

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

    try_messing_around(&conn);
}

fn try_messing_around(conn: &Connection) {
    let query = "SELECT * FROM movements";
    // let result= get_single_value(&conn, &query);
    // let arr = get_single_value::<IntArray>(&conn, "select '{4, 5, 6}'::int[]");

}

fn get_single_value<T>(conn: &Connection, query: &str) -> PgResult<T>
    where T: FromSql {
        println!("Executing query: {}", query);
        let stmt = try!(conn.prepare(query));
        let mut rows = try!(stmt.query(&[]));
        let row = try!(rows.next().ok_or(Error::BadData));
        row.get_opt(0)
    }
