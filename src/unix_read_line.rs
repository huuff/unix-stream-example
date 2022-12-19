pub mod unix_read_line {
    use std::io::{Error, Read};
    use std::os::unix::net::UnixStream;

    pub trait ReadLine {
        fn read_line(&mut self) -> Result<String, Error>;
    }

    impl ReadLine for UnixStream {
        fn read_line(&mut self) -> Result<String, Error> {
            let mut line = String::new();
            let mut current_char_buf = [0u8; 1];
            loop {
                self.read_exact(&mut current_char_buf)?;
                if current_char_buf[0] == b'\n' {
                    break;
                } else {
                    line.push(current_char_buf[0] as char);
                }
            }
            Ok(line)
        }
    }
}
