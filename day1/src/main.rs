type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> DynResult<()> {
    let input = std::fs::read("day1-input.txt")?;
    let total: DynResult<u32> = input
        .split(|t| *t as char == '\n')
        .map(|line| -> DynResult<u32> {
            Ok(std::str::from_utf8(line)?
                .parse::<u32>()
                .map(fuel_for_module_adjusted)
                .unwrap_or(0))
        })
        .sum();

    println!("total: {}", total?);
    Ok(())
}

fn fuel_for_mass(mass: u32) -> u32 {
    (mass / 3).max(2) - 2
}

fn fuel_for_module_adjusted(initial_mass: u32) -> u32 {
    let mut total = 0;
    let mut extra_mass = initial_mass;
    loop {
        extra_mass = fuel_for_mass(extra_mass);
        total += extra_mass;
        if extra_mass == 0 {
            return total;
        }
    }
}
