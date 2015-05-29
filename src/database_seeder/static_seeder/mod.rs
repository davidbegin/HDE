mod seeds;
use postgres::Connection;

pub fn seed_companies(conn: &Connection) {
    let insert_company = match conn
        .prepare("INSERT INTO companies (name, year_founded) VALUES ($1, $2)") {
        Ok(insert_name) => insert_name,
        Err(e) => {
            println!("having touble preparing to insert company");
            return;
        }
    };

    for company in seeds::companies::all() {
        let (name, year_founded) = company;

        insert_company.execute(&[&name, &year_founded])
            .ok()
            .expect("there was a problem inserting company");
    }
}

pub fn seed_watches(conn: &Connection) {
    let insert_watch = match conn
        .prepare("INSERT INTO watches (reference, year, name) VALUES ($1, $2, $3)") {
        Ok(insert_watch) => insert_watch,
        Err(e) => {
            println!("having trouble preparing to insert watch");
            return;
        }
    };

    for watch in seeds::watches::all() {
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

    for name in seeds::movements::all() {
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


