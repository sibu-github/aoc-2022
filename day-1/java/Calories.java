
import java.io.File;
import java.util.Scanner;



public class Calories {
    
    public static void main(String[] args) throws Exception {
        File file = new File("inputs.txt");
        Scanner sc = new Scanner(file);
        int highest_1 = 0;
        int highest_2 = 0;
        int highest_3 = 0;
        int sum = 0;
        
        while(sc.hasNextLine()){
            String line = sc.nextLine();
            if(line.isEmpty()){
                if(sum > highest_1){
                    highest_3 = highest_2;
                    highest_2 = highest_1;
                    highest_1 = sum;
                } else if(sum > highest_2){
                    highest_3 = highest_2;
                    highest_2 = sum;
                } else if(sum > highest_3){
                    highest_3 = sum;
                }
                sum = 0;
                continue;
            }
            int n = Integer.parseInt(line);
            sum += n;
        }
        sc.close();
        if(sum > highest_1){
            highest_3 = highest_2;
            highest_2 = highest_1;
            highest_1 = sum;
        } else if(sum > highest_2){
            highest_3 = highest_2;
            highest_2 = sum;
        } else if(sum > highest_3){
            highest_3 = sum;
        }

        System.out.println("Highest is = " + highest_1);
        System.out.println("Total is = " + (highest_1 + highest_2 + highest_3));
    }
    
}


