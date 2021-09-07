use std::io;
use std::io::Write;
use std::fs::File;
use std::env;

fn main() -> io::Result<()> {
    let args = env::args().skip(1);

    for path in args {
        let f = File::open(path)?;
        let mut reader = io::BufReader::new(f);
        let mut writer = io::stdout();

        io::copy(&mut reader, &mut writer)?;
        writer.write(b"\n")?;
    }
    Ok(())
}