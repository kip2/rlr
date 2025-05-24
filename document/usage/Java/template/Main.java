import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
      Scanner sc = new Scanner(System.in);
      // case: input to int[]
      // int[] inputs = getLineAsIntArray(sc);
      // System.out.println("input value: " + Arrays.toString(inputs));

      // case: input to string
      // String input = getLineAsString(sc);
      // System.out.println("input value: " + input);
      sc.close();
    }

    // sample function to get inputs
    public static String getLineAsString(Scanner sc) {
        String line = sc.nextLine();
        return line;
    }

    // sample function to get inputs
    public static int[] getLineAsIntArray(Scanner sc) {
      String line = sc.nextLine();

      String[] tokens = line.trim().split("\\s+");
      int[] nums = new int[tokens.length];
      for (int i = 0; i < tokens.length; i++) {
        nums[i] = Integer.parseInt(tokens[i]);
      }
      return nums;
    }

    // todo: implemented me!
    public static String solve() {
      return null;
    }
}


