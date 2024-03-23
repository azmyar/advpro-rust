use std::{
    fs::File,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {   
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] { 
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"), 
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(10));
                        ("HTTP/1.1 200 OK", "hello.html")
                    }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"), // Else
    };

    let response = handle_request(status_line, filename);

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_request(status_line: &str, filename: &str) -> String {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    response
}