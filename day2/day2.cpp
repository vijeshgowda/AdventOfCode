#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>


int main() {
  std::ifstream inputFile("input");

  std::vector<std::string> lines, games, subsets;
  std::string line;

  if (!inputFile.is_open()) {
    std::cerr << "Error opening the file." << std::endl;
    return 1;
  }

  while (std::getline(inputFile, line)) {
    // std::cout << line << std::endl;
    lines.push_back(line);

    std::istringstream ss(line);
    std::string game, subset; // Strings to store the split segments

    std::getline(ss, game,
                 ':'); // Split at ":" and store the first segment in token1
    std::getline(ss, subset);

    std::cout << game << std::endl;
    std::cout << subset << std::endl;

    games.push_back(game);
    subsets.push_back(subset);
  }

  inputFile.close();

  int resultmain = 0;

  std::cout << "---------- the sum is ---------" << std::endl;
  std::cout << resultmain << std::endl;
  printf("this is the sum --- -- %d -- --- \n", resultmain);

  return 0;
}
