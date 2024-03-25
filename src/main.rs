use vq::Document;

fn main() {
    let documents = vec![
        Document::new("test-1".to_string(), "The cat sat on the mat.".to_string()),
        Document::new("test-2".to_string(), "The dog chased the cat.".to_string()),
        Document::new(
            "test-3".to_string(),
            "The mat was sat on by the cat.".to_string(),
        ),
    ];

    let query = "cat chased";

    let results = vq::search(query, &documents);

    println!("Search Results:");
    for (doc_id, similarity) in results {
        println!("Document {}: Similarity: {}", doc_id, similarity);
    }
}
