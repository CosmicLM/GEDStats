use std::collections::HashMap;
let mut surname_counts = HashMap::new()
let count = surname_counts.entry(surname).or_insert(0);
*count += 1;
