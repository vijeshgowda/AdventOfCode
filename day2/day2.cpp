#include <cstdio>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <unordered_map>
#include <vector>

int main()
{
  std::ifstream inputFile("input");

  std::vector<std::string> lines, games, subsets;
  std::string line;
  int sum = 0, gameNumber = 1;
  bool check = true;

  if (!inputFile.is_open())
  {
    std::cerr << "Error opening the file." << std::endl;
    return 1;
  }

  std::unordered_map<std::string, int> colors = {
      {"blue", 14}, {"red", 12}, {"green", 13}};

  while (std::getline(inputFile, line))
  {
    // std::cout << line << std::endl;
    lines.push_back(line);

    std::istringstream ss(line);
    std::string game, subset, smallSubset; // Strings to store the split segments

    std::getline(ss, game, ':'); // Split at ":" and store the first segment in token1
    std::getline(ss, subset);

    // std::cout << game << std::endl;
    // std::cout << subset << std::endl;

    games.push_back(game);
    subsets.push_back(subset);

    std::istringstream bigSubset(subset);

    while (std::getline(bigSubset, smallSubset, ';'))
    {
      // std::cout << smallSubset << std::endl;
      printf("--- --- --- \n");

      std::istringstream bigSubset2(smallSubset);
      std::string smallSubset2;
      while (std::getline(bigSubset2, smallSubset2, ','))
      {
        // std::cout << smallSubset2 << std::endl;

        std::istringstream bigSubset3(smallSubset2);
        std::string smallSubset3, theMain;
        std::getline(bigSubset3, smallSubset3, ' ');

        std::getline(bigSubset3, theMain);

        std::cout << theMain << std::endl;

        std::string color = theMain.substr(theMain.find(' ') + 1, theMain.length());
        int numOfCubes = std::stoi(theMain.substr(0, theMain.find(' ')));

        int temp = colors[color];
        if (numOfCubes > temp)
        {
          check = false;
          break;
        };
      };

      if (check == false)
      {
        break;
      };
    };

    if (check == true)
    {
      sum += gameNumber;
    };

    gameNumber++;
    check = true;
  };

  // printf(" Subsets size is %d \n", subsets.size());

  // for (int i = 0; i < subsets.size(); i++)
  // {
  //   printf("Game %d and the subset is-%s \n", i + 1, subsets[i].c_str());
  // }
  inputFile.close();

  std::cout << "---------- the sum is ---------" << std::endl;
  std::cout << sum << std::endl;
  printf("this is the sum --- -- %d -- --- \n", sum);

  return 0;
}
