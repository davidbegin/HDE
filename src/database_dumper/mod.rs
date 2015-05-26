use postgres::Connection;

// TODO: make this have a timestamp
// make the path configurable
//
// find out other options other than CSV (try NULL and DELIMTER)
pub fn companies(conn: &Connection) {
    conn.execute("Copy (SELECT * FROM companies) TO '/Users/dbegin/rust/hde/dump/companies.csv' WITH CSV", &[])
        .ok()
        .expect("could not copy company information");
}

pub fn watches(conn: &Connection) {
    conn.execute("Copy (SELECT * FROM watches) TO '/Users/dbegin/rust/hde/dump/watches.csv' WITH CSV", &[])
        .ok()
        .expect("could not copy watch information");
}
