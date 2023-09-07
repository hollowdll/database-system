use crate::client::DatabaseClient;

pub struct Collection<'a> {
    client: &'a DatabaseClient,
    pub name: String,
}

impl<'a> Collection<'a> {
    pub fn new(client: &'a DatabaseClient, name: &str) -> Collection<'a> {
        Collection {
            client,
            name: name.to_string()
        }
    }
}
