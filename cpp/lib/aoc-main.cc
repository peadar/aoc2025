#include "aoc.h"
#include <fstream>
#include <set>
#include <iterator>
#include <iostream>
#include <unistd.h>

namespace aoc {
std::vector<std::pair<std::string, Executor>> functions [[gnu::init_priority(1000)]];

Case::Case(std::string_view name, Executor callable) {
   functions.emplace_back(name, callable);
}

}

struct MemReader : std::streambuf {
   std::vector<char> vec;
   MemReader() {}
   MemReader(std::vector<char> &&vec) : vec(vec) {
      reset();
   }
   void reset() {
      setg(vec.data(), vec.data(), vec.data() + vec.size());
   }
   MemReader &operator = (std::vector<char> &&rhs) {
      vec = std::move(rhs);
      reset();
      return *this;
   }
};

struct DiscardingWriter : std::streambuf {
   int overflow(int val) override {
      return val == EOF ? EOF : 0;
   }
   std::streamsize xsputn( const char *, std::streamsize n ) override {
      return n;
   }
};

std::vector<char> bufferis(std::istream &is) {
      // read the content of the file into memory, and issue it from there.
      // copy content into vector of char.
      std::vector<char> buf;
      is.seekg(0);
      std::copy( std::istreambuf_iterator<char>(is.rdbuf()),
            std::istreambuf_iterator<char>(), std::back_inserter(buf));
      return buf;
}

int main(int argc, char *argv[]) {
   std::locale::global(std::locale(""));
   std::wcout.imbue(std::locale(""));
   bool do_timeit { false };
   std::set<std::string> parts;
   bool quiet { false };

   for (int c; (c = getopt(argc, argv, "tp:q")) != -1; ) {
      switch (c) {
         case 't':
            do_timeit = true;
            break;
         case 'p':
            parts.insert(optarg);
            break;
         case 'q':
            quiet = true;
            break;
      }
   }

   std::ifstream in( argv[optind], std::ifstream::binary);
   std::fstream null;
   MemReader inbuf;
   DiscardingWriter outbuf;
   std::function<aoc::Executor (aoc::Executor)> wrap;
   if (do_timeit) {
      inbuf = bufferis(in);
      wrap = [&] (aoc::Executor e) -> aoc::Executor {
         return [&, e](std::istream &, std::ostream &) {
            timeit([&, e] {
                  inbuf.reset();
                  std::istream memin(&inbuf);
                  std::ostream noout(&outbuf);
                  e(memin, noout);
                  });
         };
      };
   } else {
      wrap = [] (aoc::Executor e) { return e; };
   }

   for (auto &[ name, func ] : aoc::functions) {
      if (!parts.empty() && parts.find(name) == parts.end())
         continue;
      if (!quiet)
         std::cout << name << ": ";
      wrap(func)(in, std::cout);
      if (!quiet)
         std::cout << "\n";
      in.clear();
      in.seekg(0);
   }
}
