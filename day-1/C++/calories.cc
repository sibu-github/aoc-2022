#include <iostream>
#include <fstream>
#include <string>


using namespace std;

int main(){
  fstream file;
  file.open("inputs.txt", ios::in);
  if(!file.is_open()){
    cout << "cannot open file" << endl;
    return 1;
  }

  int highest_1 = 0;
  int highest_2 = 0;
  int highest_3 = 0;
  int sum = 0;
  string line;
  while(getline(file, line)){
    if(line.empty()){
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
    int n = stoi(line);
    sum += n;
  }
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
  cout << "Highest is: " << highest_1 << endl;
  cout << "Total of highest 3: " << highest_1 + highest_2 + highest_3 << endl;
  file.close();
  return 0;
}