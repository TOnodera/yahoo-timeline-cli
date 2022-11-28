use anyhow::{Context, Result};

fn main() -> Result<()> {
    // let args = Cli::parse();
    let path = "test.txt";
    let content =
        std::fs::read_to_string(path).with_context(|| format!("could not read file `{}`", path))?;

    println!("file content: {}", content);
    Ok(())
}
