extern crate rand;
extern crate time;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::env;
use rand::prelude::*;



use time::PreciseTime;


fn transform_u64_to_array_of_u8(x: u64) -> [u8; 8] {
    let b1: u8 = ((x >> 56) & 0xff) as u8;
    let b2: u8 = ((x >> 48) & 0xff) as u8;
    let b3: u8 = ((x >> 40) & 0xff) as u8;
    let b4: u8 = ((x >> 32) & 0xff) as u8;
    let b5: u8 = ((x >> 24) & 0xff) as u8;
    let b6: u8 = ((x >> 16) & 0xff) as u8;
    let b7: u8 = ((x >> 8) & 0xff) as u8;
    let b8: u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4, b5, b6, b7, b8];
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let nbrequest: usize = args[1].parse().unwrap();
	let start = PreciseTime::now();
    let mut vec_thread = Vec::new();
    
    for _i in 0..nbrequest {
        let handle = thread::spawn(move || {
            let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

            //write
            let rand_u64: u64 = rand::thread_rng().gen();
			let array:[u8;8]=transform_u64_to_array_of_u8(rand_u64);

            //let request: String = rand_u64.to_string();
            //stream.write(request.as_bytes()).unwrap();
			stream.write(&array).unwrap();

            //read
            let mut buffer:[u8;8] = [0;8];
            stream.read(&mut buffer).unwrap();
			let plain= u64::from_be_bytes(buffer);
            //println!("Received : {}", String::from_utf8_lossy(&buffer));
			println!("Received : {}",plain);
        });
        vec_thread.push(handle);
    }

    for t in vec_thread {
        t.join().unwrap();
    }
	let end = PreciseTime::now();
	println!("{} seconds for executing several client threads", start.to(end));