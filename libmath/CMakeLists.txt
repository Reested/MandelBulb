cmake_minimum_required(VERSION 3.0)
project(LibMath C)
add_library(math STATIC math.c)
install(TARGETS math DESTINATION .)