// Engine core library
// Prototype phase
// Will be organized and structured better later
// This library will be split into modules later

#![allow(unused)]

/// Configure program
struct Config {}

impl Config {
    pub fn build() -> Config {
        return Config {}
    }
}

struct Database {

}

struct DatabaseTable {

}

struct DatabaseManager {
    connected: bool,
    database_count: u32,
    databases: Vec<Database>,
}

impl DatabaseManager {
    pub fn connect(&mut self) {
        self.connected = true
    }

    pub fn disconnect(&mut self) {
        self.connected = false
    }
}



#[cfg(test)]
mod tests {

}
