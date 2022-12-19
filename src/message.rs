pub mod message {
    use std::{os::unix::net::UnixStream, io::Write};

    pub struct Message {
        contents: String,
    }

    impl Message {
        pub fn new(contents: String) -> Self {
            Message { contents }
        }

        fn length(&self) -> usize {
            self.contents.len()
        }
    }

    pub trait MessageReceiver {
        fn send_message(&mut self, msg: &Message) -> Result<(), std::io::Error>;
    }

    impl MessageReceiver for UnixStream {
        fn send_message(&mut self, msg: &Message) -> Result<(), std::io::Error> {
            self.write_all(format!("{}{}", msg.length(), msg.contents).as_bytes())
        }
    }
}