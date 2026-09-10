use std::fs::File;
use std::io::Result;
fn main() -> Result<()> {
    let file = File::open("C:\\Windows\\Fonts\\segoeui.ttf")?;
    let metadata = file.metadata()?;
    let file_size = metadata.len();
    println!("{} bytes", file_size);
    Ok(())
}
