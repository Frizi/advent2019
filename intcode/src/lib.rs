mod io;
mod machine;

pub use io::*;
pub use machine::*;

pub type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn parse_intcode(bytes: &[u8]) -> DynResult<Vec<Word>> {
    bytes
        .split(|c| *c as char == ',')
        .map(|slice| -> DynResult<Word> { Ok(std::str::from_utf8(slice)?.parse()?) })
        .collect()
}
