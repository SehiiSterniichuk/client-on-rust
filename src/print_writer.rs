use std::io::{self, Write};
use crate::config::request_type::RequestType;
use crate::config::response_type::ResponseType;
use byteorder::{WriteBytesExt, BigEndian};

pub(crate) struct Writer<W: Write> {
    pub out: W,
}

impl<W: Write> Writer<W> {
    pub(crate) fn new(out: W) -> Self {
        Writer { out }
    }

    pub fn println(&mut self, text: &str) -> io::Result<()> {
        writeln!(self.out, "{}", text)?;
        self.out.flush()?;
        // The .flush() method is used to ensure that any buffered data is written to the underlying writer immediately.
        Ok(())
    }

    pub fn println_request(&mut self, request: RequestType) {
        self.println(&request.to_string()).unwrap();
    }

    pub fn println_end(&mut self) {
        self.println("").unwrap()
    }

    pub fn println_response(&mut self, response: ResponseType) {
        self.println(&response.to_string()).unwrap()
    }

    pub fn write_matrix(&mut self, data: &[Vec<f64>], client: String) -> io::Result<()> {
        let start = std::time::Instant::now();
        let size = data.len();
        for i in 0..size {
            for j in 0..size {
                self.out.write_f64::<BigEndian>(data[i][j])?;
            }
            if size >= 2000 && i % 1000 == 0 {
                println!("Writing matrix of the size: {}, row: {}", size, i);
            }
        }
        let finish = start.elapsed().as_micros();
        println!("Time to write: {} {}", finish, client);
        Ok(())
    }
}
