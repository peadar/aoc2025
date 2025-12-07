#include "aoc.h"
#include <iostream>

namespace aoc {
std::ostream &operator << (std::ostream &os, Utf8 cp) {
   uint8_t buf[8], *utf8 = buf + 8;
   for (uint8_t valmask = 0x7f, pfx = 0x00;;) {
      if ((cp.value & valmask) == cp.value) {
         *--utf8 = cp.value | pfx;
         break;
      }
      // this byte won't fit. Output "10" + top 6 bits.
      *--utf8 = (cp.value & 0x3f) | 0x80;
      if (pfx == 0) {
         // first octet: start mask at 110
         pfx = 0xc0;
         valmask = 0x1f;
      } else {
         pfx = (pfx >> 1) | 0x80;
         valmask = valmask >> 1;
      }
      cp.value >>=6; // We've written 6 bits.
   }
   os.rdbuf()->sputn((const char *)utf8, 8 - (utf8 - buf));
   return os;
}
}
