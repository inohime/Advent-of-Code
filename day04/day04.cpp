#include <iostream>
#include <fstream>
#include <string>

using namespace std;

int main() {
    auto getRange = [](string &s, bool end) -> pair<int, int> {
        if (!end) {
            string h1 = s.substr(0, s.find(','));
            return {stoi(h1.substr(0, h1.find('-'))), stoi(h1.substr(h1.find('-') + 1))};
        }
        string h2 = s.substr(s.find(',') + 1);
        return {stoi(h2.substr(0, h2.find('-'))), stoi(h2.substr(h2.find('-') + 1))};
    };

    auto inRange = [](pair<int, int> &p1, pair<int, int> &p2) -> bool {
        auto &[a, b] = p1;
        auto &[c, d] = p2;

        // range A - B
        // range  c,d

        // range C - D
        // range  a,b

        if (a <= c && d <= b) {
            return true;
        }

        if (c <= a && b <= d) {
            return true;
        }

        return false;
    };

    auto isOverlap = [](pair<int, int> &p1, pair<int, int> &p2) -> bool {
        auto &[a, b] = p1;
        auto &[c, d] = p2;

        // range A - B
        // range  c,d

        // range C - D
        // range  a,b

        if (!(d < a) && !(b < c)) {
            return true;
        }

        return false;
    };

    int sum = 0;
    string line;
    // part 1
    [&, sum]() mutable {
        fstream fs("input4.txt");
        while (getline(fs, line)) {
            auto a = getRange(line, false);
            auto b = getRange(line, true);
            if (inRange(a, b)) {
                sum += 1;
            }
        }
        cout << sum << '\n';
    }();

    // part 2
    [&, sum]() mutable {
        fstream fs("input4.txt");
        while (getline(fs, line)) {
            auto a = getRange(line, false);
            auto b = getRange(line, true);
            if (isOverlap(a, b)) {
                sum += 1;
            }
        }
        cout << sum << '\n';
    }();

    return 0;
}