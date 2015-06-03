#[path="../config/mod.rs"]
mod config;
use std::fmt;

pub struct Watch {
    pub id: i32,
    pub name: String
}

impl Watch {
     pub fn new(name: String) -> Watch{
         Watch {
             id: -1,
             name: name
         }
     }

     pub fn find(id: i32) -> Watch {
         let conn = config::database_connection().unwrap();

         let stmt = conn.prepare("SELECT * FROM watches WHERE id = $1")
             .ok()
             .expect("could not prepare to find the watch");

         let mut rows = stmt.query(&[&id]).ok().expect("could not find watch");

         let row = rows.iter().next().unwrap();
         let id: i32 = row.get("id");
         let name: String = row.get("name");

         Watch {
             id: id,
             name: name
         }
     }

     pub fn save(&self) -> i32 {
         let conn = config::database_connection().unwrap();

         let stmt = conn.prepare("INSERT INTO watches (name) VALUES ($1) RETURNING id")
             .ok()
             .expect("could not prepare to insert into watches");

         let mut rows = stmt.query(&[&self.name]).ok().expect("could not query");

         let row = rows.iter().next().unwrap();
         let id: i32 = row.get("id");
         id
     }
}

impl fmt::Debug for Watch{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for Watch{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Drop for Watch {
    fn drop(&mut self) {
        // println!("Watch {} deallocated", self);
    }
}
