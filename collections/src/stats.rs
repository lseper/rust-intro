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

// fn mode(arr: &mut Vec<i32>) -> &i32 {

// }
