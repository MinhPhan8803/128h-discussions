use std::io::Write;
use std::net::TcpStream;
use std::thread;
use std::io::{BufReader, BufRead};
use inquire::Text;

fn writer(mut server_stream: TcpStream) {
    while let Ok(data) = Text::new("enter input").prompt() {
        server_stream.write_all(format!("{}\n", data).as_bytes()).unwrap();
    }
}

fn listener(server_stream: TcpStream) {
    let mut reader = BufReader::new(server_stream);
    let mut new_data = String::new();

    while let Ok(n) = reader.read_line(&mut new_data) {
        if n == 0 {
            break;
        }
        println!("{}", new_data);
        new_data.clear();
    }
}

fn main() {
    let server_stream = TcpStream::connect("localhost:8500").unwrap();
    let server_stream_cloned = server_stream.try_clone().unwrap();
    let handle_1 = thread::spawn(|| listener(server_stream_cloned));
    let handle_2 = thread::spawn(|| writer(server_stream));

    let _ = handle_1.join();
    let _ = handle_2.join();
}
