fn main() {
    println!("--- Range Inclusive ---");
    let mut sum_positive = triangle(7);
    println!("Value of sum_positive: {}", sum_positive);

    sum_positive = fold_method(7);
    println!("Value of sum_positive: {}", sum_positive);

    sum_positive = rust_internal(7);
    println!("Value of sum_positive: {}", sum_positive);

    vector_elements();
    into_iterator();
    iter_method();
}

fn triangle(n: i32) -> i32 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    sum
}

fn fold_method(n: i32) -> i32 {
    (1..=n).fold(0, |sum, item| sum + item)
}

fn rust_internal(n: i32) -> i32 {
    n * (n + 1) / 2
}

fn vector_elements() {
    println!("--- Vector's Elements ---");
    let v = vec!["antimony", "arsenic", "aluminum", "selenium"];

    for element in &v {
        println!("{}", element);
    }
}

fn into_iterator() {
    println!("--- Into Iterator ---");
    let v = vec!["antimony", "arsenic", "aluminum", "selenium"];

    let mut iterator = (&v).into_iter();
    while let Some(element) = iterator.next() {
        println!("{}", element);
    }
}

fn iter_method() {
    let v = vec![17, 7, 27, 37, 57, 47];
    let mut iterator = v.iter();
    // the while finish when iterator.next() returns None
    while let Some(value) = iterator.next() {
        println!("value: {}", value);
    }
}