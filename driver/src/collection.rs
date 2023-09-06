pub struct Collection {
    pub name: String,
}

impl Collection {
    pub fn new(name: &str) -> Collection {
        Collection {
            name: name.to_string()
        }
    }
}
