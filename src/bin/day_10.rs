use std::error::Error;
use crate::util::parsing;

mod util;


type Num = i32;


fn main() -> Result<(), Box<dyn Error>> {
    let lines = parsing::file_into_vec("files/day_10_input.txt")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    fn basic() {

    }
}
