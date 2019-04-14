package gameObject;
import java.io.File;
import java.io.IOException;
import java.nio.charset.MalformedInputException;

import javafx.scene.Node;
import javafx.scene.image.Image;
import javafx.scene.image.ImageView;

public class Vehicule extends GameObject{
	
	protected double direction;
	protected Point speed;
	
	private int turnit = 8;
	private double thrustit = 0.4;
	private int score = 0;
	private String id;
	
	
	public Vehicule(Node view, String id) {
		super(view);
		this.id = id;
		this.coord = new Point(Arena.SIZE/2, Arena.SIZE/2);
		this.direction = 0;
		this.speed = new Point(0, 0);
	}

	public void clock() {
		direction = direction-turnit;
		view.setRotate(direction);
	}
	
	public void antiClock() {
		direction = direction+turnit;
		view.setRotate(direction);
	}
	
	public void thrust() {
		speed.x = speed.getX() + thrustit*Math.cos(Math.toRadians(direction));
		speed.y = speed.getY() + thrustit*Math.sin(Math.toRadians(direction));
	}
		
	public void update() {
		if(coord.x <0) {
			coord.x = Arena.SIZE-coord.x;
		}
		if(coord.y <0){
			coord.y = Arena.SIZE-coord.y;
		}
		coord.x = (coord.x + speed.getX())%Arena.SIZE;
		coord.y = (coord.y + speed.getY())%Arena.SIZE; 
		
		view.setTranslateX(coord.x);
		view.setTranslateY(coord.y);
	}


	public double getDirection() {
		return direction;
	}

	public void setDirection(double direction) {
		this.direction = direction;
	}

	public Point getSpeed() {
		return speed;
	}

	public void setSpeed(Point speed) {
		this.speed = speed;
	}

	public int getScore() {
		return score;
	}

	public void setScore(int score) {
		this.score = score;
	}

	public String getId() {
		return id;
	}

	public void setId(String id) {
		this.id = id;
	}

	
	
}