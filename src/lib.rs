extern crate self as vq;

mod document;
mod text_proc;
mod tokenizer;
pub use document::Document;

pub fn search(query: &str, documents: &[Document]) -> Vec<(String, f64)> {
    let idf = text_proc::inverse_document_frequency(documents);
    let query_tfidf = text_proc::tf_idf(query, &idf);

    let mut results: Vec<(String, f64)> = documents
        .iter()
        .map(|doc| {
            let doc_tfidf = text_proc::tf_idf(&doc.content, &idf);
            let similarity = text_proc::cosine_similarity(&query_tfidf, &doc_tfidf);
            (doc.id.clone(), similarity)
        })
        .collect();

    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    results
}
