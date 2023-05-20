use std::error::Error;
use std::io::{BufRead, BufReader};
use std::net::{TcpStream};
use std::{fmt};
use std::str::FromStr;
use crate::config::request_type::RequestType;
use crate::lab1::matrix::Matrix;
use crate::print_writer::Writer;
use crate::buffered_reader::BufferedReader;
use crate::prefix::{ID, SIZE, THREADS, TIME};
use std::thread;
use std::time::Duration;
use crate::config::response_type::ResponseType::{BadRequest, OK};
use crate::config::status::Status;
use crate::custom_error::CustomError;

pub(crate) struct Client {
    host: String,
    port: u16,
    size: i32,
    id: i32,
    thread_number: i32,
    task_id: i64,
}

pub struct ExecutionResult {
    matrix: Matrix,
}

impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Client {{ id={}, taskId={}, size={}, threadNumber={} }}",
            self.id, self.task_id, self.size, self.thread_number
        )
    }
}


impl Client {
    pub(crate) fn new(host: &str, port: u16, size: i32, id: i32, thread_number: i32, task_id: i64) -> Client {
        Client {
            host: String::from(host),
            port,
            size,
            id,
            thread_number,
            task_id,
        }
    }

    fn print_matrix(&self, message: &str, _matrix: &Matrix) {
        println!("{}", message);
        // _matrix.print();
    }

    pub(crate) fn run(&mut self) {
        match TcpStream::connect((self.host.as_str(), self.port)) {
            Ok(stream) => {
                let reader = BufReader::new(&stream);
                let mut print_writer: Writer<&TcpStream> = Writer::new(&stream);
                let mut buffered_reader: BufferedReader = BufferedReader::new(reader);
                self.work(&mut buffered_reader, &mut print_writer);
                /*
                The &mut syntax is used to create a mutable reference.
                It indicates that the function receiving the reference can mutate the value it refers to.
                 By passing &mut buffered_reader and &mut print_writer to the work function,
                 you are allowing that function to modify the buffered_reader and print_writer objects in the calling code.
                */
            }
            Err(e) => {
                eprintln!("Failed to connect: {}", e);
            }
        }
    }

    fn work(&mut self, reader: &mut BufferedReader, writer: &mut Writer<&TcpStream>) {
        if self.size <= 1 {
            println!("Killer client has been called");
            self.shutdown_server(reader, writer);
            return;
        }
        self.task_id = self.post_task(reader, writer);
        if self.task_id < 0 {
            println!("Server doesn't accept matrix. size: {}, threads: {}", self.size, self.thread_number);
            return;
        }
        thread::sleep(Duration::from_millis(2));
        let success_start: bool = self.start_task(reader, writer);
        if !success_start {
            println!("Failed to start. {}", self.to_string());
            return;
        }
        println!("Successful start. {}", self.to_string());
        thread::sleep(Duration::from_millis(1));
        let mut result: Option<ExecutionResult>;
        let mut i: i32 = 0;
        loop {
            result = self.get_status_or_result(reader, writer);
            if result.is_none(){
                println!("Result is not ready yet. {}", self.to_string());
            }
            thread::sleep(Duration::from_millis(1));
            i += 1;
            if result.is_some() || i >= 1 {//виставлено лише одну ітерацію, щоб мати можливість попросити результат який ще не готовий
                break;
            }
        }
        let result_matrix: Matrix;
        if result.is_none() {
            println!("The result is not ready, but the client asks for it {}", self.to_string());
            result_matrix = self.request_result(reader, writer).unwrap().matrix;
            /*
            In Rust, the .unwrap() method is used to retrieve the value from an Option or Result type by unwrapping it.
             It returns the inner value if it exists, or it will panic (throw runtime exception) if the value is None or if the Result is an Err variant.
            */
        } else {
            result_matrix = result.unwrap().matrix;
        }
        self.print_matrix(format!("\nResult received. {}", self.to_string()).as_str(), &result_matrix);
    }

    fn get_status_or_result(&self, reader: &mut BufferedReader, writer: &mut Writer<&TcpStream>) -> Option<ExecutionResult> {
        let status = self.get_status(reader, writer).unwrap();
        return match status {
            Status::WAITING | Status::RUNNING => None,
            Status::DONE => self.read_result(reader, writer)
        };
    }

    fn get_status(&self, reader: &mut BufferedReader, writer: &mut Writer<&TcpStream>) -> Result<Status, Box<dyn Error>> {
        writer.println_request(RequestType::GetTaskStatus);
        writer.println(format!("{}{}", ID, self.task_id).as_str()).unwrap();
        writer.println_end();
        let response = reader.get_response_type();
        match response {
            Ok(response_type) => match response_type {
                OK => {
                    let line = reader.read_line();
                    Ok(Status::from_str(&line).unwrap())
                }
                BadRequest => {
                    reader.read_error();
                    let error = CustomError { message: String::from("get_status error") };
                    Err(Box::try_from(error).unwrap())
                }
            },
            Err(e) => {
                let message = format!("task: {}. IOException in getStatus().\n{}", self.task_id, e);
                let error = CustomError::new(&message);
                Err(Box::try_from(error).unwrap())
            }
        }
    }

    fn start_task(&self, reader: &mut BufferedReader, writer: &mut Writer<&TcpStream>) -> bool {
        writer.println_request(RequestType::StartTask);
        writer.println(format!("{}{}", ID, self.task_id).as_str()).unwrap();
        writer.println_end();
        let response = reader.get_response_type().unwrap();
        return response != BadRequest;
    }

    fn post_task(&self, reader: &mut BufferedReader, writer: &mut Writer<&TcpStream>) -> i64 {
        let matrix = Matrix::new(self.size as usize);
        let id = self.id;
        let size = self.size;
        let message = format!("Client {id} created matrix of the size: {size}");
        self.print_matrix(&message, &matrix);
        return self.write_task(&matrix, reader, writer);
    }

    fn shutdown_server(&self, reader: &mut BufferedReader, writer: &mut Writer<&TcpStream>) {
        println!("Current client: {}", self.to_string());
        writer.println_request(RequestType::SHUTDOWN);
        writer.println_end();
        let mut line = String::new();
        match reader.reader.read_line(&mut line) {
            Ok(bytes_read) => {
                if bytes_read > 0 {
                    // Process the line
                    println!("SHUTDOWN response: {}", line.trim());
                } else {
                    println!("End of input");
                }
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
            }
        }
    }


    fn write_task(&self, matrix: &Matrix, reader: &mut BufferedReader, writer: &mut Writer<&TcpStream>) -> i64 {
        let threads_header = format!("{}{}", THREADS, self.thread_number);
        let size_header = format!("{}{}", SIZE, self.size);
        writer.println_request(RequestType::PostNewTask);
        writer.println(&threads_header).unwrap();
        writer.println(&size_header).unwrap();
        writer.println_end();
        let result = reader.get_response_type();
        return match result {
            Ok(response_type) => {
                return match response_type {
                    OK => {
                        if let Err(error) = writer.write_matrix(&matrix.data, self.to_string()) {
                            println!("{}", error);
                            return -1;
                        }
                        reader.parse_long(ID)
                    }
                    BadRequest => {
                        reader.read_error();
                        -1
                    }
                };
            }
            Err(error) => {
                println!("{}", error);
                -1
            }
        };
    }
    fn read_result(&self, reader: &mut BufferedReader, writer: &mut Writer<&TcpStream>) -> Option<ExecutionResult> {
        let response = reader.get_response_type().unwrap();
        return match response {
            OK => {
                let execution_time = reader.parse_long(TIME);
                println!("Downloading the result: {} executionTime: {}", self.to_string(), execution_time);
                writer.println_response(OK);
                let read: Vec<Vec<f64>> = reader.read_matrix(self.size as usize, self.to_string().as_str()).unwrap();
                /*
                In Rust, 'usize' is an unsigned integer type that represents the size of memory in bytes.
                 It is platform-dependent, meaning its size depends on the architecture of the underlying system.
                The usize type is commonly used for indexing and representing the size of collections,
                 arrays, and memory allocations. It is guaranteed to be able to hold the size of the largest possible object
                 that can be created on the current platform.
                */
                writer.println_response(OK);
                return Some(
                    ExecutionResult {
                        matrix: Matrix {
                            size: self.size as usize,
                            data: read,
                        },
                    }
                );
            }
            BadRequest => {
                reader.read_error();
                None
            }
        };
    }

    fn request_result(&self, reader: &mut BufferedReader, writer: &mut Writer<&TcpStream>) -> Option<ExecutionResult> {
        writer.println_request(RequestType::GetResult);
        writer.println(format!("{}{}", ID, self.task_id).as_str()).unwrap();
        writer.println_end();
        return self.read_result(reader, writer);
    }
}







