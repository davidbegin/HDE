#![allow(unused_imports, unused_variables, dead_code)]

extern crate postgres;
extern crate type_printer;
mod database_creator;
mod database_seeder;
mod database_cleaner;
mod database_dumper;
mod database_querier;
mod config;

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

    database_cleaner::drop_companies_table(&conn);
    database_creator::create_companies_table(&conn);
    database_seeder::seed_companies(&conn);
    // database_cleaner::clear_companies(&conn);
    // database_dumper::companies(&conn);
    database_querier::print_all_companies(&conn);
}

fn title() {
    println!("\nA Project Combing 3 of my interests, Rust, Postgres and Watches\n");
}

#[cfg(test)]
mod tests {
    use config;
    use database_creator;
    use database_querier;
    use postgres::Connection;
    use type_printer;

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
        let names = database_querier::select_all_company_names(&conn);
        assert_eq!(
            names,
            vec!["Panerai", "Rolex", "A. Lange & Sohne", "Audemars Piguet", "IWC Schaffhausen"]
        );
    }

    fn before_each(conn: &Connection) {
        database_creator::create_companies_table(&conn);
    }
}

