#include <cstdio>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

int main()
{
  ifstream inputFile("input");

  vector<string> lines, games, subsets;
  string line;
  int sum = 0, gameNumber = 1, multipliedSum = 0, theMultiTemp = 0;
  bool check = true;

  if (!inputFile.is_open())
  {
    cerr << "Error opening the file." << endl;
    return 1;
  }

  unordered_map<string, int> colors = {
      {"blue", 14}, {"red", 12}, {"green", 13}};

  unordered_map<string, int> maxColors = {
      {"blue", 1}, {"red", 1}, {"green", 1}};

  while (getline(inputFile, line))
  {
    // cout << line << endl;
    lines.push_back(line);

    istringstream ss(line);
    string game, subset, smallSubset; // Strings to store the split segments

    getline(ss, game, ':'); // Split at ":" and store the first segment in token1
    getline(ss, subset);

    // cout << game << endl;
    // cout << subset << endl;

    games.push_back(game);
    subsets.push_back(subset);

    istringstream bigSubset(subset);

    while (getline(bigSubset, smallSubset, ';'))
    {
      // cout << smallSubset << endl;
      // printf("--- --- --- \n");

      istringstream bigSubset2(smallSubset);
      string smallSubset2;
      while (getline(bigSubset2, smallSubset2, ','))
      {
        // cout << smallSubset2 << endl;

        istringstream bigSubset3(smallSubset2);
        string smallSubset3, theMain;
        getline(bigSubset3, smallSubset3, ' ');

        getline(bigSubset3, theMain);

        // cout << theMain << endl;

        string color = theMain.substr(theMain.find(' ') + 1, theMain.length());
        int numOfCubes = stoi(theMain.substr(0, theMain.find(' ')));

        int temp = colors[color];
        int temp2 = maxColors[color];
        if (numOfCubes > temp)
        {
          check = false;
          // break;
        };
        if (numOfCubes > temp2)
        {
          maxColors[color] = numOfCubes;
        };
      };

      if (check == false)
      {
        // break;
      };
    };

    if (check == true)
    {
      sum += gameNumber;
    };

    gameNumber++;
    check = true;

    // printf("-------------------------------------- \n");
    theMultiTemp = maxColors["blue"] * maxColors["red"] * maxColors["green"];
    multipliedSum += theMultiTemp;

    maxColors = {
        {"blue", 1}, {"red", 1}, {"green", 1}};
  };

  // printf(" Subsets size is %d \n", subsets.size());

  // for (int i = 0; i < subsets.size(); i++)
  // {
  //   printf("Game %d and the subset is-%s \n", i + 1, subsets[i].c_str());
  // }
  inputFile.close();

  printf("this is the sum for part 1 --- -- %d -- --- \n", sum);

  printf("this is the sum for part 2 --- -- %d -- --- \n", multipliedSum);

  return 0;
}
