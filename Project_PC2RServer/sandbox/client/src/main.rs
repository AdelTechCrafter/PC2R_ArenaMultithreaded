use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::env;
fn main() {
    let mut vec_thread = Vec::new();
	

	let handle = thread::spawn(move || {
			let args: Vec<String> = env::args().collect();
			let nom=&args[1];

            let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
		   let req = format!("{}{}{}", "CONNECT/",nom,"/");
			
			//let req = format!("{}{}{}", "EXIT/",nom,"/");

		
			stream.write(req.as_bytes()).unwrap();
			println!("Request from user {} : {}",nom,req);
           

            //read
            let mut buffer = [0; 128];
            stream.read(&mut buffer).unwrap();


			let received=String::from_utf8_lossy(&buffer[..]);
			println!("Received: {}",received );

    });

    vec_thread.push(handle);
    

    for t in vec_thread {
        t.join().unwrap();
    }
}


