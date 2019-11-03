use atty::Stream;
use bytesize;
use std::io::Read;

fn main() {
    let len = std::io::stdin().bytes().count();
    let human = atty::is(Stream::Stdout);

    if human {
        println!("{}", bytesize::to_string(len as u64, true));
    } else {
        println!("{}", len);
    }
}
