use postgres::Connection;
use type_printer;

mod movements {
    pub fn all() -> Vec<String> {
        let first_movement = "1030".to_string();
        let second_movement = "P.2003".to_string();
        let third_movement = "L121.1".to_string();
        let fourth_movement = "52010".to_string();
        let fifth_movement = "3120".to_string();

        vec![
            first_movement,
            second_movement,
            third_movement,
            fourth_movement,
            fifth_movement,
        ]
    }
}

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
        // 1030 movement
        let first_watch = ("6541".to_string(), 1958i16, "Milguass".to_string());

        // P.2003
        let second_watch = (
            "PAM00335".to_string(),
            2015,
            "Luminor 1950 10 Days Black Dial Ceramic Black".to_string()
        );

        // L121.1
        let third_watch = (
            "101.021".to_string(),
            2015,
            "Lange 1".to_string()
        );

        // 5007 is the watch 03 is coloring
        // maybe a good watch to test out the waters of splitting on variation
        // 52010
        let fourth_watch = (
            "IW500703".to_string(),
            2015,
            "Portugieser Automatic".to_string()
        );

        // 3120
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
            println!("having trouble preparing to insert watch");
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

pub fn seed_movements(conn: &Connection) {
    let insert_movement = match conn.prepare("INSERT INTO movements (calibre_id) VALUES ($1)") {
        Ok(insert_movement) => insert_movement,
        Err(e) => {
            println!("having trouble preparing to insert a movement");
            return;
        }
    };

    for calibre_id in movements::all() {
        insert_movement.execute(&[&calibre_id]).ok().expect("having trouble inserting a movement");
    }
}

pub fn associate_movements_and_watches(conn: &Connection) {
    // I need to associate two rows of two different tables
    //
    // let first_movement = "1030".to_string();
    // calibre_id

    // let first_watch = ("6541".to_string(), 1958i16, "Milguass".to_string());
    // reference_id

    // so lets find the movement

    let movement_query = match conn.prepare("SELECT * FROM movements WHERE calibre_id = '1030'") {
        Ok(movement_query) => movement_query,
        Err(e) => {
            println!("couldn't find movement");
            return;
        }
    };

    let watch_query = match conn.prepare("SELECT * FROM watches WHERE name = 'Milguass'") {
        Ok(watch_query) => watch_query,
        Err(e) => {
            println!("couldn't find watch");
            return;
        }
    };

    let movements = movement_query.query(&[]).ok().expect("dang it");
    let movement = movements.iter().next().unwrap();
    let movement_id: i32 = movement.get("id");

    let watches = watch_query.query(&[]).ok().expect("dang it");
    let watch = watches.iter().next().unwrap();
    let watch_id: i32 = watch.get("id");

    let movement_association = match conn.prepare("UPDATE watches SET movement_id=$1 WHERE id=$2") {
        Ok(movement_association) => movement_association,
        Err(e) => {
            println!("could not associate watch and movement");
            return;
        }
    };

    movement_association.execute(&[&movement_id, &watch_id]).ok().expect("dang it");
}
