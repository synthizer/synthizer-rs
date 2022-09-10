cmake_minimum_required(VERSION 3.15.0)
project(syz_deps C CXX)

add_library(hedley INTERFACE)
target_include_directories(hedley INTERFACE "${CMAKE_CURRENT_SOURCE_DIR}")
