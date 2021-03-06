use anyhow::Result;
use std::io::Write;

pub fn find_matches(content: String, pattern: &str, mut writer: impl Write) -> Result<()> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
        }
    }

    Ok(())
}
