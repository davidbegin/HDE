extern crate csv;
use postgres::Connection;
use type_printer;

mod movements {
    pub fn all() -> Vec<String> {
        let first_movement = "1030".to_string();
        let second_movement = "P.2003".to_string();
        let third_movement = "L121.1".to_string();
        let fourth_movement = "52010".to_string();
        let fifth_movement = "3120".to_string();
        let sixth_movement  = "3185".to_string();

        vec![
            first_movement,
            second_movement,
            third_movement,
            fourth_movement,
            fifth_movement,
            sixth_movement
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

        let sixth_watch = (
            "16710".to_string(),
            1989,
            "GMT Master II".to_string()
        );

        let seventh_watch = (
            "16570".to_string(),
            1989,
            "Explorer II".to_string()
        );

        // I am still having trouble figuring out how to store watches
        // it seems that this ref should be able to given a year
        //
        //
        // Ref. 16710
        // Production Period: 1989-2007
        // Model Name: Rolex GMT Master II
        // Caliber: 3185 (late models with 3186), 28800A/h, hacking, quickset (24-hour-hand)
        // Pressure proof to 100m/330ft
        // Bracelet: Oyster 78360 and 78790 (Oysterlock), Jubilé 62510
        // Glass: Sapphire crystal
        // Bezel: Anodized aluminum, 120 clicks

        vec![
            first_watch,
            second_watch,
            third_watch,
            fourth_watch,
            fifth_watch,
            sixth_watch,
            seventh_watch
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
    let insert_watch = match conn.prepare("INSERT INTO watches (reference, year, name) VALUES ($1, $2, $3)") {
        Ok(insert_watch) => insert_watch,
        Err(e) => {
            println!("having trouble preparing to insert watch");
            return;
        }
    };

    for watch in watches::all() {
        let (reference, year, name) = watch;

        insert_watch.execute(&[&reference, &year, &name])
            .ok()
            .expect("there was a problem inserting a watch");
    }
}

pub fn seed_movements(conn: &Connection) {
    let insert_movement = match conn.prepare("INSERT INTO movements (name) VALUES ($1)") {
        Ok(insert_movement) => insert_movement,
        Err(e) => {
            println!("having trouble preparing to insert a movement");
            return;
        }
    };

    for name in movements::all() {
        insert_movement.execute(&[&name]).ok().expect("having trouble inserting a movement");
    }
}

pub fn associate_movements_and_watches(conn: &Connection) {
    let watch_movement_pairs = vec![
        ("Milguass".to_string(), "1030".to_string()),
        ("Luminor 1950 10 Days Black Dial Ceramic Black".to_string(), "P.2003".to_string()),
        ("Portugieser Automatic".to_string(), "52010".to_string()),
        ("Royal Oak Stainless Steel".to_string(), "3120".to_string()),
        ("Lange 1".to_string(), "L121.1".to_string()),
        ("GMT Master II".to_string(), "3185".to_string()),
        ("Explorer II".to_string(), "3185".to_string())
    ];

    for pair in watch_movement_pairs {
        let (watch_name, movement_name) = pair;
        let (watch_id, movement_id) = extract_watch_and_movement_ids(&conn, watch_name, movement_name);
        add_movement_to_watch(&conn, movement_id, watch_id);
    }

}

fn extract_watch_and_movement_ids(conn: &Connection, watch_name: String, movement_name: String) -> (i32, i32) {
    let movement_query = match conn.prepare("SELECT * FROM movements WHERE name = $1") {
        Ok(movement_query) => movement_query,
        Err(e) => {
            println!("couldn't find movement");
            return (0, 0);
        }
    };

    let watch_query = match conn.prepare("SELECT * FROM watches WHERE name = $1") {
        Ok(watch_query) => watch_query,
        Err(e) => {
            println!("couldn't find watch");
            return (0, 0);
        }
    };

    let movements = movement_query.query(&[&movement_name]).ok().expect("dang it");
    let movement = movements.iter().next().unwrap();
    let movement_id: i32 = movement.get("id");

    let watches = watch_query.query(&[&watch_name]).ok().expect("dang it");
    let watch = watches.iter().next().unwrap();
    let watch_id: i32 = watch.get("id");

    (watch_id, movement_id)
}

fn add_movement_to_watch(conn: &Connection, movement_id: i32, watch_id: i32) {
    let movement_association = match conn.prepare("UPDATE watches SET movement_id=$1 WHERE id=$2") {
        Ok(movement_association) => movement_association,
        Err(e) => {
            println!("could not associate watch and movement");
            return;
        }
    };

    movement_association.execute(&[&movement_id, &watch_id]).ok().expect("dang it");
}

pub fn seed_from_csv(conn: &Connection) {
    let mut reader = csv::Reader::from_file("./src/database_seeder/data/test_watch_data_1.csv").unwrap();

    // so first I need to find or create the movement and return an i32 the movement id

    let movement_id = find_or_create_movement(&conn, "3185".to_string());
    println!("movement_id: {}", movement_id);

    for record in reader.decode() {
        let (name, reference, year, movement_name): (String, String, i16, String) = record.unwrap();
        create_watch(&conn, name, reference, year);
        let movement_id = find_or_create_movement(&conn, movement_name);
        println!("movement_id: {}", movement_id);
    }
}

pub fn create_watch(conn: &Connection, name: String, reference: String, year: i16) {
    let insert_watch = match conn
        .prepare("INSERT INTO watches (name, reference, year) VALUES ($1, $2, $3)") {
        Ok(insert_watch) => insert_watch,
        Err(e) => {
            println!("having trouble preparing to insert watch");
            return;
        }
    };

    insert_watch.execute(&[&name, &reference, &year])
        .ok()
        .expect("there was a problem inserting a watch");
}

fn find_or_create_movement(conn: &Connection, name: String) -> i32 {
    let find_movement = match conn.prepare("SELECT * FROM movements WHERE name = $1") {
        Ok(v) => v,
        Err(e) => {
            println!("could not prepare to find movement");
            return -1;
        }
    };

    let result = find_movement.query(&[&name]).ok().expect("could not find movement");
    let mut id: i32 = -1;

    for row in result {
        id = row.get("id");
    }

    if id > 0 {
        return id;
    } else {
        return create_movement(&conn, name);
    }
}

fn create_movement(conn: &Connection, name: String) -> i32 {
    let create_movement = match conn.prepare("INSERT INTO movements (name) VALUES ($1)") {
        Ok(v) => v,
        Err(e) => {
            println!("could not prepare to create movement");
            return -1;
        }
    };

    let result = create_movement.execute(&[&name]).ok().expect("could not create movement");
    let id = find_or_create_movement(conn, name);
    id
}
