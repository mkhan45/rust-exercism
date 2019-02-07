use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut set = HashSet::new();
    
    let chars: String = candidate.to_lowercase().chars()
        .filter(|c| c.is_alphabetic())
        .collect();

    for c in chars.chars() {
        
        if !set.insert(c) {
            return false;
        }

    }
    return true;
}
