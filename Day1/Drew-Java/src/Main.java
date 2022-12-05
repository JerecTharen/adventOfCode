import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;
import java.util.Scanner;
import java.util.stream.Collectors;

public class Main {

    /**
     * Takes input file path and reads in the elf objects from the file
     *
     * @param path
     * @return
     */
    public static List<Elf> readInput(String path) {

        Scanner scanner = null;

        //null catch
        try {
            scanner = new Scanner(new File(path));
        } catch (FileNotFoundException e) {
            throw new RuntimeException(e);
        }

        List<Elf> result = new ArrayList<Elf>();

        //adding the first elf to the list
        result.add(new Elf());

        while (scanner.hasNextInt()){

            String input = scanner.nextLine();

            if(input == ""){
                result.add(new Elf());
            }
            else{
                result.get(result.size()-1).addFood(Integer.parseInt(input));
            }
        }

        return result;
    }

    /**
     * takes path to input file then returns the result for part 1
     *
     * @param path
     * @return
     */
    public static int partOne(String path){

        List<Elf> input = readInput(path);

        int result = input.stream()
                .map(x -> x.sumFood())
                .collect(Collectors.toList())
                .stream()
                .max(Integer::compare)
                .get();

        return result;
    }

    /**
     * takes path to input file then returns the result for part 2
     *
     * @param path
     * @return
     */
    public static int partTwo(String path){

        List<Elf> input = readInput(path);

        List<Integer> foodSums = input.stream()
                .map(x -> x.sumFood())
                .collect(Collectors.toList());

        List<Integer> mostSnacks = foodSums.stream().sorted(Comparator.reverseOrder())
                .limit(3)
                .collect(Collectors.toList());

        int result = mostSnacks.stream()
                .reduce(0, Integer::sum);

        return result;
    }

    public static void main(String []args){

        String filePath = "PATH_TO_FILE";

        int answerOne = partOne(filePath);

        int answerTwo = partTwo(filePath);

        System.out.println(
                "answer for part one: " + answerOne + "\n" + "answer for part two: " + answerTwo
        );
    }
}
