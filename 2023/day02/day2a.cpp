#include <exception>
#include <fstream>
#include <ios>
#include <iostream>
#include <regex>
#include <string>
#include <vector>

class Color {
private:
  std::string colorName{};

public:
  Color(std::string _colorName) {
    if (_colorName == "red" || _colorName == "green" || _colorName == "blue") {
      colorName = _colorName;
    } else {
      colorName = "invalid";
      // throw std::invalid_argument("Color can only be red, green or blue");
    }
  }

  std::string getName() const { return colorName; }
};

bool operator<(const Color &l, const Color &r) {
  return l.getName() < r.getName();
}

class Handfull {
private:
  std::map<Color, int> colors{};

public:
  Handfull(std::string handfull, std::map<Color, int> bag = {}) {

    std::cout << "Handfull: ";

    std::smatch m;
    std::string::const_iterator searchStart(handfull.cbegin());
    while (std::regex_search(searchStart, handfull.cend(), m,
                             std::regex(R"((\d+) (\w+))"))) {
      searchStart = m.suffix().first;

      auto res = colors.emplace(Color(m[2].str()), std::stoi(m[1].str()));

      std::cout << m[1].str() << " " << m[2].str() << " - ";
    }
    if (!bag.empty()) {
      std::cout << "valid: " << std::boolalpha << isValid(bag);
    }
    std::cout << "\n";
  };

  auto getColors() { return colors; }
  bool isValid(std::map<Color, int> bag) {
    auto valid = true;
    for (auto &color : colors) {
      auto colorName = color.first;
      auto colorCount = color.second;
      auto bagColorCount = bag[colorName];
      if (colorCount > bagColorCount) {
        valid = false;
      }
    }
    return valid;
  }
};

class Game {
private:
  int gameID{};
  std::vector<Handfull> handfulls{};
  std::map<Color, int> totals{};
  bool validGame{true};

public:
  Game(std::string input, std::map<Color, int> bag) {

    // Construct gameID
    std::smatch m;
    std::regex_search(input, m, std::regex(R"(Game (\d+):)"));
    gameID = std::stoi(m[1].str());
    std::cout << "Game " << gameID << "\n";
    std::cout << input << "\n";

    std::string::const_iterator searchStart(input.cbegin());
    while (std::regex_search(searchStart, input.cend(), m,
                             std::regex(R"((\d+ \w+, ){1,}(\d+ \w+[;]?))"))) {
      searchStart = m.suffix().first;

      handfulls.push_back(Handfull(m[0].str(), bag));
    }

    for (auto &handfull : handfulls) {
      for (auto &color : handfull.getColors()) {
        auto colorName = color.first;
        auto colorCount = color.second;

        if (totals.find(colorName) != totals.end()) {
          totals[colorName] += colorCount;
        } else {
          totals.emplace(colorName, colorCount);
        }

        auto bagColorCount = bag[colorName];
        if (colorCount > bagColorCount) {
          validGame = false;
        }
      }
    }

    std::cout << "Game valid? " << std::boolalpha << validGame << "\n";
    std::cout << "\n";
    // std::cout << "Total colors in handfull: ";
    // for (auto &t : totals) {
    //   std::cout << t.first.getName() << " " << t.second << ", ";
    // }
    // std::cout << "\n";
  }

  bool isValid() { return validGame; }
  auto getID() { return gameID; }
};

int main() {
  std::ifstream infile("input.txt");
  std::string working_string{};

  std::vector<Game> games{};
  std::map<Color, int> bag = {
      {Color("red"), 12}, {Color("green"), 13}, {Color("blue"), 14}};

  while (std::getline(infile, working_string)) {
    games.push_back(Game(working_string, bag));
  }

  int sum{0};
  for (auto &game : games) {
    if (game.isValid()) {
      sum += game.getID();
    }
  }

  std::cout << "final result: " << sum << "\n";

  // std::map<Color, int> validationMap{
  //     {Color("red"), 12}, {Color("green"), 13}, {Color("blue"), 14}};

  // int sum{0};
  // for (auto &game : games) {
  //   auto res = game.validateGame(validationMap);
  //   std::cout << res << "\n";
  //   sum += res;
  // }

  // std::cout << "final result: " << sum << "\n";

  return 0;
}
