#include <fstream>
#include <iostream>
#include <string>
#include <regex>

int main() {

  std::ifstream infile("input.txt");
  std::string working_string{};

  while (std::getline(infile, working_string)) {

    std::regex game_pattern {R"(Game \d+\:)"};
    std::smatch match;
    if (std::regex_search(working_string.begin(), working_string.end(), match, game_pattern))
    {
      std::cout << "Match: " << match[1] << '\n';
    }


    std::cout << working_string << "\n";
  }

  return 0;
}
