using System;
using System.Collections.Generic;
using System.IO;


public class Calories {

    public static void Main(){
        string fileName = "inputs.txt";
        int highest1 = 0;
        int highest2 = 0;
        int highest3 = 0;
        int sum = 0;
        IEnumerable<string> lines = File.ReadLines(fileName);
        foreach(var line in lines){
            if(line == ""){
                if(sum > highest1){
                    highest3 = highest2;
                    highest2 = highest1;
                    highest1 = sum;
                } else if(sum > highest2){
                    highest3 = highest2;
                    highest2 = sum;
                } else if(sum > highest3){
                    highest3 = sum;
                }
                sum = 0;
                continue;
            }
            int num = int.Parse(line);
            sum += num;
        }

        if(sum > highest1){
            highest3 = highest2;
            highest2 = highest1;
            highest1 = sum;
        } else if(sum > highest2){
            highest3 = highest2;
            highest2 = sum;
        } else if(sum > highest3){
            highest3 = sum;
        }

        Console.WriteLine("Highest is: {0}", highest1);
        Console.WriteLine("Total of highest 3 is: {0}", highest1 + highest2 + highest3);

    }
}