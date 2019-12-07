fn main() {
    let mut total = 0;
    'search: for input in 153517..630395 {
        let mut remaining = input;
        let mut counts = [0usize; 10];
        let mut last = None;
        while remaining > 0 {
            let digit = remaining % 10;
            if last.map_or(false, |last| digit > last) {
                continue 'search;
            }
            last.replace(digit);
            remaining /= 10;
            counts[digit] += 1;
        }

        if counts.iter().any(|c| *c == 2) {
            println!("{}", input);
            total += 1;
        }
    }

    println!("total: {}", total);
}
