#pragma once
#include <functional>

namespace aoc {
template <typename Scalar> struct Point {
   Scalar row{};
   Scalar col{};
   Point operator + (const Point &rhs) const noexcept { return { row + rhs.row, col + rhs.col }; }
   Point &operator += (const Point &rhs) noexcept {
      row += rhs.row;
      col += rhs.col;
      return *this;
   }
   Point operator - (const Point &rhs) const noexcept { return { row - rhs.row, col - rhs.col }; }
   Point operator - () const noexcept { return { -row, -col }; }
   Point operator * (Scalar m) const noexcept { return { row * m, col * m }; }
   Point abs () const { return { std::abs(row), std::abs(col) }; }
   auto operator <=> (const Point &rhs) const noexcept = default;
   bool operator == (const Point &rhs) const noexcept = default;
   unsigned magnitude() const noexcept { return std::abs(row) + std::abs(col); }
};

template <typename Scalar> struct Box {
   using P = Point<Scalar>;
   std::pair<P, P> extent {};
   Box(P a, P b) noexcept : extent { a, b } {}
   Box() noexcept : extent {} {}
   bool contains (const P &p) const noexcept {
      return p.row >= extent.first.row && p.col >= extent.first.col &&
         p.row < extent.second.row && p.col < extent.second.col;
   }
};
}

namespace std {
template <typename T> struct hash<aoc::Point<T>> {
   size_t operator() (const aoc::Point<T> &p) const noexcept { return p.row * 65537 + p.col; }
};
}
