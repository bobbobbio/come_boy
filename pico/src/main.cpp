#include "picosystem.hpp"

namespace rust {
    extern "C" {
        void init();
        void update(uint32_t tick);
        void draw(uint32_t tick);
    }
}

void init() {
    rust::init();
}

void update(uint32_t tick) {
    rust::update(tick);
}

void draw(uint32_t tick) {
    rust::draw(tick);
}
