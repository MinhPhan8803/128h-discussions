use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::mpsc;
use std::io::{BufReader, BufRead};

enum ReceiverType {
    Data(String),
    Stream(TcpStream),
}

fn message_distributer(rx: mpsc::Receiver<ReceiverType>) {
    let mut current_streams: Vec<TcpStream> = Vec::new();

    for message in rx {
        match message {
            ReceiverType::Data(data) => {
                for stream in current_streams.iter_mut() {
                    stream.write_all(data.as_bytes()).unwrap();
                }
            }
            ReceiverType::Stream(new_stream) => {
                current_streams.push(new_stream);
            }
        }
    }
}

fn handle_client(tx: mpsc::Sender<ReceiverType>, client_stream: TcpStream) {
    let mut reader = BufReader::new(client_stream);
    let mut new_data = String::new();

    while let Ok(n) = reader.read_line(&mut new_data) {
        if n == 0 {
            break;
        }
        println!("message: {}", new_data);
        tx.send(ReceiverType::Data(new_data.clone())).unwrap();
        new_data.clear();
    }
}

fn main() {
    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new(); 
    let (tx, rx) = mpsc::channel::<ReceiverType>();

    handles.push(thread::spawn(|| message_distributer(rx)));

    let listener = TcpListener::bind("localhost:8500").unwrap();

    for stream in listener.incoming() {
        println!("received new client");
        let new_client_stream = stream.unwrap();
        tx.send(ReceiverType::Stream(new_client_stream.try_clone().unwrap())).unwrap();
        let tx_cloned = tx.clone();
        handles.push(thread::spawn(|| handle_client(tx_cloned, new_client_stream)));
    }

    for _handle in handles {

    }
}
