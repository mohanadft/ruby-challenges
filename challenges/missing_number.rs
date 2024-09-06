use std::collections::HashSet;

fn main() {
    // Test the function here
}

/**
 * Assuming arr => [1..n];
 */

fn missing_number(arr: &[usize]) -> usize {
    let len = arr.len() + 1;
    let mut complete_set: HashSet<usize> = (1..=len).collect();

    for n in arr {
        complete_set.remove(n);
    }

    if !complete_set.is_empty() {
        return *complete_set.iter().next().unwrap();
    }

    0
}
