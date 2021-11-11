cmake_minimum_required(VERSION 3.12)

# Pull in PICO SDK (must be before project)
include(pico-sdk/pico_sdk_init.cmake)

project(come_boy C CXX ASM)
set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)

# Initialize the SDK
pico_sdk_init()

# Include the PicoSystem library
include(picosystem/libraries/picosystem.cmake REQUIRED)

picosystem_executable(
    come_boy
    src/main.cpp
    src/picosystem.cpp
)

disable_startup_logo(come_boy)
target_link_libraries(
    come_boy ${CMAKE_SOURCE_DIR}/../target/thumbv6m-none-eabi/release/libcome_boy_pico.a
)
