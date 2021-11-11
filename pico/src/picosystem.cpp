#include "picosystem.hpp"

extern "C" {
    #include "picosystem.h"

  void
  pen(unsigned char r, unsigned char g, unsigned char b, unsigned char a)
  {
      picosystem::pen(r, g, b, a);
  }
}
