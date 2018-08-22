pub fn map(input: Vec<i32>, mapper: fn(i32) -> i32) -> Vec<i32> {
    let mut acc = Vec::new();
    for val in input {
        acc.push(mapper(val));
    }
    acc
}
