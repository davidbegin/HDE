use postgres::Connection;

// Lets make some associations
//
// And many questions are raised
//
// manufacture is the same as company

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
    // FOREIGN KEY (book_id) REFERENCES books (id);
    // product_no integer REFERENCES products (product_no),

    conn.execute("
        CREATE TABLE IF NOT EXISTS watches (
            id SERIAL PRIMARY KEY,
            reference_id text,
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
            caliber text
        )
    ", &[]).ok().expect("could not create movements table");
}
