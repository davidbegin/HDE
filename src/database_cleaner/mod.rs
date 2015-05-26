use postgres::Connection;

pub fn clear_companies(conn: &Connection) {
    println!("Deleting all companies...");
    conn.execute("DELETE FROM companies", &[]).ok().expect("could not delete companies");
}

pub fn drop_companies_table(conn: &Connection) {
    println!("Dropping the companies table...");
    conn.execute("DROP TABLE IF EXISTS companies", &[]).ok().expect("could not drop table companies");
}
