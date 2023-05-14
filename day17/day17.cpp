#include <iostream>
#include <fstream>
#include <vector>
#include <array>
#include <ranges>
#include <set>
#include <unordered_map>
#include <xutility>

using namespace std;

struct Hash;

using CharBuffer = istreambuf_iterator<char>;
using ChamberSet = set<pair<int, int>>;
using Pattern = vector<pair<int, int>>;
using Cycle = tuple<int, int, Pattern>;
using Cache = unordered_map<Cycle, pair<size_t, size_t>, Hash>;

enum class MV_OP {
    SUB = 0,
    ADD
};

struct Hash {
    size_t operator()(const Cycle &c) const {
        const auto &[jetIdx, patIdx, fc] = c;
        size_t seed = fc.size();
        for (const auto &[px, py] : fc) {
            // not great
            seed ^= hash<int>()(px) ^ hash<int>()(py << 1);
        }
        return seed;
    }
};

Pattern newPattern(uint32_t idx, int y) {
    array<Pattern, 5> patterns = {
        {
            {{2, y}, {3, y}, {4, y}, {5, y}}, // wide
            {{3, y}, {2, y + 1}, {3, y + 1}, {4, y + 1}, {3, y + 2}}, // plus
            {{2, y}, {3, y}, {4, y}, {4, y + 1}, {4, y + 2}}, // L
            {{2, y}, {2, y + 1}, {2, y + 2}, {2, y + 3}}, // long
            {{2, y}, {3, y}, {2, y + 1}, {3, y + 1}} // box
        },
    };
    return patterns[idx];
}

Pattern moveVertical(Pattern &curr, MV_OP op) {
    return curr | views::transform([&](auto &x) {
               return make_pair(x.first, op == MV_OP::SUB ? x.second -= 1 : x.second += 1);
           }) |
           ranges::to<vector>();
}

Pattern moveHorizontal(Pattern &curr, MV_OP op) {
    for (const auto &x : curr) {
        if ((x.first == 0 && op == MV_OP::SUB) || (x.first == 6 && op == MV_OP::ADD)) {
            return curr;
        }
    }

    return curr | views::transform([&](auto &x) {
               return make_pair(op == MV_OP::SUB ? x.first -= 1 : x.first += 1, x.second);
           }) |
           ranges::to<vector>();
}

bool isColliding(Pattern &curr, const ChamberSet &chamber) {
    return ranges::find_if(curr, [&](const auto &x) {
               return chamber.contains(x);
           }) != curr.end();
}

bool hitBase(Pattern &curr) {
    return ranges::find_if(curr, [](const auto &x) {
               return x.second == 0;
           }) != curr.end();
}

Pattern collect(ChamberSet &chamber) {
    auto maxTopBound = ranges::max_element(chamber, [](const auto &u, const auto &v) {
                           return u.second < v.second;
                       })->second;

    return chamber | views::filter([&](const auto &x) {
               return maxTopBound - x.second <= 20;
           }) |
           views::transform([&](const auto &x) {
               return make_pair(x.first, maxTopBound - x.second);
           }) |
           ranges::to<vector>();
}

size_t simulate(string_view stream, const size_t MAX_ROCKS) {
    size_t add = 0;
    size_t topBound = 0;
    size_t idx = 0;
    ChamberSet chamber;
    Cache cache;

    stream.remove_suffix(min(stream.size() - stream.find_last_not_of("\r\n") - 1, stream.size()));

    for (size_t i = 0; i < MAX_ROCKS; i++) {
        auto rock = newPattern(i % 5, topBound + 4);

        while (true) {
            if (stream[idx] == '>') {
                rock = moveHorizontal(rock, MV_OP::ADD);
                if (isColliding(rock, chamber)) {
                    rock = moveHorizontal(rock, MV_OP::SUB);
                }
            } else {
                rock = moveHorizontal(rock, MV_OP::SUB);
                if (isColliding(rock, chamber)) {
                    rock = moveHorizontal(rock, MV_OP::ADD);
                }
            }

            idx = (idx + 1) % stream.size();

            rock = moveVertical(rock, MV_OP::SUB);
            if (isColliding(rock, chamber) || hitBase(rock)) {
                rock = moveVertical(rock, MV_OP::ADD);
                ranges::copy(rock, inserter(chamber, chamber.end()));
                topBound = ranges::max_element(chamber, [](const auto &u, const auto &v) {
                               return u.second < v.second;
                           })->second;

                if (i >= 2022) {
                    auto fc = make_tuple(i % 5, idx, collect(chamber));
                    if (cache.find(fc) != cache.end()) {
                        const auto &[lsIt, lsTop] = cache[fc];
                        size_t cl = i - lsIt;
                        size_t td = topBound - lsTop;
                        size_t skip = (MAX_ROCKS - i) / cl;
                        add += skip * td;
                        i += skip * cl;
                    }
                    cache[fc] = {i, topBound};
                }
                break;
            }
        }
    }

    return topBound + add;
}

int main() {
    fstream fs("input17.txt");
    string stream((CharBuffer(fs)), CharBuffer());

    // part 1
    cout << simulate(stream, 2022) << '\n';
    
    // part 2
    cout << simulate(stream, 1000000000000) << '\n';

    return 0;
}