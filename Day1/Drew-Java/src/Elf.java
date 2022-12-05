import java.util.ArrayList;
import java.util.List;

public class Elf {
    private List<Integer> food;

    public Elf() {
        this.food = new ArrayList<Integer>();
    }

    public int sumFood(){
        return this.food.stream().reduce(0, Integer::sum);
    }

    public void addFood(Integer food){
        this.food.add(food);
    }

    @Override
    public String toString() {
        return "Elf{" +
                "food=" + food +
                '}';
    }
}
