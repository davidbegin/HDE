use postgres::Connection;

// TODO: make this have a timestamp
// make the path configurable
//
// find out other options other than CSV (try NULL and DELIMTER)
pub fn companies(conn: &Connection) {
    conn.execute("Copy (Select * From companies) To '/Users/dbegin/rust/hde/dump/companies.csv' With CSV", &[])
        .ok()
        .expect("could not copy company information");
}
