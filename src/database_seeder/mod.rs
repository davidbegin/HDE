extern crate csv;
use postgres::{Connection, Error, FromSql, SslMode};
use type_printer;
mod static_seeder;
use config;

pub fn seed_from_csv(conn: &Connection) {
    let mut reader = csv::Reader::from_file("./src/database_seeder/data/2015_rolex.csv").unwrap();

    let rolex_id = rolex_finder(&conn);

    for record in reader.decode() {
        let (name, reference, year, movement_name): (String, String, i16, String) = record.unwrap();
        let watch_id = create_watch(&conn, name, reference, year, rolex_id);
        let movement_id = find_or_create_movement(&conn, movement_name);
        add_movement_to_watch(conn, movement_id, watch_id);
    }
}

pub fn create_watch(conn: &Connection, name: String, reference: String, year: i16, company_id: i32) -> i32 {
    let stmt = conn.prepare("INSERT INTO watches (name, reference, year, company_id) VALUES ($1, $2, $3, $4) RETURNING id")
        .ok().expect("there was a problem preparing to insert a watch");

    let result = stmt.query(&[&name, &reference, &year, &company_id]).ok().expect("there was a problem inserting a watch");

    result.iter().next().unwrap().get("id")
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

fn rolex_creator(conn: &Connection) -> i32 {
    let create_rolex_stmt = match conn.execute("INSERT INTO companies (name) VALUES ($1)", &[&"Rolex".to_owned()]) {
        Ok(v) => v,
        Err(e) => {
            return -1;
        }
    };

    10
}

fn rolex_finder(conn: &Connection) -> i32 {
    let rolex_id_query = match conn.prepare("SELECT * FROM companies WHERE name ~* 'rolex'") {
      Ok(v) => v,
      Err(e) => {
        return -1;
      }
    };

    let id: i32 = match rolex_id_query.query(&[]).ok().expect("darnit").iter().next() {
        Some(e) => e.get("id"),
        None => {
            return -1;
        }
    };

    id
}




#[test]
fn it_is_real() {
    assert_eq!(true, true);

    // I want to delete all delete all watches
    //
    // I then want to assert there are no watches

    let conn = match config::database_connection() {
        Some(conn) => conn,
        None => {
            println!("Looks like we got a nil connection");
            return;
        }
    };

    let company_count = match conn.prepare("SELECT count(*) FROM companies") {
        Ok(v) => v,
        Err(e) => {
            return;
        }
    };

    let rows = company_count.query(&[]).ok().expect("could not select the company count");
}
