use std::io;
use std::process;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    println!("You entered: {}", buffer.trim());

    if buffer.starts_with("quit") {
        process::exit(0);
    } else {
        return Err(io::ErrorKind::InvalidInput.into())
    }
}
