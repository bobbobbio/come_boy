#include <stdbool.h>
#include <stdint.h>

void
pen(unsigned char r, unsigned char g, unsigned char b);

void
blend_copy(void);

void
clear(void);

void
hline(int x, int y, int l);

void
vline(int x, int y, int l);

void
text(const char *c, int x, int y);

void
pixel(int x, int y);

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
