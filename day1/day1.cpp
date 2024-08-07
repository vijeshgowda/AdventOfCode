#include <fstream>
#include <iostream>
#include <string>
#include <unordered_set>
#include <vector>
#include <unordered_map>

using namespace std;

int getResult(vector<string> &lines);

int main()
{
  ifstream inputFile("input");

  vector<string> lines;
  string line;

  if (!inputFile.is_open())
  {
    cerr << "Error opening the file." << endl;
    return 1;
  }

  while (getline(inputFile, line))
  {
    // cout << line << endl;
    lines.push_back(line);
  }

  inputFile.close();

  int resultmain = getResult(lines);

  cout << "---------- the sum is ---------" << endl;
  cout << resultmain << endl;
  printf("this is the sum --- -- %d -- --- \n", resultmain);

  return 0;
}

int getResult(vector<string> &lines)
{
  int i = 0, sum = 0;
  int j = lines.size();

  unordered_set<char> digits = {'0', '1', '2', '3', '4',
                                '5', '6', '7', '8', '9'};

  for (int k = 0; k < lines.size(); k++)
  {
    string tempL = lines[k];

    cout << tempL << endl;

    char iv = 'a', jv = 'a';
    for (i = 0, j = lines[k].size() - 1; iv == 'a' || jv == 'a'; i++, j--)
    {
      char tempi = tempL[i];
      // cout << tempi << endl;
      char tempj = tempL[j];
      // cout << tempj << endl;

      if (digits.find(tempi) != digits.end() && iv == 'a')
      {
        iv = tempi;
        // cout << iv << endl;
      }
      else if (iv == 'a')
      {
        if (i + 2 < tempL.length() && tempL.substr(i, 3) == "one")
        {
          iv = '1';
        }
        else if (i + 2 < tempL.length() && tempL.substr(i, 3) == "two")
        {
          iv = '2';
        }
        else if (i + 4 < tempL.length() && tempL.substr(i, 5) == "three")
        {
          iv = '3';
        }
        else if (i + 3 < tempL.length() && tempL.substr(i, 4) == "four")
        {
          iv = '4';
        }
        else if (i + 3 < tempL.length() && tempL.substr(i, 4) == "five")
        {
          iv = '5';
        }
        else if (i + 2 < tempL.length() && tempL.substr(i, 3) == "six")
        {
          iv = '6';
        }
        else if (i + 4 < tempL.length() && tempL.substr(i, 5) == "seven")
        {
          iv = '7';
        }
        else if (i + 4 < tempL.length() && tempL.substr(i, 5) == "eight")
        {
          iv = '8';
        }
        else if (i + 3 < tempL.length() && tempL.substr(i, 4) == "nine")
        {
          iv = '9';
        }
        // cout << iv << endl;
      }

      if (digits.find(tempj) != digits.end() && jv == 'a')
      {
        jv = tempj;
        // cout << jv << endl;
      }
      else if (jv == 'a')
      {
        if (j >= 2 && tempL.substr(j - 2, 3) == "one")
        {
          jv = '1';
        }
        else if (j >= 2 && tempL.substr(j - 2, 3) == "two")
        {
          jv = '2';
        }
        else if (j >= 4 && tempL.substr(j - 4, 5) == "three")
        {
          jv = '3';
        }
        else if (j >= 3 && tempL.substr(j - 3, 4) == "four")
        {
          jv = '4';
        }
        else if (j >= 3 && tempL.substr(j - 3, 4) == "five")
        {
          jv = '5';
        }
        else if (j >= 2 && tempL.substr(j - 2, 3) == "six")
        {
          jv = '6';
        }
        else if (j >= 4 && tempL.substr(j - 4, 5) == "seven")
        {
          jv = '7';
        }
        else if (j >= 4 && tempL.substr(j - 4, 5) == "eight")
        {
          jv = '8';
        }
        else if (j >= 3 && tempL.substr(j - 3, 4) == "nine")
        {
          jv = '9';
        }
        // cout << jv << endl;
      }
    }

    string ijv = string() + iv + jv;
    // cout << ijv << endl;
    sum += stoi(ijv);
  }

  return sum;
}
