#include <cstdio>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

// string trim2(string &s)
// {
//     int start = 0, end = s.length() - 1;
//     if (s[0] == ' ')
//     {
//         start = 1;
//     }
//     if (s[end] == ' ')
//     {
//         end--;
//     }
//     string res = "";
//     for (int i = start; i <= end; i++)
//     {
//         res += s[i];
//     }

//     return res;
// }

int totalCards(vector<vector<int>> cardMatch, vector<int> card)
{
    int sum = 0;
    if (card[1] == 0)
    {
        return 1;
    }

    for (int i = card[0]; i < card[0] + card[1]; i++)
    {
        sum += totalCards(cardMatch, cardMatch[i]);
    }
    return sum + 1;
}

int main()
{
    ifstream inputFile("input");

    int sum = 0, cardNo = 1;
    string line;
    vector<string> lines;
    vector<vector<int>> cardMatch;

    if (!inputFile.is_open())
    {
        cerr << "file not opening" << endl;
        return 1;
    }

    while (getline(inputFile, line))
    {
        string winNums, ourNums, allNums, numsTemp;
        vector<int> winNums2, ourNums2;
        int count = -1;

        istringstream ss(line);
        getline(ss, allNums, ':');
        getline(ss, allNums);

        istringstream temp(allNums);
        getline(temp, winNums, '|');
        getline(temp, ourNums);

        // cout << winNums << endl;
        // cout << ourNums << endl;

        // winNums = trim2(winNums);
        istringstream ss2(winNums);
        while (getline(ss2, numsTemp, ' '))
        {
            if (numsTemp == "")
            {
                continue;
            }
            int number = std::stoi(numsTemp);
            winNums2.push_back(number);
        }

        // ourNums = trim2(ourNums);
        istringstream ss3(ourNums);
        while (getline(ss3, numsTemp, ' '))
        {
            if (numsTemp == "")
            {
                continue;
            }

            int number = std::stoi(numsTemp);
            ourNums2.push_back(number);
        }

        for (int i : winNums2)
        {
            if (find(ourNums2.begin(), ourNums2.end(), i) != ourNums2.end())
            {
                count++;
            }
        }
        int tempSum = pow(2, count);
        sum += tempSum;

        cardMatch.push_back({cardNo, count + 1});
        cardNo++;
    }

    printf("sum for part 1 --- -- %d -- --- \n", sum);

    int part2 = 0;
    for (int i = 0; i < cardMatch.size(); i++)
    {
        part2 += totalCards(cardMatch, cardMatch[i]);
    }

    printf("total cards for part 2 --- -- %d -- --- \n", part2);
}