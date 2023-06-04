use std::collections::HashMap;

fn mean_median_mode(itr: impl IntoIterator<Item = i32>) -> (f64, i32, i32) {
    let mut sum = 0.0;
    let mut values = Vec::new();
    let mut counts = HashMap::new();
    for x in itr {
        sum += f64::from(x);
        values.push(x);
        *counts.entry(x).or_default() += 1
    }
    let mean = sum / values.len() as f64;
    values.sort();
    let median = values[(values.len() - 1) / 2];
    let (mode, _) = counts.iter().max_by_key(|(_, v)| *v).unwrap_or((&0, &0));
    (mean, median, *mode)
}

fn main() {
    let input = [1, 2, 3, 4, 4, 45, 2, 2];
    let x = mean_median_mode(input);
    println!("{:?}", x);
}
