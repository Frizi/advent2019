use intcode::*;

fn main() -> DynResult<()> {
    let file = std::fs::read("day5-input.txt")?;
    let mem = parse_intcode(&file)?;

    let mut m = Machine::new(mem);
    m.execute(&mut StdIo);
    Ok(())
}
