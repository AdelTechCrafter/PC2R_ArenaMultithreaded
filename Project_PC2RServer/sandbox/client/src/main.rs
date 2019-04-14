use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
//use rand::Rng;

fn main() {
    let mut vec_thread = Vec::new();

    //Send 100 requests at the same time
    for _i in 0..10 {
        let handle = thread::spawn(move || {
            let mut stream = TcpStream::connect("127.0.0.1:7870").unwrap();

            //let mut rng = rand::thread_rng();
            //let r: [u8; 8] = rng.gen();
			
            //write
            //println!("Sending  {}", u64::from_be_bytes(r));
			//stream.write(&r).unwrap();
			stream.write("CONNECT/toto".as_bytes());
			println!("CONNECT/toto");
           

            //read
            let mut buffer = [0; 128];
            stream.read(&mut buffer).unwrap();


			let received=String::from_utf8_lossy(&buffer[..]);
			println!("Request: {}",received );

            // Check that we are getting back the same data
            //assert!(buffer == r);
        });
        vec_thread.push(handle);
    }

    for t in vec_thread {
        t.join().unwrap();
    }
}