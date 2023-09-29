use driver::document::DocumentModel;

/// Displays a document listing its data.
pub fn display_document(document: &DocumentModel) {
    println!("_id: {}", document.id());

    for (key, value) in &document.data {
        println!("{}: {}", key, value);
    }
}

/// Displays a list of documents and their data.
pub fn display_document_list(documents: &Vec<DocumentModel>) {
    for document in documents {
        println!("{{");
        println!("  _id: {}", document.id);

        for (key, value) in &document.data {
            println!("  {}: {}", key, value);
        }

        println!("}}");
    }
}