use driver::document::{
    DocumentModel,
    DataType,
};

pub fn create_test_document() -> DocumentModel {
    let mut person = DocumentModel::new();
    person.data.insert("first_name".to_string(), DataType::Text("John".to_string()));
    person.data.insert("last_name".to_string(), DataType::Text("Smith".to_string()));
    person.data.insert("age".to_string(), DataType::Int32(42));

    return person;
}