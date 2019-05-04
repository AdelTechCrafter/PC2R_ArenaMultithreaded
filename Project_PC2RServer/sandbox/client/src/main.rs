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
		   //let req = format!("{}{}{}", "CONNECT/",nom,"/");
			
			//let req = format!("{}{}{}", "EXIT/",nom,"/");
			let coord=format!("{}:X{:.6}Y{:.6}",nom,String::from("400"),String::from("400"));
			let req = format!("{}{}{}", "NEWPOS/",coord,"/");

		
			stream.write(req.as_bytes()).unwrap();
			println!("Request from user {} : {}",nom,req);
           

            //read
			let mut buffer = [0; 512];
			stream.read(&mut buffer).unwrap();
			let received=String::from_utf8_lossy(&buffer[..]);
			println!("Received: {}",received );
			
			let mut buffer2 = [0; 512];
			stream.read(&mut buffer2).unwrap();
			let received2=String::from_utf8_lossy(&buffer2[..]);
			println!("Received2: {}",received2 );
			
			/*
			let mut buffer3 = [0; 128];
			stream.read(&mut buffer3).unwrap();
			let received3=String::from_utf8_lossy(&buffer3[..]);
			println!("Received3: {}",received2 );
			*/

    });

    vec_thread.push(handle);
    

    for t in vec_thread {
        t.join().unwrap();
    }
}


