mod message;
mod unix_read_line;

use std::os::unix::net::UnixStream;
use std::error::Error;
use std::io::Write;
use std::thread;
use message::message::{Message, MessageReceiver};
use unix_read_line::unix_read_line::ReadLine;

fn length_prepender_thread(mut prepender_consumer: UnixStream, mut prepender_producer: UnixStream) -> Result<(), Box<dyn Error>> {
    println!("Prepender thread started");
    loop {
        let input = prepender_consumer.read_line()?;
        let length_prepended_input = format!("{} {}", input.len(), input);
        prepender_producer.send_message(&Message::new(length_prepended_input))?;
    }
}

fn printer_thread(mut printer_consumer: UnixStream) -> Result<(), Box<dyn Error>> {
    println!("Printer thread started");
    loop {
        let input = printer_consumer.read_line();
        println!("Printer received: {}", input?);
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let (printer_producer, printer_consumer) = UnixStream::pair()?;
    let (mut prepender_producer, prepender_consumer) = UnixStream::pair()?;

    let _printer_handle = thread::spawn(|| {
        printer_thread(printer_consumer).unwrap() 
    });
    let _prepender_handle = thread::spawn(|| { 
        length_prepender_thread(prepender_consumer, printer_producer).unwrap()
    });

    let inputs = vec![
        Message::new(String::from("first")),
        Message::new(String::from("second")),
        Message::new(String::from("third")),
        Message::new(String::from("fourth")),
    ];

    for input in inputs {
        prepender_producer.send_message(&input)?;
        prepender_producer.flush()?;
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    Ok(())
}
