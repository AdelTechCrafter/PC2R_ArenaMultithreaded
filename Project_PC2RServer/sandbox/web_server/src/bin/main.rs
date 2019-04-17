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
	println!("Server is running...");
	//variable en acces partage
	let mut pl:Vec<String> = Vec::new();
	let mut Countdown_start= Instant::now();
	let actual_phase=Phase::attente;
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

