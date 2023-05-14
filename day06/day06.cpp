#include <iostream>
#include <fstream>
#include <string>
#include <set>

using namespace std;

int main() {
    string line;
    int sum = 0;
    // part1
    [&, sum](int unique) mutable {
        fstream fs("input6.txt");
        while (getline(fs, line)) {
            for (int i = 0; i < line.length(); i++) {
                set<char> start;
                string str = line.substr(i, unique);
                for (auto &x : str) {
                    start.insert(x);
                    if (start.size() == unique) {
                        sum = i + unique;
                        goto end;
                    }
                }
            }
        end: 
            break;
        }
        cout << sum << '\n';
    }(4);
    
    // part2
    [&, sum](int unique) mutable {
        fstream fs("input6.txt");
        while (getline(fs, line)) {
            for (int i = 0; i < line.length(); i++) {
                set<char> start;
                string str = line.substr(i, unique);
                for (auto &x : str) {
                    start.insert(x);
                    if (start.size() == unique) {
                        sum = i + unique;
                        goto end;
                    }
                }
            }
        end:
            break;
        }
        cout << sum << '\n';
    }(14);

    return 0;
}