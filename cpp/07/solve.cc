#include <numeric>
#include "aoc.h"

namespace {

class BitSeq {
public:
    using BitInt = uint64_t;
    BitSeq(size_t size) { resize(size); }
    BitSeq(const std::string &rhs, char one);
    BitSeq(const BitSeq &) = default;

    size_t size() const { return size_; }

    BitSeq operator | (const BitSeq &rhs) const { return bitop( rhs, [](BitInt lhs, BitInt rhs) { return lhs | rhs; } ); }
    BitSeq operator & (const BitSeq &rhs) const { return bitop( rhs, [](BitInt lhs, BitInt rhs) { return lhs & rhs; } ); }
    BitSeq operator ^ (const BitSeq &rhs) const { return bitop( *this, [](BitInt lhs, BitInt rhs) { return lhs ^ rhs; } ); }
    BitSeq operator ~ () const { return bitop( *this, [](BitInt lhs, BitInt ) { return ~lhs; } ); }
    BitSeq operator << (size_t qty) const { return shift(-int(qty)); }
    BitSeq operator >> (size_t qty) const { return shift(qty); }

    unsigned popcount() const {
        return std::accumulate(bitseq.begin(), bitseq.end(), BitInt(0),
                               [](BitInt accum, BitInt v){ return accum + __builtin_popcountll( v ); });
    }

    template <typename Op, typename ...Args>
    void foreachbit(Op op, Args... args) const {
        size_t wordsize = bitseq.size();
        for (size_t i = 0; i < wordsize; ++i) {
            auto val = bitseq[i] ;
            auto wordbits = i == wordsize - 1 ? size_ % BitsPerInt : BitsPerInt;
            while (val) {
                auto bit = __builtin_ctzll(val);
                if (bit >= wordbits)
                    break;
                op(bit + i * BitsPerInt, args...);
                val &= ~(BitInt(1)<<bit);
            }
        }
    }

    void resize(size_t offset) {
        size_t newsize = (offset + BitsPerInt - 1) / BitsPerInt;
        if (newsize > bitseq.size())
            bitseq.resize(newsize);
        if (offset >= size_)
            size_ = offset;
    }

    bool get(size_t offset) const {
        return (bitseq[offset / BitsPerInt] & (BitInt(1U) << (offset % BitsPerInt))) != 0;
    }

    void set(size_t offset) {
        resize(offset + 1);
        bitseq[offset / BitsPerInt] |= (BitInt(1U) << (offset % BitsPerInt));
    }

    void clr(size_t offset) {
        resize(offset + 1);
        bitseq[offset / BitsPerInt] &= ~(BitInt(1U) << (offset % BitsPerInt));
    }

    void put(size_t offset, bool what) {
        if (what)
            set(offset);
        else
            clr(offset);
    }
    auto operator<=>(const BitSeq &) const = default;

private:
    static constexpr unsigned BitsPerInt = sizeof(BitInt) * 8;
    std::vector<BitInt> bitseq{};
    size_t size_{};

    template <typename Op>
    BitSeq bitop(const BitSeq &rhs, Op op) const {
        BitSeq rv(size());
        for (size_t i = 0; i < bitseq.size(); ++i)
            rv.bitseq[i] = op( bitseq[i], rhs.bitseq[i]);
        return rv;
    }

    BitSeq shift( int shift ) const {
        BitSeq result(size());
        BitInt prev = 0, cur = bitseq[0], next = bitseq.size() ? bitseq[1] : 0;
        for (size_t i = 0;; ) {
            BitInt value = cur;
            if (shift < 0) {
                // we want the low bits from the next word in the high bits for this one
                value >>= abs(shift);
                value |= next << (BitsPerInt + shift);
            } else if (shift > 0) {
                // we want the high bits from the prev word in the low bits for this one
                value <<= shift;
                value |= prev >> (BitsPerInt - shift);
            }
            result.bitseq[i] = value;
            prev = cur;
            cur = next;
            if (++i == bitseq.size())
                break;
            next = i <= bitseq.size() ? bitseq[i+1] : 0;
        }
        return result;
    }
};

BitSeq::BitSeq(const std::string &s, char c) {
    size_t i = 0;
    for (auto in : s) {
        put(i, in == c);
        ++i;
    }
}

void part1(std::istream &is, std::ostream &os) {
    std::string l;
    std::getline(is, l);
    BitSeq flow{l, 'S'};
    unsigned branches = 0;
    while (std::getline(is, l)) {
        BitSeq seq{l, '^'};
        BitSeq collide = seq & flow;
        branches += collide.popcount();
        flow = flow & ~collide | collide << 1 | collide >> 1;
    }
    os << branches;
}

void part2(std::istream &is, std::ostream &os) {
    std::string l;
    std::getline(is, l);
    BitSeq flow{l, 'S'};
    std::vector<uint64_t> counts(flow.size());
    flow.foreachbit( [&counts] (size_t bit) { counts[bit] = 1; });
    while ( std::getline(is, l) ) {
        BitSeq seq{l, '^'};
        std::vector<uint64_t> next = counts;
        seq.foreachbit([&](int bit) {
            if (counts[bit]) {
                next[bit-1] += counts[bit];
                next[bit+1] += counts[bit];
                next[bit] -= counts[bit];
            }
        });
        counts = next;
    }
    std::cout << std::accumulate(counts.begin(), counts.end(), uint64_t(0));
}

aoc::Case P1{"part1", part1};
aoc::Case P2{"part2", part2};
}
