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

pub fn print_all_watches(conn: &Connection) {
    let stmt = match conn.prepare("SELECT * FROM watches") {
        Ok(stmt) => stmt,
        Err(e) => {
            println!("There was an Error: {:?}", e);
            return;
        }
    };

    let result = stmt.query(&[]).ok().expect("dang it");

    for row in result {
        let id: i32              = row.get("id");
        let name: String         = row.get("name");
        let reference: String = row.get("reference");
        let year: i16            = row.get("year");
        println!("Reference: {}", reference);
        println!("Name: {}", name);
        println!("Year: {}", year);
        println!("\n-------------------------------------\n");
    }

}

pub fn print_all_movements(conn: &Connection) {
    let stmt = match conn.prepare("SELECT * FROM movements") {
        Ok(stmt) => stmt,
        Err(e) => {
            println!("There was an error: {:?}", e);
            return;
        }
    };

    let result = stmt.query(&[]).ok().expect("dang it");

    for row in result {
        let id: i32 = row.get("id");
        let name: String = row.get("name");

        println!("Caliber: {:?}", name);
    }
}

// So I want this to return a result, so I can use
// try to propogate up
fn select_all_companies_2() -> Result<String, String> {
    // Ok("Dummy Data".to_string())
    Err("Dummy Data".to_string())
}

#[cfg(test)]
mod tests {
    use super::select_all_companies_2;

    // #[test]
    // fn select_all_companies_2_returns_a_result() {
    //     assert_eq!(select_all_companies_2().unwrap(), "Dummy Data");
    // }

    // This will not work, becuase expect(propagates the error)
    // #[test]
    // fn select_all_companies_2_returns_a_result() {
    //     assert_eq!(
    //         select_all_companies_2().ok().expect("Dummy Data"),
    //         "Dummy Data"
    //     );
    // }

    // so how can I work with Err, when asserting
    #[test]
    fn errors_can_matched_and_asserted_against() {
        let matched_error = match select_all_companies_2() {
            Ok(e) => e,
            Err(e) => "Rescued Error Data".to_string()
        };

        assert_eq!(matched_error, "Rescued Error Data");
    }

}
