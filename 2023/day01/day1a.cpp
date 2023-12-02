#include <fstream>
#include <iostream>
#include <vector>
#include <string>
#include <numeric>


int main(){
  std::ifstream infile("input.txt");
//  std::ifstream infile("input_small.txt");
  
  std::vector<int> numbers{};
  std::string working_string {};

  while (infile >> working_string)
  {
    std::cout << working_string << " ";
    int a = -1;
    int b = -1;

    for(char& c : working_string) {
      if (c >= '0' && c <= '9') {
        if( a == -1) {
          a = c - '0';
          b = a; // Just incase we only have one number
        } else {
          b = c - '0';
        }
      }
    }
    std::cout << a << b << "\n";
    numbers.push_back(a*10+b);
  }

  int result = std::accumulate(numbers.begin(), numbers.end(), 0);

  std::cout << "The result is: " << result << "\n";
  
  return 0;
}
