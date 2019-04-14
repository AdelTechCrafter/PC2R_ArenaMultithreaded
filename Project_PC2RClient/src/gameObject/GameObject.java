package gameObject;
import javafx.scene.Node;
import javafx.scene.paint.Color;
import javafx.scene.shape.Circle;

public class GameObject {
	protected Point coord;
	protected Node view;
	protected boolean isVisible;
	
	public GameObject(Node n) {
		this.view = n;
		isVisible = true;
	}

	
	public boolean isVisible() {
		return isVisible;
	}


	public void setVisible(boolean isVisible) {
		this.isVisible = isVisible;
	}


	public Point getCoord() {
		return coord;
	}

	public void setCoord(Point coord) {
		this.coord = coord;
	}


	public Node getView() {
		return view;
	}

	public void setView(Node view) {
		this.view = view;
	}
	
	public boolean collide(GameObject v) {
		return getView().getBoundsInParent().intersects(v.getView().getBoundsInParent());
	}
}