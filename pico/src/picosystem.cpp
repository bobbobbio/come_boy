#include "picosystem.hpp"

extern "C" {
    #include "picosystem.h"

  void
  pen(unsigned char r, unsigned char g, unsigned char b, unsigned char a)
  {
      picosystem::pen(r, g, b, a);
  }

  void
  hline(int x, int y, int l)
  {
      picosystem::hline(x, y, l);
  }

  void
  vline(int x, int y, int l)
  {
      picosystem::vline(x, y, l);
  }

  void
  clear(void)
  {
      picosystem::clear();
  }

  void
  text(const char *c, int x, int y)
  {
      std::string msg(c);
      picosystem::text(msg, x, y);
  }
}
