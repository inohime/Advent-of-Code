#include <iostream>
#include <fstream>
#include <string>
#include <array>
#include <deque>
#include <regex>

using namespace std;

int main() {
    string line;
    regex cratePattern("\\[(.*?)\\] ");
    regex movePattern(R"(move ([0-9]+) from ([0-9]+) to ([0-9]+))");
    smatch m;
    int count = 0;
    int src = 0;
    int dst = 0;
    
    // part1
    [&]() mutable {
        array<deque<char>, 9> crates;
        fstream fs("input5.txt");
        while (getline(fs, line)) {
            if (regex_search(line, cratePattern)) {
                for (int i = 0; i < 9; i++) {
                    auto c = line[(i * 4) + 1];
                    if (c == ' ' || c == '[' || c == ']') {
                        continue;
                    }
                    crates[i].push_back(c);
                }
            }

            if (regex_match(line, m, movePattern)) {
                count = stoi(m[1]);
                src = stoi(m[2]) - 1;
                dst = stoi(m[3]) - 1;

                for (int i = 0; i < count; i++) {
                    crates[dst].push_front(crates[src].front());
                    crates[src].pop_front();
                }
            }
        }

        for (auto &x : crates) {
            cout << x.front();
        }
    }();

    cout << '\n';

    // part2
    [&]() mutable {
        array<deque<char>, 10> crates;
        fstream fs("input5.txt");
        while (getline(fs, line)) {
            if (regex_search(line, cratePattern)) {
                for (int i = 0; i < 9; i++) {
                    auto c = line[(i * 4) + 1];
                    if (c == ' ' || c == '[' || c == ']') {
                        continue;
                    }
                    crates[i].push_back(c);
                }
            }

            if (regex_match(line, m, movePattern)) {
                count = stoi(m[1]);
                src = stoi(m[2]) - 1;
                dst = stoi(m[3]) - 1;

                for (int i = 0; i < count; i++) {
                    crates[9].push_front(crates[src].front());
                    crates[src].pop_front();
                }

                for (int i = 0; i < count; i++) {
                    crates[dst].push_front(crates[9].front());
                    crates[9].pop_front();
                }
            }
        }

        for (int i = 0; i < crates.size(); i++) {
            if (crates[i].size() == 0) {
                break;
            }
            cout << crates[i].front();
        }
    }();

    return 0;
}