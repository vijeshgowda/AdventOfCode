#include <fstream>
#include <functional>
#include <iostream>
#include <stdio.h>
#include <string>
#include <unordered_set>
#include <vector>


int getResult(std::vector<std::string> &lines);

int main() {
  std::ifstream inputFile("input");

  std::vector<std::string> lines;
  std::string line;

  if (!inputFile.is_open()) {
    std::cerr << "Error opening the file." << std::endl;
    return 1;
  }

  while (std::getline(inputFile, line)) {
    // std::cout << line << std::endl;
    lines.push_back(line);
  }

  inputFile.close();

  // std::cout << "------- Lines read from the file: -------" << std::endl;

  // for (auto i = lines.begin(); i != lines.end(); ++i)
  // {
  //   std::cout << *i << std::endl;
  // }

  // std::cout << "------- Lines read from the file: ------- 222" << std::endl;

  // for (int j = 0; j < lines.size(); j++)
  // {
  //   std::cout << lines[j] << std::endl;
  // }

  std::cout << "---------- the sum is ---------" << std::endl;
  std::cout << getResult(lines) << std::endl;

  return 0;
}

int getResult(std::vector<std::string> &lines) {
  int i = 0, sum = 0;
  int j = lines.size();

  std::unordered_set<char> digits = {'0', '1', '2', '3', '4',
                                     '5', '6', '7', '8', '9'};

  for (int k = 0; k < lines.size(); k++) {
    std::string tempL = lines[k];
    char iv = 'a', jv = 'a';
    for (i = 0, j = lines[k].size() - 1; iv == 'a' || jv == 'a'; i++, j--) {

      // std::cout << tempL << std::endl;

      char tempi = tempL[i];
      // std::cout << tempi << std::endl;
      char tempj = tempL[j];
      // std::cout << tempj << std::endl;

      if (digits.find(tempi) != digits.end() && iv == 'a') {
        iv = tempi;
        // std::cout << iv << std::endl;
      }

      if (digits.find(tempj) != digits.end() && jv == 'a') {
        jv = tempj;
        // std::cout << jv << std::endl;
      }
    }

    std::string ijv = std::string() + iv + jv;
    // std::cout << ijv << std::endl;
    sum += std::stoi(ijv);
  }

  return sum;
}
