use postgres::Connection;

// Rust Postgres does not support the interval type yet
// I should look into helping out,
// although that is far beyond my knowledge for both Rust and Postgres
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
            reference text,
            year smallint,
            name TEXT,
            movement_id integer REFERENCES movements (id)
        )
    ",
    &[]).ok().expect("could not create watches table");
}

pub fn create_movements_table(conn: &Connection) {
    conn.execute("
        CREATE TABLE IF NOT EXISTS movements (
            id SERIAL PRIMARY KEY,
            caliber text UNIQUE
        )
    ", &[]).ok().expect("could not create movements table");
}

// I need to explore the best way to add foreign keys in postgres
//
// I think I am just naming the actual constraint watches_company_id_fkey
pub fn add_company_id_to_watches(conn: &Connection) {
    conn.execute("
        ALTER TABLE watches
          ADD COLUMN company_id integer
    ", &[]).ok().expect("could not add company_id to watches");

    conn.execute("
        ALTER TABLE watches
          ADD CONSTRAINT watches_company_id_fkey FOREIGN KEY (company_id)
                  REFERENCES companies (id) MATCH SIMPLE
    ", &[]).ok().expect("could not add company_id to watches");
}
