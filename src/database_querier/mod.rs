use postgres::Connection;
use type_printer;

pub fn select_all_company_names(conn: &Connection) -> Vec<String> {
    let stmt = match conn.prepare("SELECT * FROM companies") {
        Ok(stmt) => stmt,
        Err(e) => {
            return vec![];
        }
    };

    let result = stmt.query(&[]).ok().expect("dang it");
    let mapped_result = result.iter().map(|i| {
        let name: String = i.get("name");
        name
    }).collect::<Vec<String>>();

    mapped_result
}

pub fn print_all_companies(conn: &Connection) {
    let stmt = match conn.prepare("SELECT * FROM companies") {
        Ok(stmt) => stmt,
        Err(e) => {
            return;
        }
    };

    let result = stmt.query(&[]).ok().expect("dang it");

    println!("\n\tCompanies:\n");

    // TODO: I need to handle the fancy characters being read from the DB
    for row in result {
        let id: i32 = row.get("id");
        let name: String = row.get("name");
        let year_founded: i16 = row.get("year_founded");
        println!("Name: {:?}", name);
        println!("Year Founded: {:?}", year_founded);
        println!("\n-------------------------------------\n");
    }
}
