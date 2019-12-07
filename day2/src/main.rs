use intcode::*;

fn main() -> DynResult<()> {
    let file = std::fs::read("day2-input.txt")?;
    let mem = parse_intcode(&file)?;

    for noun in 0..100 {
        for verb in 0..100 {
            let mut mem = mem.clone();
            mem[1] = noun;
            mem[2] = verb;

            let mut m = Machine::new(mem);
            m.execute(&mut StdIo);

            let output = m.read_mem_at(0);

            if output == 19690720 {
                println!("noun: {}, verb: {}", noun, verb);
                return Ok(());
            }
        }
    }

    Ok(())
}
