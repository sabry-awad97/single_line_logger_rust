use std::io::{self, Write};
struct SingleLineLogger<W: Write> {
    stdout: W,
}

impl<W: Write> SingleLineLogger<W> {
    fn new(stdout: W) -> Self {
        SingleLineLogger { stdout }
    }

    fn log(&mut self, msg: &str) -> io::Result<()> {
        // Move the cursor up one line and clear the line
        write!(self.stdout, "\x1B[1F\x1B[0J")?;

        // Write new output
        write!(self.stdout, "{}", msg)?;

        // Flush the output immediately
        self.stdout.flush()?;

        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut logger = SingleLineLogger::new(io::stdout());
    logger.log("Sabry\n")?;
    logger.log("Awad")?;
    Ok(())
}
