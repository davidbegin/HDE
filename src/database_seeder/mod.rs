use postgres::Connection;

mod companies {
    pub fn all() -> Vec<(String, i16)> {
        let first_company = ("Panerai".to_string(), 1860i16);
        let second_company = ("Rolex".to_string(), 1905i16);
        let third_company = ("A. Lange & Sohne".to_string(), 1845i16);
        let fourth_company = ("Audemars Piguet".to_string(), 1875i16);
        let fifth_company = ("IWC Schaffhausen".to_string(), 1868i16);

        vec![
            first_company,
            second_company,
            third_company,
            fourth_company,
            fifth_company
        ]
    }
}

pub fn seed_companies(conn: &Connection) {
    let insert_company = match conn.prepare("INSERT INTO companies (name, year_founded) VALUES ($1, $2)") {
        Ok(insert_name) => insert_name,
        Err(e) => {
            println!("having touble preparing to insert company");
            return;
        }
    };

    for company in companies::all() {
        let (name, year_founded) = company;

        insert_company.execute(&[&name, &year_founded])
            .ok()
            .expect("there was a problem inserting company");
    }
}
