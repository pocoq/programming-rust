use std::fs::File;
use std::io::Result;
use std::io::Write;

fn say_hello<W: Write>(out: &mut W) -> Result<()> {
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

pub struct Sink;

impl Write for Sink {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

pub fn use_sink() {
    let mut out = Sink;
    out.write_all(b"Hello world! \n")
        .expect("Failed to write to sink");
}
