struct MapSum {
    values: Vec<i32>,
    keys: Vec<String>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MapSum {
    fn new() -> Self {
        MapSum {
            keys: Vec::new(),
            values: Vec::new(),
        }
    }

    fn insert(&mut self, key: String, val: i32) {
        // check if key present
        match self.keys.iter().position(|k| k.eq(&key)) {
            Some(idx) => self.values[idx] = val,
            None => {
                self.keys.push(key);
                self.values.push(val);
            }
        }
    }

    fn sum(&self, prefix: String) -> i32 {
        self.keys
            .iter()
            .zip(self.values.iter())
            .filter(|(k, v)| k.starts_with(&prefix))
            .fold(0, |acc, (_, val)| acc + val)
    }
}
