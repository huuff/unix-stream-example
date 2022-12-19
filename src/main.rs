mod message;
mod unix_read_line;

use std::os::unix::net::UnixStream;
use std::error::Error;
use std::io::{Read, Write};
use std::thread;
use message::message::{Message, MessageReceiver};
use unix_read_line::unix_read_line::ReadLine;

fn printer_thread(mut printer_consumer: UnixStream) -> Result<(), Box<dyn Error>> {
    println!("Printer thread started");
    loop {
        let input = printer_consumer.read_line();
        println!("Printer received: {}", input?);
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let (mut printer_producer, printer_consumer) = UnixStream::pair()?;

    let printer_handle = thread::spawn(|| { printer_thread(printer_consumer).unwrap() });

    let inputs = vec![
        Message::new(String::from("first")),
        Message::new(String::from("second")),
        Message::new(String::from("third")),
        Message::new(String::from("fourth")),
    ];

    for input in inputs {
        printer_producer.send_message(&input)?;
        printer_producer.flush()?;
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    Ok(())
}
