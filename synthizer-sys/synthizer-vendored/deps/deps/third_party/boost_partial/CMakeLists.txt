cmake_minimum_required(VERSION 3.15.0)
project(syz_deps C CXX)

add_library(boost_partial INTERFACE)
target_include_directories(boost_partial INTERFACE "${CMAKE_CURRENT_SOURCE_DIR}")
