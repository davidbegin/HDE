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

mod watches {
    pub fn all() -> Vec<(String, i16, String)> {
        let first_watch = ("6541".to_string(), 1958i16, "Milguass".to_string());
        let second_watch = (
            "PAM00335".to_string(),
            2015,
            "Luminor 1950 10 Days Black Dial Ceramic Black".to_string()
        );
        let third_watch = (
            "101.021".to_string(),
            2015,
            "Lange 1".to_string()
        );
        let fourth_watch = (
            "IW500703".to_string(),
            2015,
            "Portugieser Automatic".to_string()
        );
        let fifth_watch = (
            "15400ST.OO.1220ST.01".to_string(),
            2015,
            "Royal Oak Stainless Steel".to_string()
        );

        vec![
            first_watch,
            second_watch,
            third_watch,
            fourth_watch,
            fifth_watch
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

pub fn seed_watches(conn: &Connection) {
    let insert_watch = match conn.prepare("INSERT INTO watches (reference_id, year, name) VALUES ($1, $2, $3)") {
        Ok(insert_watch) => insert_watch,
        Err(e) => {
            println!("haveing trouble preparing to insert watch");
            return;
        }
    };

    for watch in watches::all() {
        let (reference_id, year, name) = watch;

        insert_watch.execute(&[&reference_id, &year, &name])
            .ok()
            .expect("there was a problem inserting a watch");
    }
}
