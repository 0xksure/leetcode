struct Solution;

impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let mut dd = dictionary.clone();
        dd.sort();
        sentence
            .split(" ")
            .map(|f| match dd.iter().find(|&root| f.starts_with(root)) {
                Some(root) => root.clone(),
                None => f.to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}
