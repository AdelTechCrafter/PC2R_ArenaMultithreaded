extern crate num_cpus;
extern crate rand;

use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use web_server::ThreadPool;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::sync::atomic::{
    AtomicBool,
    Ordering::*,
};
use rand::Rng;
//use std::env;

//for print
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
enum Phase {
    attente,
    jeu,
}

fn main() {
	//init
	let num = num_cpus::get();
	let pool = ThreadPool::new(num);
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
	
   
	println!("Server is running on {} cores...",num);
	//variables en acces partage
	let mut pl:Vec<String> = Vec::new();
	let mut countdown_start= Instant::now();
	let actual_phase=Phase::attente;
	let debut = AtomicBool::new(true);
	let mut s:Vec<String> = Vec::new();
	let mut coords:Vec<String> = Vec::new();
	
	//objectif initiale
	let mut rng = rand::thread_rng();
	let x=rng.gen_range(0.000,800.000);
	let y=rng.gen_range(0.000,800.000);
	let objectif=format!("X{:.6}Y{:.6}",x.to_string(),y.to_string());

	
	let players = Arc::new(Mutex::new(pl));
	let cs = Arc::new(Mutex::new(countdown_start));
	let phase = Arc::new(Mutex::new(actual_phase));
	let deb=Arc::new(debut);
	let scores = Arc::new(Mutex::new(s));
	let coords = Arc::new(Mutex::new(coords));
	let obj=Arc::new(Mutex::new(objectif));



    for stream in listener.incoming() {
        let stream = stream.unwrap();
		let p = Arc::clone(&players);
		let c=Arc::clone(&cs);
		let ph=Arc::clone(&phase);
		let d =deb.clone();
		let sc=Arc::clone(&scores);
		let cor=Arc::clone(&coords);
		let o=Arc::clone(&obj);

        pool.execute(|| {
            handle_connection(stream,p,c,ph,d,sc,cor,o);
        });

    }
}

fn handle_connection(mut stream: TcpStream,p: Arc<Mutex<Vec<String>>>,c:Arc<Mutex<Instant>>,
					ph:Arc<Mutex<Phase>>,d:Arc<AtomicBool>,sc: Arc<Mutex<Vec<String>>>,cor: Arc<Mutex<Vec<String>>>,
					o:Arc<Mutex<String>>) {
	
	loop{
	let mut buffer = [0;128];
    stream.read(&mut buffer).unwrap();
	let s=String::from_utf8_lossy(&buffer[..]);

	let split =s.split("/");
	let vec: Vec<&str> = split.collect();
	let name= vec[1];
	let r= vec[0];

	let mut players = p.lock().unwrap();
	let mut scores = sc.lock().unwrap();
	let mut coords = cor.lock().unwrap();
	let mut obj=o.lock().unwrap();

	let name=String::from(vec[1]);
	let countdown = c.lock().unwrap();
	let phase=ph.lock().unwrap();

	//pour pouvoir affecter de nouvelles valeurs a la variable partagee
	let mut count=countdown.clone();
	let mut pha=phase.clone();
	let mut objectif=obj.clone();

	let mut modifcoords=coords.clone();
	
	match r{
	"CONNECT"=>{
		if d.load(Acquire){
			count=Instant::now();
			pha=Phase::attente;
			d.store(false, Release);
		}
		if players.contains(&name){
			println!("=======================================================");
			println!("Nouvelle connexion d'un client nomme {}",name);
			println!("Refus de connection: l'utilisateur {} existe deja",name);
			println!("=======================================================");
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
			println!("=======================================================");
			println!("Nouvelle connexion d'un client nomme {}",name);
			println!("Connection etablie avec l'utilisateur {}",name);
			println!("=======================================================");
			let duration=count.elapsed().as_secs();
			
			//coordonees initiales
			let mut rng = rand::thread_rng();
			let x=rng.gen_range(0.000,800.000);
			let y=rng.gen_range(0.000,800.000);
			let coord=format!("{}:X{:.6}Y{:.6}",name.clone(),x.to_string(),y.to_string());
			coords.push(coord);
			let jcoords= coords.join("|");
			println!("\n");
			println!("Contents of coords:");
			println!("{}",jcoords);
			println!("\n");

			if duration >=20{
				println!("\n");
				println!("Lancement d'une session:");
			    println!("\n");
				pha=Phase::jeu;

				let session = format!("{}{}/{}","SESSION/",jcoords,objectif);
				stream.write(session.as_bytes()).unwrap();
				stream.flush().unwrap();
				/*
				for x in players.iter() {
					println!("> {}", x);
				}
				*/
				
				
			}

			//players
			players.push(name.clone());
			//scores
			let score_user=format!("{}{}{}",name.clone(),":",String::from("0"));
			scores.push(score_user);
			let jscores= scores.join("|");
			println!("\n");
			println!("Contents of scores:");
			println!("{}",jscores);
			println!("\n");

			
			
			//envoi au client qui a emit la requete
			let req_jeu = format!("{}{:?}/{}/{}/","WELCOME/",pha,jscores,objectif);
			let req_attente = format!("{}{:?}/{}/","WELCOME/",pha,jscores);
			
			if pha==Phase::jeu {
				stream.write(req_jeu.as_bytes()).unwrap();
				stream.flush().unwrap();
			}
			else{
				stream.write(req_attente.as_bytes()).unwrap();
				stream.flush().unwrap();
			}
			
			let new_player = format!("{}{}","NEWPLAYER/",name.clone());
			stream.write(new_player.as_bytes()).unwrap();
			stream.flush().unwrap();
			
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

		let player_left = format!("{}{}/","PLAYERLEFT/",name.clone());
		stream.write(player_left.as_bytes()).unwrap();
		stream.flush().unwrap();
		break;
	},
	"NEWPOS"=>{
		//vec[1]= "riri:X0.000125Y-2.5"
		let newcoord= vec[1];
		let splitnew =newcoord.split(":");
		let vec_coord: Vec<&str> = splitnew.collect();
		for i in 0..coords.len() {
			let sp=coords[i].split(":");
			let vsp:Vec<&str>=sp.collect();
			if vec_coord[0]==vsp[0] {
				coords[i]=String::from(newcoord);
			}
		}

		let jcoords2= coords.join("|");
		let tick = format!("{}{}/","TICK/",jcoords2);
		stream.write(tick.as_bytes()).unwrap();
		stream.flush().unwrap();
		
		println!("=======================================================");
		println!("Modification des coordonnes:");
		println!("{}",jcoords2);
		println!("=======================================================");

	},
	 _ => println!("unknown request {} ",r),
	}
	std::mem::drop(players);
	std::mem::drop(scores);
	std::mem::drop(coords);

	std::mem::drop(countdown);
	std::mem::drop(phase);
    stream.write(&buffer).unwrap();
	}
}

