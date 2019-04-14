use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
	//let nbconnection = Arc::new(Mutex::new(0));
	//let countdown:u8 = 20;
	//let server_tickrate:u8=10;

    for stream in listener.incoming() {
		//let nbconnection = Arc::clone(&nbconnection);
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}


fn handle_connection(mut stream: TcpStream) {
	//let mut nbplayer = nbconnection.lock().unwrap();
	//*nbplayer++;
	//if(*nbplayer==1){
	//}

	let mut buffer = String::new();
    stream.read_to_string(&mut buffer)?;
    //stream.read(&mut buffer).unwrap();
	//let mut received= String::from_utf8_lossy(&buffer[..]).to_ascii_lowercase();
	println!("Request: {}",buffer );
	//Nouvelle connexion d’un client nomme ’user’	}
    

}