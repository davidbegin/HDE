#![allow(unused_imports, unused_variables, dead_code)]

extern crate postgres;
extern crate type_printer;

// MVP SCHEMA:
//     Company
//         name:
//     Watch
//         series:
//         ref:
//     Movement
//         name:

// TODO:
//     2. finish creating basic schema
//          a. associating the tables
//          b. building some basic queries
//
//     4. add some functions for cleaning state
//     5. get some benchmarks
//     6. gets some tests
//     7. figure how to dump the schema and data
//     8. seperate out into modules
//     9. find out to show better errors from postgres
//     10. learn more about what postgres offers

fn main() {
    title();
    let connection_option = config::database_connection();

    let conn = match connection_option {
        Some(conn) => conn,
        None => {
            println!("Looks like we got a nil connection");
            return;
        }
    };

    database_creator::create_companies_table(&conn);
    database_cleaner::clear_companies(&conn);
    database_seeder::seed_companies(&conn);
    database_dumper::companies(&conn);
    database_cleaner::drop_companies_table(&conn);
}

mod database_seeder {
    use companies;
    use postgres::Connection;

    pub fn seed_companies(conn: &Connection) {
        let stmt = match conn.prepare("insert into companies (name) values ($1)") {
            Ok(stmt) => stmt,
            Err(e) => {
                println!("having touble preparing to insert company");
                return;
            }
        };

        for company in companies::all() {
            stmt.execute(&[&company])
                .ok()
                .expect("there was a problem inserting company");
        }
    }
}

mod companies {
    pub fn all() -> Vec<String> {
        // I want to read these in from a CSV file,
        // and then I want to experiment with dumping the data
        let first_company = format!("Panerai");
        let second_company = format!("Rolex");
        let third_company = format!("A. Lange & Söhne");
        let fourth_company = format!("Audemars Piguet");
        let fifth_company = format!("IWC Schaffhausen");

        // Copy (Select * From companies) To 'tmp/test.csv' With CSV;

        vec![
            first_company,
            second_company,
            third_company,
            fourth_company,
            fifth_company
        ]
    }
}

mod database_creator {
    use postgres::Connection;

    pub fn create_companies_table(conn: &Connection) {
        conn.execute("
            CREATE TABLE IF NOT EXISTS companies (
                id SERIAL PRIMARY KEY,
                name TEXT UNIQUE
            )",
        &[]).ok().expect("could not create companies table");
    }
}

mod database_dumper {
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
}

mod database_cleaner {
    use postgres::Connection;

    pub fn clear_companies(conn: &Connection) {
        println!("Deleting all companies...");
        conn.execute("DELETE FROM companies", &[]).ok().expect("could not delete companies");
    }

    pub fn drop_companies_table(conn: &Connection) {
        println!("Dropping the companies table...");
        conn.execute("DROP TABLE IF EXISTS companies", &[]).ok().expect("could not drop table companies");
    }
}

mod config {
    use postgres::{Connection, SslMode};

    pub fn database_connection() -> Option<Connection> {
        match Connection::connect(database_url(), &SslMode::None) {
            Ok(connection_option) => Some(connection_option),
            Err(e) => {
                println!("connection error: {:?}", e);
                None
            }
        }
    }

    fn database_url() -> &'static str {
        let database_url: &'static str = "postgresql://dbegin@localhost/watches";
        database_url
    }
}

fn title() {
    println!("\nA Project Combing 3 of my interests, Rust, Postgres and Watches\n");
}

#[cfg(test)]
mod tests {
    use config;
    use database_creator;
    use postgres::Connection;

    #[test]
    fn it_can_be_tested() {
        assert_eq!(true, true);
    }

    #[test]
    fn it_can_find_all_companies() {
        let connection_option = config::database_connection();

        let conn = match connection_option {
            Some(conn) => conn,
            None => {
                println!("Looks like we got a nil connection");
                return;
            }
        };

        before_each(&conn);

        let result = conn.execute("SELECT * FROM companies", &[])
            .ok()
            .expect("there was a problem querying");

        assert_eq!(true, true);


        // let stmt = try!(conn.prepare("SELECT bar, baz FROM foo"));
        // for row in try!(stmt.query(&[])) {
        //     let bar: i32 = row.get(0);
        //     let baz: String = row.get("baz");
        //     println!("bar: {}, baz: {}", bar, baz);
        // }
    }

    fn before_each(conn: &Connection) {
        database_creator::create_companies_table(&conn);
    }
}

