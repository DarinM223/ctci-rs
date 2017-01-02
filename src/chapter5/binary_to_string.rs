use std::fmt::{Error, Write};

pub fn print_binary(num: f64) -> Result<String, Error> {
    if num >= 1. || num <= 0. {
        return Ok("ERROR".to_string());
    }

    let mut binary = String::new();
    write!(&mut binary, ".")?;

    let mut num = num;
    while num > 0. {
        if binary.len() >= 32 {
            return Ok("ERROR".to_string());
        }

        let r = num * 2.;
        if r >= 1. {
            write!(&mut binary, "{}", 1)?;
            num = r - 1.;
        } else {
            write!(&mut binary, "{}", 0)?;
            num = r;
        }
    }

    Ok(binary)
}
