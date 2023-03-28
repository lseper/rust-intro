use std::collections::HashMap;

pub fn median(arr: &mut Vec<i32>) -> Option<f32> {
    arr.sort();
    if arr.len() > 0 {
        if arr.len() % 2 == 0 {
            let median = (arr[(arr.len() / 2 - 1)] + arr[arr.len() / 2]) as f32 / 2.0;
            return Some(median);
        } else {
            return Some(arr[arr.len() / 2] as f32);
        }
    }
    return None;
}

pub fn mode(arr: &mut Vec<i32>) -> i32 {
    let mut counts = HashMap::new();
    for num in arr {
        let count = counts.entry(*num).or_insert(0);
        *count += 1;
    }

    let mut max_value = 0;
    for (key, count) in counts.iter() {
        if *count > max_value {
            max_value = *key;
        }
    }
    return max_value;
}
