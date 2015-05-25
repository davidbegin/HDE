#![allow(unused_imports, unused_variables)]

extern crate postgres;

use postgres::{Connection, SslMode};

fn main() {
    println!("\nA Project Combing 3 of my interests, Rust, Postgres and Watches\n");

    let database = "postgresql://dbegin@localhost/watches";

    let conn = match Connection::connect(database, &SslMode::None) {
        Ok(conn) => conn,
        Err(e) => {
            println!("connection error: {:?}", e);
            return;
        }
    };

    // Lets start with something like this:
    // Company
    //     name:
    // Watch
    //     series:
    //     ref:
    // Movement
    //     name:

    conn.execute("
        CREATE TABLE IF NOT EXISTS companies (
            id serial primary key,
            name text
        )
    ", &[]).ok().expect("could not create companies table");

    let stmt = match conn.prepare("insert into companies (name) values ($1)") {
        Ok(stmt) => stmt,
        Err(e) => {
            println!("having touble preparing to insert company");
            return;
        }
    };

    let first_company = format!("Panerai");
    let second_company = format!("Rolex");
    let third_company = format!("Rolex");
    let fourth_company = format!("Audemars Piguet");
    let fifth_company = format!("IWC Schaffhausen");

    let companies: Vec<String> = vec![
        first_company,
        second_company,
        third_company,
        fourth_company,
        fifth_company
    ];

    for company in companies {
        stmt.execute(&[&company])
            .ok()
            .expect("there was a problem inserting company");
    }
}
