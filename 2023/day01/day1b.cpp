#include <fstream>
#include <iostream>
#include <vector>
#include <string>
#include <numeric>
#include <map>


auto replaceNumbers(std::string& input) -> void {

  std::map<std::string, std::string> numbersMap {
    {"one", "1e"},
    {"two", "2o"},
    {"three", "3e"},
    {"four", "4r"},
    {"five", "5e"},
    {"six", "6x"},
    {"seven", "7n"},
    {"eight", "8t"},
    {"nine", "9e"}
  };

  std::string::size_type n;
  for(size_t i = 0; i <= input.size(); i++) {
    for(const auto& [text, num] : numbersMap) {
      n = input.rfind(text,i);
      if(n != std::string::npos)
      {
        //std::cout  << " " << i << " " << input << " " << text;
        input.replace(n,text.size(),num); 
      }
    }
  }
//  std::cout <<"\n";
}

int main(){
  std::ifstream infile("input.txt");

    
  std::vector<int> numbers{};
  std::string working_string {};

  while (infile >> working_string)
  {
    std::cout << "input: " << working_string << " ";
    replaceNumbers(working_string);
    std::cout << "numbers replaced: "<< working_string << " ";
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

  std::cout << "The sum is: " << result << "\n";
  
  return 0;
}
