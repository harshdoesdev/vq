use std::collections::{HashMap, HashSet};

pub fn term_frequency(document: &str) -> HashMap<String, f64> {
    let lowercased = document.to_lowercase();
    let tokens = vq::tokenizer::tokenize(&lowercased);
    let total_terms = tokens.len() as f64;

    tokens
        .iter()
        .fold(HashMap::new(), |mut tf, token| {
            *tf.entry(token.to_string()).or_insert(0.0) += 1.0;
            tf
        })
        .into_iter()
        .map(|(token, freq)| (token, freq / total_terms))
        .collect()
}

pub fn inverse_document_frequency(documents: &[vq::Document]) -> HashMap<String, f64> {
    let mut idf: HashMap<String, f64> = HashMap::new();
    let num_documents = documents.len() as f64;

    for document in documents {
        let tokens: HashSet<String> = vq::tokenizer::tokenize(&document.content)
            .into_iter()
            .map(|s| s.to_lowercase())
            .collect();

        for token in tokens {
            *idf.entry(token.to_string()).or_insert(0.0) += 1.0;
        }
    }

    for (_, freq) in idf.iter_mut() {
        *freq = (num_documents / *freq).ln();
    }

    idf
}

pub fn tf_idf(document: &str, idf: &HashMap<String, f64>) -> HashMap<String, f64> {
    let tf = term_frequency(document);

    tf.iter()
        .fold(HashMap::new(), |mut tf_idf, (term, tf_value)| {
            let idf_value = idf.get(term).unwrap_or(&0.0);
            tf_idf.insert(term.to_string(), tf_value * idf_value);

            tf_idf
        })
}

pub fn cosine_similarity(vec1: &HashMap<String, f64>, vec2: &HashMap<String, f64>) -> f64 {
    let dot_prod = dot_product(vec1, vec2);
    let mag_vec1 = magnitude(vec1);
    let mag_vec2 = magnitude(vec2);

    if mag_vec1 == 0.0 || mag_vec2 == 0.0 {
        return 0.0;
    }

    dot_prod / (mag_vec1 * mag_vec2)
}

fn dot_product(vec1: &HashMap<String, f64>, vec2: &HashMap<String, f64>) -> f64 {
    vec1.iter().fold(0.0, |acc, (term, tfidf)| {
        acc + tfidf * vec2.get(term).unwrap_or(&0.0)
    })
}

fn magnitude(vec: &HashMap<String, f64>) -> f64 {
    vec.values().map(|x| x.powi(2)).sum::<f64>().sqrt()
}
