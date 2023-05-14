#include <iostream>
#include <string>
#include <fstream>
#include <vector>
#include <unordered_map>
#include <algorithm>
#include <ranges>

using namespace std;

int main() {
    int sum = 0;
    vector<string> lines;
    string line;
    fstream fs("input7.txt");
    while (getline(fs, line)) {
        lines.push_back(line);
    }

    vector<string> cwd;
    unordered_map<string, int> vals;
    for (const auto &line : lines) {
        if (line.find("$ cd") != line.npos) {
            if (line.find("..") != line.npos) {
                cwd.pop_back();
            } else {
                cwd.push_back(line.substr(5));
            }
        } else if (line.find("$ ls") != line.npos) {
            continue;
        } else {
            if (!cwd.empty()) {
                if (any_of(line.begin(), line.end(), ::isdigit)) {
                    string path;
                    for (size_t i = 0; i < cwd.size(); i++) {
                        path += '/' + cwd[i];
                        vals[path] += stoll(line);
                    }
                }
            }
        }
    }

    // part 1
    [&vals, sum]() mutable {
        for (const auto &x : vals | views::filter([](auto &&v) {
                                 return v.second <= 100'000;
                             })) {
            sum += x.second;
        }
        cout << sum << '\n';
    }();

    // part 2
    [&vals]() mutable {
        int needSize = 30'000'000;
        int unusedSize = 70'000'000 - vals["//"];
        auto min = ranges::min(vals | views::filter([&](auto &&v) {
                                   return v.second > (needSize - unusedSize);
                               }) |
                               views::values);
        cout << min << '\n';
    }();

    return 0;
}