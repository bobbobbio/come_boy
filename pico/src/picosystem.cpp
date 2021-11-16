#include "picosystem.hpp"

extern "C" {
    #include "picosystem.h"

  void
  pen(unsigned char r, unsigned char g, unsigned char b)
  {
      picosystem::pen(r, g, b);
  }

  void
  blend_copy(void)
  {
      picosystem::blend(picosystem::COPY);
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
      picosystem::text(msg, x, y, 240);
  }

  void
  pixel(int x, int y)
  {
      picosystem::pixel(x, y);
  }

  void
  wait_vsync(void)
  {
      picosystem::_wait_vsync();
  }

  void
  flip(void)
  {
      picosystem::_flip();
  }

  bool
  button(unsigned b)
  {
      return picosystem::button(b);
  }

  struct buffer *
  target_buffer(void)
  {
      return (struct buffer *)picosystem::_dt;
  }

}
