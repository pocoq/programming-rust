use std::fs::File;
use std::io::Write;

fn say_hello<W: Write>(out: &mut W) -> Result<(), std::io::Error> {
    out.write_all(b"Hello, world!\n")?;
    out.flush()
}

pub fn write_to_file() {
    let mut local_file = File::create("hello.txt").expect("Failed to create file");
    match say_hello(&mut local_file) {
        Ok(_) => println!("Successfully wrote to file"),
        Err(e) => println!("Error writing to file: {}", e),
    };

    let mut bytes = vec![];
    say_hello(&mut bytes).expect("Failed to write to bytes");
    println!("Bytes written: {:?}", bytes);
}
