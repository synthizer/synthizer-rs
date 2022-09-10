cmake_minimum_required(VERSION 3.15.0)
project(syz_deps C CXX)

add_library(miniaudio INTERFACE)
target_include_directories(miniaudio INTERFACE "${CMAKE_CURRENT_SOURCE_DIR}")
