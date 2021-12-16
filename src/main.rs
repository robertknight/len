use atty::Stream;
use bytesize;
use std::io::Read;

fn main() -> Result<(), std::io::Error> {
    let mut buf: [u8; 4096] = [0; 4096];
    let mut stdin = std::io::stdin();
    let mut len = 0;

    loop {
        match stdin.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => len += n,
            Err(e) => return Err(e),
        }
    }

    let human = atty::is(Stream::Stdout);
    if human {
        println!("{}", bytesize::to_string(len as u64, true));
    } else {
        println!("{}", len);
    }

    Ok(())
}
