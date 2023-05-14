#include <iostream>
#include <fstream>
#include <string>
#include <unordered_map>

using namespace std;

int main() {
    auto check = [](string &s1, string &s2) -> pair<bool, char> {
        for (auto &t1 : s1) {
            for (auto &t2 : s2) {
                if (t1 == t2) {
                    return {true, t1};
                }
            }
        }
        return {false, NULL};
    };
    // inefficient
    auto check2 = [](string &s1, string &s2, string &s3) -> pair<bool, char> {
        for (auto &t1 : s1) {
            for (auto &t2 : s2) {
                for (auto &t3 : s3) {
                    if (t1 == t2) {
                        if (t2 == t3 && t3 == t1) {
                            return {true, t1};
                        }
                    }
                }
            }
        }
        return {false, NULL};
    };

    unordered_map<char, int> list;
    // lowercase 1-26
    // uppercase 27-52
    int x = 0;
    for (char ch = 'a'; ch <= 'z'; ch++) {
        ++x;
        list.insert({ch, x});
    }
    for (char ch = 'A'; ch <= 'Z'; ch++) {
        ++x;
        list.insert({ch, x});
    }

    int sum = 0;
    string line;
    // part 1
    [&, sum]() mutable {
        fstream fs("input3.txt");
        while (getline(fs, line)) {
            auto h1 = line.substr(0, line.length() / 2);
            auto h2 = line.substr(line.length() / 2);

            auto [a, b] = check(h1, h2);
            if (a) {
                sum += list[b];
            }
        }
        cout << sum << '\n';
    }();

    // part 2
    [&, sum]() mutable {
        fstream fs("input3.txt");
        int u = 0;
        vector<string> m;
        while (getline(fs, line)) {
            m.push_back(line);
            ++u;
            if (u >= 3) {
                u = 0;
                auto [a, b] = check2(m[0], m[1], m[2]);
                if (a) {
                    sum += list[b];
                }
                m.clear();
            }
        }
        cout << sum << '\n';
    }();

    return 0;
}