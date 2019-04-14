use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use web_server::ThreadPool;



fn main() {
    let listener = TcpListener::bind("127.0.0.1:7870").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0;128];
    stream.read(&mut buffer).unwrap();
	println!("Request: {}", String::from_utf8_lossy(&buffer[..]).to_ascii_lowercase());

	//let lu = u64::from_be_bytes(buffer);
	//println!("Request: {}",lu);
    
	//println!("Request: {}", String::from_utf8_lossy(&buffer[..]).to_ascii_lowercase());
    //let plain = u64::from_be_bytes(buffer);
    //let key = rand::thread_rng().gen();
    stream.write(&buffer).unwrap();
    stream.flush().unwrap();
}



