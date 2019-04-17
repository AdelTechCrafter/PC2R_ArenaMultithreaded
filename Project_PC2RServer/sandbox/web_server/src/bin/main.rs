use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use web_server::ThreadPool;
use std::sync::{Arc, Mutex};
use std::time::Instant;
//use std::env;

//for print
#[derive(Debug)]
#[derive(Clone)]
enum Phase {
    attente,
    jeu,
}

fn main() {
	//init
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
	let actual_phase=Phase::attente;
	println!("Server is running...");

	//variable en acces partage
	let mut pl:Vec<String> = Vec::new();
	let mut Countdown_start= Instant::now();
	//let mut s:Vec<String> = Vec::new();
	//let mut coords:Vec<String> = Vec::new();
	

	
	let players = Arc::new(Mutex::new(pl));
	let cs = Arc::new(Mutex::new(Countdown_start));
	let phase = Arc::new(Mutex::new(actual_phase));
	//let scores = Arc::new(Mutex::new(s));
	//et coords = Arc::new(Mutex::new(coords));

	



	//let players: Arc<Mutex<Vec<str>>> = Arc::new(Mutex::new(players));
	//let players: Arc<Mutex<Vec<&str>>> = Arc::new(Mutex::new(players));


    for stream in listener.incoming() {
        let stream = stream.unwrap();
		let p = Arc::clone(&players);
		let c=Arc::clone(&cs);
		let ph=Arc::clone(&phase);
        pool.execute(|| {
            handle_connection(stream,p,c,ph);
        });
    }
}


fn handle_connection(mut stream: TcpStream,p: Arc<Mutex<Vec<String>>>,c:Arc<Mutex<Instant>>,ph:Arc<Mutex<Phase>>) {
    let mut buffer = [0;128];
    stream.read(&mut buffer).unwrap();
	let s=String::from_utf8_lossy(&buffer[..]);

	let split =s.split("/");
	let vec: Vec<&str> = split.collect();
	let name= vec[1];
	let r= vec[0];

	let mut players = p.lock().unwrap();
	let name=String::from(vec[1]);
	let countdown = c.lock().unwrap();
	let phase=ph.lock().unwrap();

	let mut count=countdown.clone();
	let mut pha=phase.clone();

	match r{
	"CONNECT"=>{
		if players.len()==0{
			count=Instant::now();
			pha=Phase::attente;
		}
		if players.contains(&name){
			println!("Nouvelle connexion d'un client nomme {}",name);
			println!("Refus de connection: l'utilisateur {} existe deja",name);
			let req = format!("{}","DENIED/");
			stream.write(req.as_bytes()).unwrap();
			stream.flush().unwrap();
			println!("{}", req);
			println!("Contents of players:");
			for x in players.iter() {
				println!("> {}", x);
			}
		}
		else{
			println!("Nouvelle connexion d'un client nomme {}",name);
			println!("Connection etablie avec l'utilisateur {}",name);
			let duration=count.elapsed().as_secs();
			if duration >=20{
				pha=Phase::jeu;
			}
			let req = format!("{}{:?}","WELCOME/",pha);
			stream.write(req.as_bytes()).unwrap();
			stream.flush().unwrap();

			players.push(name);
			println!("Contents of players:");
			for x in players.iter() {
				println!("> {}", x);
			}
		}
	},
	"EXIT"=>{
		println!("le client nomme {} se deconnecte",name);
		let index = players.iter().position(|x| *x == name).unwrap();
		players.remove(index);
		println!("Contents of players:");
		for x in players.iter() {
			println!("> {}", x);
		}
	},
	 _ => println!("unknown request {} ",r),
	}
	std::mem::drop(players);
	std::mem::drop(countdown);
	std::mem::drop(phase);
    stream.write(&buffer).unwrap();
    
}


/*
extern crate timer;
extern crate chrono;
use std::sync::mpsc::channel;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use web_server::ThreadPool;
use std::sync::{Arc, Mutex};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7870").unwrap();
    let pool = ThreadPool::new(4);
	println!("Server is running...");
	//let mut players:Vec<&str> = Vec::new();
	let mut nbplayers =0;
	//let players: Arc<Mutex<Vec<&str>>> = Arc::new(Mutex::new(players));


    for stream in listener.incoming() {
        let stream = stream.unwrap();
		nbplayers=nbplayers+1;
		//countdown
		if nbplayers ==1 {
			println!("The first player is trying to connect, countdown start(10s)...");
			let timer = timer::Timer::new();
			let (tx, rx) = channel();
			let _guard = timer.schedule_with_delay(chrono::Duration::seconds(10), move || {let _ignored = tx.send(());});
			rx.recv().unwrap();
			println!("10 seconds after first connection");
		}

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0;128];
    stream.read(&mut buffer).unwrap();
	let s=String::from_utf8_lossy(&buffer[..]);

	let split =s.split("/");
	let vec: Vec<&str> = split.collect();
	println!("Nouvelle connexion d'un client nomme {}",vec[1]);

	//let mut players = players.lock().unwrap();
	/*
	if !players.contains(&vec[1]) {
		players.push(&vec[1]);
		println!("Nouvelle connexion d'un client nomme {}",vec[1]);
	}
	else{
		println!("Connection denied , user {} already exists",vec[1]);
	}
	*/
	//std::mem::drop(players);
    stream.write(&buffer).unwrap();
    stream.flush().unwrap();
}
*/