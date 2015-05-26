use postgres::{Connection, SslMode};

pub fn database_connection() -> Option<Connection> {
    match Connection::connect(database_url(), &SslMode::None) {
        Ok(connection_option) => Some(connection_option),
        Err(e) => {
            println!("connection error: {:?}", e);
            None
        }
    }
}

fn database_url() -> &'static str {
    let database_url: &'static str = "postgresql://dbegin@localhost/watches";
    database_url
}
