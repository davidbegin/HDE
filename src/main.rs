#![allow(unused_imports, unused_variables, dead_code)]

extern crate postgres;
extern crate type_printer;
extern crate csv;
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

    // database_cleaner::clear_companies(&conn);
    // database_cleaner::clear_watches(&conn);
    // database_cleaner::clear_movements(&conn);

    // database_cleaner::drop_companies_table(&conn);
    // database_cleaner::drop_watches_table(&conn);
    // database_cleaner::drop_movements_table(&conn);
    //
    // database_creator::create_companies_table(&conn);
    // database_creator::create_movements_table(&conn);
    // database_creator::create_watches_table(&conn);
    // database_creator::add_company_id_to_watches(&conn);
    //
    // database_seeder::seed_companies(&conn);
    // database_seeder::seed_movements(&conn);
    // database_seeder::seed_watches(&conn);
    // database_seeder:: associate_movements_and_watches(&conn);

    // database_dumper::companies(&conn);
    // database_dumper::watches(&conn);
    // database_dumper::movements(&conn);

    // database_querier::print_all_companies(&conn);
    // database_querier::print_all_watches(&conn);
    // database_querier::print_all_movements(&conn);

    csv_experiments();
}

fn title() {
    println!("\nA Project Combing 3 of my interests, Rust, Postgres and Watches\n");
}

fn csv_experiments() {
    println!("Time to parse some CSV");

    let mut reader = csv::Reader::from_file("./src/database_seeder/data/test_watch_data_1.csv").unwrap();

    for record in reader.decode() {
        let (name, reference_id, year, caliber_name): (String, String, String, String) = record.unwrap();
        println!("name: {}, reference: {}, year: {}, caliber name: {}",
            name, reference_id, year, caliber_name
        );
    }
}
