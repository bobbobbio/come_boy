#include "picosystem.hpp"

extern "C" {
    #include "picosystem.h"

  void
  pen(uint8_t r, uint8_t g, uint8_t b, uint8_t a)
  {
      picosystem::pen(r, g, b, a);
  }
}
