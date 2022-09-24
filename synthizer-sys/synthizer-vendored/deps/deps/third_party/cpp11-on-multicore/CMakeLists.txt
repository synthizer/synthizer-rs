cmake_minimum_required(VERSION 3.15.0)
project(syz_deps C CXX)

add_library(cpp11-on-multicore INTERFACE)
target_include_directories(cpp11-on-multicore INTERFACE "${CMAKE_CURRENT_SOURCE_DIR}/common")
