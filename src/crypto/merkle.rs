use crate::crypto::hash::sha256;

pub fn merkle_root(mut hashes: Vec<String>) -> String {
    if hashes.is_empty() {
        return sha256(&[]);
    }

    while hashes.len() > 1 {
        if hashes.len() % 2 != 0 {
            hashes.push(hashes.last().unwrap().clone());
        }

        hashes = hashes
            .chunks(2)
            .map(|pair| {
                let combined = format!("{}{}", pair[0], pair[1]);
                sha256(combined.as_bytes())
            })
            .collect();
    }

    hashes[0].clone()
}
