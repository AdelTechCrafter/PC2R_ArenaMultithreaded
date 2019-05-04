package gameObject;

import java.io.BufferedReader;
import java.io.BufferedWriter;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.io.OutputStream;
import java.io.OutputStreamWriter;
import java.net.Socket;
import java.net.UnknownHostException;
import java.nio.Buffer;
import java.util.ArrayList;
import java.util.Random;
import java.time.Duration;
import java.time.Instant;
import javafx.animation.AnimationTimer;
import javafx.application.Application;
import javafx.geometry.Pos;
import javafx.scene.Node;
import javafx.scene.Scene;
import javafx.scene.control.Alert;
import javafx.scene.control.Alert.AlertType;
import javafx.scene.control.Label;
import javafx.scene.input.KeyCode;
import javafx.scene.layout.Pane;
import javafx.scene.layout.StackPane;
import javafx.scene.paint.Color;
import javafx.scene.shape.Circle;
import javafx.scene.shape.Polygon;
import javafx.scene.shape.Rectangle;
import javafx.scene.text.Font;
import javafx.scene.text.FontWeight;
import javafx.scene.text.Text;
import javafx.scene.text.TextAlignment;
import javafx.stage.Stage;

public class Arena extends Application { 
	
	public final static int SIZE = 800;
	public final int WIN_CAP_POINTS = 10;
	private Vehicule player;
	private ArrayList<Vehicule> players = new ArrayList<>();
	private ArrayList<Obstacle> obstacles = new ArrayList<>();
	private GameObject objectif;
	private Text text ;
	Pane root;
	private static String adress = "127.0.0.1";
	private static int port = 7878;
	private static Socket socket;
	private static BufferedWriter writer;
	private static OutputStream out;
	private static InputStream in;
	private static BufferedReader reader;
	private final static int TICK = 750;
	Instant start_timer;
	double elapsed_time = 0;
	String coords;
	private Boolean isFinish = false;
	
	
	private Pane createPane() {
		try {
			socket = new Socket(adress, port);
			out = socket.getOutputStream();
			in = socket.getInputStream();
			writer = new BufferedWriter(new OutputStreamWriter(out));
			reader = new BufferedReader(new InputStreamReader(in));
		} catch (IOException e1) {
			// TODO Auto-generated catch block
			e1.printStackTrace();
		}
		start_timer = Instant.now();
		root = new Pane();
		root.setPrefSize(SIZE,  SIZE);
		objectif = new GameObject(new Circle(15,15,15, Color.YELLOW));
		createObstacle();
		text = new Text();
        text.setTextAlignment(TextAlignment.CENTER);
		root.getChildren().add(text);
		Random r = new Random();
		double x = 1 + (600)*r.nextDouble();
		double y = 1 + (600)*r.nextDouble();
		moveObj(objectif, x ,y);


		root.getChildren().add(objectif.getView());
		
		player = new Vehicule(createTriangle(Color.BLUE), "toto");
		players.add(player);
		player.getView().setTranslateX(400);
		player.getView().setTranslateY(400);
		root.getChildren().add(player.getView());
		connectToServ();

		AnimationTimer timer = new AnimationTimer() {
			
			@Override
			public void handle(long now) {
				onUpdate();			
			}

		};
		timer.start();
		
		return root;
	}
	
	

	@Override
	public void start(Stage stage) throws Exception {
		stage.setScene(new Scene(createPane()));
		stage.setTitle("Game"); 
		
		stage.getScene().setOnKeyPressed(e -> {
			if( e.getCode() == KeyCode.UP) {
				player.thrust();
			}
			
			else if( e.getCode() == KeyCode.LEFT) {
				player.clock();
			}
			
			else if( e.getCode() == KeyCode.RIGHT) {
				player.antiClock();
			}
			else if (e.getCode() == KeyCode.ESCAPE) {
                try {
                    sendExit();
                } catch (IOException e1) {
                    // TODO Auto-generated catch block
                    e1.printStackTrace();
                }
            }
			else if(e.getCode() == KeyCode.DOWN) {
                player.anti_thrust();
            }
			
			
		});
		stage.show();
	}
	
	private void sendExit() throws IOException {
        String str = "EXIT/"+player.getId()+"\n";
        writer.write(str);
        writer.flush();
        System.exit(0);
    }
	private void onUpdate() {
		Instant end_timer = Instant.now();
		
		elapsed_time = Duration.between(start_timer, end_timer).toMillis();
		
		for(Vehicule p : players) {

			if(p.getScore() >= WIN_CAP_POINTS & !isFinish) {
				isFinish = true;
				//implementer Pop up victoire
				victoryPopUp(p);
			}
			
			if(p.collide(objectif) & !isFinish){
				Random r = new Random();
				double x = 1 + (600)*r.nextDouble();
				double y = 1 + (600)*r.nextDouble();
				moveObj(objectif, x ,y);
				p.setScore(p.getScore()+1);
			}
			
			for(Obstacle o : obstacles) {
				if(p.collide(o)) {
					p.speed.x = -p.speed.x;
					p.speed.y = -p.speed.y;
				}
			}
			if(elapsed_time>TICK) {
				try {
					//sendVCoord();
					sendCoord();
					handle_Server_Message();
				} catch (IOException e) {
					// TODO Auto-generated catch block
					e.printStackTrace();
				}
				start_timer = Instant.now();
			}
			
			p.update();
			
		}
		
}
	
	


	private void victoryPopUp(Vehicule p) {
		text = new Text("\n"+p.getId()+" is the Winner of the game ! \n Congratulations !");
		text.setFill(Color.DARKRED);
		text.setFont(Font.font(null, FontWeight.BOLD, 36));
        text.setTextAlignment(TextAlignment.CENTER);
		root.getChildren().add(text);
		root.getChildren().remove(objectif.getView());
		
	}

	private void moveObj(GameObject obj, double x, double y) {
		obj.getView().setTranslateX(x);
		obj.getView().setTranslateY(y);
	}
		
	public static void main(String[] args) throws UnknownHostException, IOException {
		
		launch(args);
	}
	
	private void sendCoord() throws IOException {
		
		String str = "NEWPOS/"+player.getId()+":X"+player.getCoord().x+"Y"+player.getCoord().y+"/\n";
		writer.write(str);
		writer.flush();
	}
	
	private void newPlayer(String user) {
		int r = (int) (Math.random()*(255 -0));
		int g = (int) (Math.random()*(255 -0));
		int b = (int) (Math.random()*(255 -0));
		Color color = Color.rgb(r, g, b);
		Vehicule v = new Vehicule(createTriangle(color), user);
		players.add(v);
		v.getView().setTranslateX(400);
		v.getView().setTranslateY(400);
		root.getChildren().add(v.getView());
		System.out.println("A new challenger have entered the arena !");
	}
	
	private void removePlayer(String user) {
		for(Vehicule p: players) {
			if(p.getId() == user) {
				root.getChildren().remove(p.getView());
				players.remove(p);
			}
		}
	}
	
	private void connectToServ() {
		try {
			String str = "CONNECT/"+player.getId()+"/\n";
			writer.write(str);
		} catch (IOException e) {
			e.printStackTrace();
		}
		
	}
	
	private void updatePlayersCoords(String coords) {
		String[] coord = coords.split("\\|");
		for(String str : coord) {
			boolean exist = false;
			String[] info = str.split(":");
			for(Vehicule v : players) {
				if(v.getId().equals(info[0])) {
					exist = true;
					String[] pos = info[1].split("Y");
					double x = Double.parseDouble(pos[0].substring(1, pos[0].length()));
					double y = Double.parseDouble(pos[1]);
					v.getCoord().x = x;
					v.getCoord().y =y;
				}
			}
			if(!exist) {
				newPlayer(info[0]);
				String[] pos = info[1].split("Y");
				double x = Double.parseDouble(pos[0].substring(1, pos[0].length()));
				double y = Double.parseDouble(pos[1]);
				players.get(players.size()-1).getCoord().x = x;
				players.get(players.size()-1).getCoord().y =y;
			}
		}
	}

	private void updatePlayersVCoords(String coords) {
		double x,y,vx,vy, rad;
		String[] coord = coords.split("\\|");
		for(String str : coord) {
			boolean exist = false;
			String[] info = str.split(":");
			for(Vehicule v : players) {
				if(v.getId().equals(info[0])) {
					exist = true;
					String[] pos = info[1].split("Y");
					x = Double.parseDouble(pos[0].substring(1, pos[0].length()));
					String[] posY = pos[1].split("VX");
					y = Double.parseDouble(posY[0]);
					vx = Double.parseDouble(posY[1].substring(0, posY[0].length()-2));
					pos = pos[2].split("T");
					vy = Double.parseDouble(pos[0]);
					rad = Double.parseDouble(pos[1]);
					v.setDirection(rad);
				}
			}
			if(!exist) {
				newPlayer(info[0]);
				String[] pos = info[1].split("Y");
				x = Double.parseDouble(pos[0].substring(1, pos[0].length()));
				y = Double.parseDouble(pos[1]);
				players.get(players.size()-1).getCoord().x = x;
				players.get(players.size()-1).getCoord().y =y;
			}
		}
	}
	
	private void updatePlayersScores(String scores) {
		String[] score = scores.split("|");
		for(String str : score) {
			String[] info = str.split(":");
			for(Vehicule v : players) {
				if(v.getId().equals(info[0])) {
					int point = Integer.parseInt(info[1]);
					v.setScore(point);
				}
			}
		}
	}

	private void updateObj(String coord) {
		String[] pos = coord.split("Y");
		double x = Double.parseDouble(pos[0].substring(1, pos[0].length()));
		double y = Double.parseDouble(pos[1]);
		moveObj(objectif, x, y);
	}

	private void reveiveObstacle(String ocoords) {
		String[] coord = coords.split("\\|");
		for(String str : coord) {
			boolean exist = false;
			String[] info = str.split(":");
			
		}
	}
	
	private void handle_Server_Message() throws IOException {
		String server_string;
		while((server_string = reader.readLine())!=null) {
			String[] str = server_string.split("/");
			switch (str[0]) {
				case "NEWPLAYER" :
					newPlayer(str[1]);
					break;
					
				case "DENIED":
					//reconnect();
					break;
					
				case "WELCOME" :
					updatePlayersScores(str[2]);
					if(str[1].equals("jeu")) {
						updateObj(str[3]);
						reveiveObstacle(str[4]);
					}
					break;
					
				case "PLAYERLEFT":
					removePlayer(str[1]);
					break;
					
				
				case "SESSION" :
					updatePlayersCoords(str[1]);
					updateObj(str[2]);
					reveiveObstacle(str[3]);
					break;					
				
				case "WINNER":
					updatePlayersScores(str[1]);
					//endOfSession();
					break;
					
				case "TICK" :
					updatePlayersVCoords(str[1]);
					break;
					
				case "NEWOBJ":
					updateObj(str[1]);
					updatePlayersScores(str[2]);
					break;
					
				default :
					continue;
			}
		}
		
	}
	
	
	private void sendComms() throws IOException {
		double radian = Math.toRadians(player.last_rotation);
		//toModify
		String str = "NEWCOM/A"+radian+"T"+player.nb_thrust+"/\n";
		player.nb_thrust = 0;
		player.last_rotation = 0;
		writer.write(str);
		writer.flush();
	}
	
	private void sendVCoord() throws IOException {
		double radian = Math.toRadians(player.direction);
		String str = "NEWPOS/"+player.getId()+":X"+player.getCoord().x+"Y"+player.getCoord().y+"VX"+player.getSpeed().x+"VY"+player.getSpeed().y+"T"+radian+"\n";
		writer.write(str);
		writer.flush();
	}
	
	
	public void createObstacle() {
		Random r = new Random();
		double x,y;
		boolean closeToAnother = false;
		for(int i = 0; i<8; i++) {
			Obstacle obs = new Obstacle(new Circle(20, 20, 20, Color.BLACK));
			do {
				closeToAnother = false;
				x = 1 + (600)*r.nextDouble();
				y = 1 + (600)*r.nextDouble();
				for(Obstacle o : obstacles) {
					if((x>o.getCoord().x-150 && x<o.getCoord().x+150) && (y>o.getCoord().y-150 && y<o.getCoord().y+150))
						closeToAnother = true;
				}
			}while((x>SIZE/2-200 && x<SIZE/2+200) && (y>SIZE/2-200 && y<SIZE/2+200) || closeToAnother);
			obs.setCoord(new Point(x, y));
			moveObj(obs, x, y);
			obs.getView().setTranslateX(x);
			obs.getView().setTranslateY(y);
			root.getChildren().add(obs.getView());
			obstacles.add(obs);
		}
	}
	
	public Node createTriangle(Color color) {
        Polygon polygon = new Polygon();
        polygon.getPoints().addAll(new Double[]{
            0.0, 0.0,
            40.0, 15.0,
            0.0, 30.0 });
        polygon.setFill(color);
        return polygon;
	}
}