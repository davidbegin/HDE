use postgres::Connection;

// Is this cool?
use {
    database_cleaner,
    database_creator,
    database_seeder,
    database_dumper,
    database_querier
};

pub fn title() {
    println!("\nA Project Combing 3 of my interests, Rust, Postgres and Watches\n");
}

pub fn clear(conn: &Connection) {
    database_cleaner::clear_watches(&conn);
    database_cleaner::clear_companies(&conn);
    database_cleaner::clear_movements(&conn);
}

pub fn drop_tables(conn: &Connection) {
    database_cleaner::drop_companies_table(&conn);
    database_cleaner::drop_watches_table(&conn);
    database_cleaner::drop_movements_table(&conn);
}

pub fn create_tables(conn: &Connection) {
    database_creator::create_companies_table(&conn);
    database_creator::create_movements_table(&conn);
    database_creator::create_watches_table(&conn);
    database_creator::add_company_id_to_watches(&conn);
}

pub fn seed_database(conn: &Connection) {
    // database_seeder::seed_companies(&conn);
    // database_seeder::seed_movements(&conn);
    // database_seeder::seed_watches(&conn);
    // database_seeder::associate_movements_and_watches(&conn);
    database_seeder::seed_from_csv(&conn);
}

pub fn copy_database(conn: &Connection) {
    database_dumper::companies(&conn);
    database_dumper::watches(&conn);
    database_dumper::movements(&conn);
}

pub fn blue_angels(conn: &Connection) {
    database_querier::print_all_companies(&conn);
    database_querier::print_all_watches(&conn);
    database_querier::print_all_movements(&conn);
}
