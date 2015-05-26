use postgres::Connection;

// Rust Postgres does not support the interval type yet
// I should look into helping out,
// although that is far beyond my knowledge for both categories
pub fn create_companies_table(conn: &Connection) {
    conn.execute("
        CREATE TABLE IF NOT EXISTS companies (
            id SERIAL PRIMARY KEY,
            name TEXT UNIQUE,
            year_founded smallint
        )",
    &[]).ok().expect("could not create companies table");
}

pub fn create_watches_table(conn: &Connection) {
    conn.execute("
        CREATE TABLE IF NOT EXISTS watches (
            id SERIAL PRIMARY KEY,
            reference_id text,
            year smallint,
            name TEXT
        )
    ",
    &[]).ok().expect("could not create watches table");
}
