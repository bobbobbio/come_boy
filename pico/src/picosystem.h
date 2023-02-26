#include <stdbool.h>
#include <stdint.h>

void
pen(unsigned char r, unsigned char g, unsigned char b);

void
blend_copy(void);

void
clear(void);

void
text(const char *c, int x, int y);

void
wait_vsync(void);

void
flip(void);

bool
button(unsigned b);

struct buffer {
  int32_t w;
  int32_t h;
  void *data;
};

struct buffer *
target_buffer(void);

uint64_t
now_us(void);

void
frect(int x, int y, int w, int h);

void
launch_core1(void(*func)(void));

struct pico_mutex {
    unsigned long data[2];
};

void
pico_mutex_init(struct pico_mutex *);

void
pico_mutex_enter_blocking(struct pico_mutex *);

void
pico_mutex_exit(struct pico_mutex *);
