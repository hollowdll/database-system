use engine::DocumentInputDataField;

// Creates input data for document tests
pub fn create_document_input_data() -> Vec<DocumentInputDataField> {
    let mut data = Vec::new();
    data.push(DocumentInputDataField::new("first_name", "Text", "John"));
    data.push(DocumentInputDataField::new("last_name", "Text", "Smith"));
    data.push(DocumentInputDataField::new("age", "Int32", "42"));

    return data;
}