pub struct Database {
    pub name: String,
}

impl Database {
    pub fn new(name: &str) -> Database {
        Database {
            name: name.to_string()
        }
    }
}