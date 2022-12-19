use std::os::unix::net::UnixStream;
use std::error::Error;
use std::io::{Read, Write};
use std::thread;

fn printer_thread(mut printer_consumer: UnixStream) -> Result<(), Box<dyn Error>> {
    println!("Printer thread started");
    loop {
        let mut length_buffer = [0u8; 1];
        printer_consumer.read_exact(&mut length_buffer)?;
        
        let length: usize = std::str::from_utf8(&length_buffer)?.parse()?;

        let mut input_buffer = vec![0u8; length];
        printer_consumer.read_exact(&mut input_buffer)?;
        let actual_input = String::from_utf8_lossy(&input_buffer);
        println!("Printer received: {}", actual_input);
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let (mut printer_producer, printer_consumer) = UnixStream::pair()?;

    let printer_handle = thread::spawn(|| { printer_thread(printer_consumer).unwrap() });

    let inputs = vec![
        "5first",
        "6second",
        "5third",
        "6fourth",
    ];

    for input in inputs {
        printer_producer.write_all(input.as_bytes())?;
        printer_producer.flush()?;
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    Ok(())
}
