#include <cassert>
#include <algorithm>
#include <memory>
#include <numeric>
#include "aoc.h"

namespace {
using Scalar = long int;
struct Point;

using Circuit = std::vector<Point *>;

struct Point {
    Scalar x, y, z;
    Circuit *circuit;
    Point(const std::string &txt)
        : circuit {new Circuit{}}
    {
        std::string line {txt};
        x = aoc::parsetoken<Scalar>(line, ",");
        y = aoc::parsetoken<Scalar>(line, ",");
        z = aoc::parsetoken<Scalar>(line, ",");
    }
};

struct Distance {
    std::pair<Point *, Point *> points;
    Scalar distance;
    Distance(Point *pa, Point *pb, Scalar distance)
    : points{std::min(pa,pb), std::max(pa,pb)}
    , distance(distance)
    {
    }
    auto operator<=>(const Distance &rhs) const {
        return distance <=> rhs.distance;
    }
};

std::ostream &operator << (std::ostream &os, const Point &p) {
    return os << p.x << "," << p.y << "," << p.z;
}

template <typename T> T square(T v) { return v*v; }

struct Puzzle {
    std::vector<Point> points;
    std::vector<Distance> distances;

    void parse(std::istream &is) {
        std::string l;
        while (std::getline(is, l))
            points.emplace_back(l);
        for (size_t i = 0; i < points.size(); ++i)
            points[i].circuit->push_back(&points[i]);

        for (size_t i = 0; i < points.size() - 1; ++i) {
            for (size_t j = i+1; j < points.size(); ++j) {
                auto &ip = points[i];
                auto &jp = points[j];
                auto sqd = square(ip.x - jp.x) + square(ip.y - jp.y) + square(ip.z - jp.z);
                distances.emplace_back(&ip,&jp,sqd);
            }
        }
    }

    Scalar best(const Distance &d, int iter) {
        std::array<Circuit *, 4> best{};
        for (auto &p : points) {
            best[3] = p.circuit;
            for (size_t i = 0; i < 3; ++i) {
                if (best[3] == best[i]) {
                    best[3] = nullptr;
                    break;
                }
            }
            if (best[3] == nullptr)
                continue;
            for (size_t i = 2; i < 4; --i) {
                if (best[i] == nullptr || best[i]->size() < best[i+1]->size())
                    std::swap(best[i], best[i+1]);
            }
        }
        return std::accumulate(
                               best.begin(),
                               best.end() - 1,
                               Scalar(1),
                                [](Scalar accum, Circuit *c) { return accum * (c ? c->size() : 1UL); });
    }

    void merge(Distance &d) {
        auto merged = d.points.first->circuit;
        auto killed = d.points.second->circuit;
        if (merged != d.points.second->circuit) {
            merged->insert(merged->end(), killed->begin(), killed->end());
            for (auto j : *killed)
            j->circuit = merged;
        }
    }

    void part1(std::ostream &os) {
        size_t i;
        std::sort(distances.begin(), distances.end());
        for (i = 0; i < std::min(distances.size(), 1000UL); ++i) {
            merge(distances[i]);
        }
        os <<best(distances[0], i);
    }

    void part2(std::ostream &os) {
        size_t i;
        std::sort(distances.begin(), distances.end());
        for (i = 0;; ++i) {
            auto &d = distances[i];
            merge(d);
            if (d.points.first->circuit->size() == points.size()) {
                os << d.points.first->x * d.points.second->x;
                break;
            }
        }
    }
};

aoc::Case P1{"part1", [](std::istream &is, std::ostream &os)
    {
        Puzzle pz;
        pz.parse(is); 
        pz.part1(os);
    }
};

aoc::Case P2{"part2", [](std::istream &is, std::ostream &os)
    {
        Puzzle pz;
        pz.parse(is); 
        pz.part2(os);
    }
};

aoc::Case P3{"parseonly", [](std::istream &is, std::ostream &os)
    {
        Puzzle pz;
        pz.parse(is); 
    }
};


}
