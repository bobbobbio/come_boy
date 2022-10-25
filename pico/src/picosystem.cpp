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
  frect(int x, int y, int w, int h)
  {
      picosystem::frect(x, y, w, h);
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

  uint64_t
  now_us(void)
  {
      return time_us_64();
  }

}
