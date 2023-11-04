fn main() {
    let mut numbers = Vec::new();
    for i in 1..=5  {
        numbers.push(i);
    }

    let mut doubled_numbers = Vec::new();
    for number in numbers {
        doubled_numbers.push(number * 2);
    }
}


