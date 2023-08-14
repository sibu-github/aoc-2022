#include <stdio.h>
#include <stdlib.h>

int main(void){
  FILE *ptr;
  ptr = fopen("inputs.txt", "r");

  if(ptr == NULL){
    printf("cannot open the input file");
    exit(1);
  }
  int highest_1 = 0;
  int highest_2 = 0;
  int highest_3 = 0;
  int sum = 0;
  // line size of 50 is more than enough in our case,
  // not handling the edge case where line size might be greater than the declared array size
  int lize_size = 50; 
  char line[50];
  while(fgets(line, 255, ptr) != NULL){
    if(line[0] == '\n'){
      if(sum > highest_1){
        highest_3 = highest_2;
        highest_2 = highest_1;
        highest_1 = sum;
      } else if(sum > highest_2){
        highest_3 = highest_2;
        highest_2 = sum;
      } else if (sum > highest_3){
        highest_3 = sum;  
      }
      sum = 0;
      continue;
    }
    int n = atoi(line);
    sum += n;
  }

  if(sum > highest_1){
    highest_3 = highest_2;
    highest_2 = highest_1;
    highest_1 = sum;
  } else if(sum > highest_2){
    highest_3 = highest_2;
    highest_2 = sum;
  } else if (sum > highest_3){
    highest_3 = sum;  
  }

  fclose(ptr);

  printf("Highest is: %d\n", highest_1);
  printf("Total of highest 3: %d\n", highest_1 + highest_2 + highest_3);

  return 0;
  
}