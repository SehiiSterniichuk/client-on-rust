use crate::config::response_type::ResponseType;
use byteorder::{ReadBytesExt, LittleEndian};
use std::net::{TcpStream};
use std::io::{BufRead, BufReader};

pub(crate) struct BufferedReader<'a> {
    pub reader: BufReader<&'a TcpStream>,
}

impl<'a> BufferedReader<'a> {
    pub(crate) fn new(reader: BufReader<&'a TcpStream>) -> Self {
        BufferedReader { reader }
    }

    pub fn parse_long(&mut self, prefix: &str) -> i64 {
        let read_line = self.read_line();
        let substring = read_line.trim_start_matches(prefix);
        let res = match substring.parse::<i64>() {
            Ok(parsed_value) => parsed_value,
            Err(e) => {
                println!("{}", e);
                return -1;
            }
        };
        return res;
    }

    pub fn read_line(&mut self) -> String {
        let mut read_line = String::new();
        self.reader.read_line(&mut read_line).expect("Failed to read line");
        let string = read_line.trim().to_string();
        return string;
    }

    pub fn get_response_type(&mut self) -> Result<ResponseType, String> {
        let mut line = String::new();
        match self.reader.read_line(&mut line) {
            Ok(bytes_read) => {
                if bytes_read > 0 {
                    if line.trim() == ResponseType::OK.to_string() {
                        Ok(ResponseType::OK)
                    } else {
                        Ok(ResponseType::BadRequest)
                    }
                } else {
                    Err(String::from("No data read from the stream."))
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn read_error(&mut self) {
        let mut line: String = String::new();
        match self.reader.read_line(&mut line) {
            Ok(bytes_read) => {
                if bytes_read > 0 {
                    eprintln!("Server {}", line)
                } else {
                    eprintln!("No error message")
                }
            }
            Err(e) => {
                eprintln!("{}", e.to_string())
            }
        }
    }

    pub fn read_matrix(&mut self, size: usize, client: &str) -> std::io::Result<Vec<Vec<f64>>> {
        let mut array = vec![vec![0.0; size]; size];
        let start = std::time::Instant::now();
        for i in 0..size {
            for j in 0..size {
                let v = self.reader.read_f64::<LittleEndian>()?;
                array[i][j] = v;
            }
            if size >= 2000 && i % 1000 == 0 {
                println!("Reading matrix of the size: {}, row: {}", size, i);
            }
        }
        let finish = start.elapsed().as_micros();
        println!("Time to read: {} {}", finish, client);
        Ok(array)
    }
}
