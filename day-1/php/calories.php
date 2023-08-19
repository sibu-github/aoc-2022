<?php
    $highest_1 = 0;
    $highest_2 = 0;
    $highest_3 = 0;
    $sum = 0;
    if ($file = fopen("inputs.txt", "r"))  {
        while (($line = fgets($file)) !== false) {
            if ($line == "\n"){
                if($sum > $highest_1){
                    $highest_3 = $highest_2;
                    $highest_2 = $highest_1;
                    $highest_1 = $sum;
                } else if($sum > $highest_2){
                    $highest_3 = $highest_2;
                    $highest_2 = $sum;
                } else if($sum > $highest_3){
                    $highest_3 = $sum;
                }
                $sum = 0;
                continue;
            }
            $num = (int)$line;
            $sum = $sum + $num;
        }
        fclose($file);
        if($sum > $highest_1){
            $highest_3 = $highest_2;
            $highest_2 = $highest_1;
            $highest_1 = $sum;
        } else if($sum > $highest_2){
            $highest_3 = $highest_2;
            $highest_2 = $sum;
        } else if($sum > $highest_3){
            $highest_3 = $sum;
        }

        echo "Higest is: ", $highest_1, "\n";
        echo "Total of highest 3 is: ", $highest_1 + $highest_2 + $highest_3 , "\n";
    } else {
        echo "input file not found", "\n";
    }
?>