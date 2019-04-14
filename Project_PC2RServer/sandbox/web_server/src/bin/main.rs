use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use web_server::ThreadPool;
//use std::sync::{Arc, Mutex};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7870").unwrap();
    let pool = ThreadPool::new(4);
	let mut players:Vec<&str> = Vec::new();

	//let players: Arc<Mutex<Vec<&str>>> = Arc::new(Mutex::new(players));


    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream,players);
        });
    }
}


fn handle_connection(mut stream: TcpStream,mut players:Vec<&str>) {
    let mut buffer = [0;128];
    stream.read(&mut buffer).unwrap();
	let s=String::from_utf8_lossy(&buffer[..]);

	let mut split =s.split("/");
	let vec: Vec<&str> = split.collect();

	//let mut players = players.lock().unwrap();
	if !players.contains(&vec[1]) {
		players.push(&vec[1]);
		println!("Nouvelle connexion d'un client nomme {}",vec[1]);
	}
	else{
		println!("Connection denied , user {} already exists",vec[1]);
	}
	//std::mem::drop(players);
    stream.write(&buffer).unwrap();
    stream.flush().unwrap();
}



