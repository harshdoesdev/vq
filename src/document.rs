#[derive(Debug, Clone)]
pub struct Document {
    pub id: String,
    pub content: String,
}

impl Document {
    pub fn new(id: String, content: String) -> Self {
        Document { id, content }
    }
}
