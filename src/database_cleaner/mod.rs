use postgres::Connection;

// TODO: add a verbose mode

pub fn clear_companies(conn: &Connection) {
    conn.execute("UPDATE watches SET company_id = null", &[])
        .ok()
        .expect("could not delete company_id references");

    // println!("Deleting all companies...");
    conn.execute("DELETE FROM companies CASCADE", &[])
        .ok()
        .expect("could not delete companies");
}

pub fn drop_companies_table(conn: &Connection) {
    // println!("Dropping the companies table...");
    conn.execute("DROP TABLE IF EXISTS companies CASCADE", &[])
        .ok()
        .expect("could not drop table companies");
}

pub fn clear_watches(conn: &Connection) {
    // println!("Deleting all watches...");
    conn.execute("DELETE FROM watches CASCADE", &[])
        .ok()
        .expect("could not delete watches");
}

pub fn drop_watches_table(conn: &Connection) {
    // println!("Dropping watches table...");
    conn.execute("DROP TABLE IF EXISTS watches CASCADE", &[])
        .ok()
        .expect("could not drop watches table");
}

pub fn clear_movements(conn: &Connection) {
    // println!("Deleting all movement...");
    conn.execute("DELETE FROM movements CASCADE", &[]).ok().expect("could not delete movements");
}

pub fn drop_movements_table(conn: &Connection) {
    // println!("Dropping movements table...");
    conn.execute("DROP TABLE IF EXISTS movements CASCADE", &[])
        .ok()
        .expect("could not drop movements table");
}
