use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut map: HashMap<String, u32> = HashMap::new();
    let mut filtered: String = words.to_lowercase();
    filtered.retain(|c| c != '\"' && c != ':');
    println!("{}", filtered);

    filtered.split(|c| c == ' ' || c == '\n' || c == ',')
            .filter(|word| !word.is_empty())
            .for_each(|word| {
                let mut word: String = word.to_string();
                word.retain(|c| c.is_alphanumeric() || c == '\'');


                    if map.contains_key(&word) {
                        *map.get_mut(&word).unwrap() += 1; //get_mut returns an Option, Option.unwrap() returns the value
                    } else {
                        map.insert(word.to_string(), 1u32);
                    }
            });

    println!("{:?}", map);
    map
}
