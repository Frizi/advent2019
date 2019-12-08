fn main() -> std::io::Result<()> {
    let data = std::fs::read("day8-input.txt")?;

    const WIDTH: usize = 25;
    const HEIGHT: usize = 6;

    let mut image = [2u8; WIDTH * HEIGHT];

    let mut min_layer = None;
    for layer in data.chunks(WIDTH * HEIGHT) {
        let mut zero_digits = 0;
        let mut one_digits = 0;
        let mut two_digits = 0;
        for (idx, digit) in layer.iter().enumerate() {
            match *digit as char {
                '0' => {
                    if image[idx] == 2 {
                        image[idx] = 0;
                    }
                    zero_digits += 1;
                },
                '1' => {
                    if image[idx] == 2 {
                        image[idx] = 1;
                    }
                    one_digits += 1;
                },
                '2' => {
                    two_digits += 1;
                },
                _ => {},
            }
        }

        let replace = min_layer.map_or(true, |(zero, _)| zero > zero_digits);
        if replace {
            min_layer = Some((zero_digits, one_digits * two_digits));
        }
    }

    println!("Checksum: {:?}", min_layer);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let character = match image[y * WIDTH + x] {
                0 => ' ',
                1 => '#',
                2 => '.',
                _ => panic!("unknown image value"),
            };
            print!("{}", character);
        }
        println!("");
    }
    Ok(())
}
