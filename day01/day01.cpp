#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <algorithm>

using namespace std;

int main() {
    int sum = 0;
    string line;
    vector<int> calories;
    fstream fs("input1.txt");
    while (getline(fs, line)) {
        if (line.length() == 0) {
            calories.push_back(sum);
            sum = 0;
            continue;
        }
        sum += stoi(line);
    }
    
    sort(calories.begin(), calories.end(), greater<int>());

    // part 1
    cout << calories[0] << '\n';

    // part 2
    cout << calories[0] + calories[1] + calories[2] << '\n';

    return 0;
}