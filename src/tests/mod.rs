#[cfg(test)]
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
