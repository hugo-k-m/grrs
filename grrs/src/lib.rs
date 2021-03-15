use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

pub fn find_matches(content: BufReader<File>, pattern: &str, mut writer: impl Write) -> Result<()> {
    for content_line in content.lines() {
        let line = content_line?;

        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use std::io::BufReader;
    #[test]
    fn find_match() -> Result<()> {
        let mut result = Vec::new();
        let file = File::open("unit_test.txt")?;
        let reader = BufReader::new(file);

        find_matches(reader, "lorem", &mut result)?;
        assert_eq!(&result, "lorem ipsum\n".as_bytes());

        Ok(())
    }
}
