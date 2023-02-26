#include "picosystem.hpp"
#include "pico/multicore.h"
#include "pico/mutex.h"

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

  void
  launch_core1(void(*func)(void))
  {
      multicore_launch_core1(func);
  }

  void
  pico_mutex_init(struct pico_mutex *self)
  {
      mutex_init((mutex_t *)self);
  }

  void
  pico_mutex_enter_blocking(struct pico_mutex *self)
  {
      mutex_enter_blocking((mutex_t *)self);
  }

  void
  pico_mutex_exit(struct pico_mutex *self)
  {
      mutex_exit((mutex_t *)self);
  }
}

static_assert(sizeof(struct pico_mutex) == sizeof(mutex_t));
static_assert(alignof(struct pico_mutex) == alignof(mutex_t));
