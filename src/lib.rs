pub fn min(a: usize, b: usize) -> usize {
    let smallest = if a < b { a } else { b };
    smallest
}

pub fn max(a: usize, b: usize) -> usize {
    let largest = if a > b { a } else { b };
    largest
}

pub fn min_max(a: usize, b: usize) -> (usize, usize) {
    let smallest = min(a, b);
    let largest = max(a, b);
    (smallest, largest)
}