#![allow(unused_imports, unused_variables, dead_code)]

extern crate postgres;
extern crate type_printer;
mod database_creator;
mod database_seeder;
mod database_cleaner;
mod database_dumper;
mod database_querier;
mod config;
mod tests;

fn main() {
    title();
    let conn = match config::database_connection() {
        Some(conn) => conn,
        None => {
            println!("Looks like we got a nil connection");
            return;
        }
    };

    database_cleaner::drop_companies_table(&conn);
    database_creator::create_companies_table(&conn);
    database_seeder::seed_companies(&conn);

    // database_cleaner::clear_companies(&conn);
    // database_dumper::companies(&conn);

    database_querier::print_all_companies(&conn);
}

fn title() {
    println!("\nA Project Combing 3 of my interests, Rust, Postgres and Watches\n");
}

