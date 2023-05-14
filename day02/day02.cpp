#include <iostream>
#include <fstream>
#include <string>
#include <unordered_map>

using namespace std;

int main() {
    auto mod = [](int num, int mod) {
        return ((num % mod) + mod) % mod;
    };

    unordered_map<char, int> list {{{'X', 1}, {'Y', 2}, {'Z', 3}, {'A', 1}, {'B', 2}, {'C', 3}}};
    int sum = 0;
    string line;
    // part 1
    [&, sum]() mutable {
        fstream fs("input2.txt");
        while (getline(fs, line)) {
            int op = list[line[0]];
            int me = list[line[2]];
            sum += me;
            if (mod(me - op, 3) == 1) {
                sum += 6;
            }
            if (op == me) {
                sum += 3;
            }
        }
        cout << sum << '\n';
    }();
    
    // part 2
    [&, sum]() mutable {
        fstream fs("input2.txt");
        while (getline(fs, line)) {
            int op = list[line[0]];
            int oc = list[line[2]];
            sum += ((mod(op + oc, 3) + 1) + (oc - 1) * 3);
        }
        cout << sum << '\n';
    }();

    return 0;
}