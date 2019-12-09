use intcode::*;

fn main() -> DynResult<()> {
    let boost = parse_intcode(&std::fs::read("day9-input.txt")?)?;

    let mut machine = Machine::new(boost);
    machine.execute(&mut StdIo);

    Ok(())
}
