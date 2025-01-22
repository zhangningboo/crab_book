cmake_minimum_required(VERSION 3.0)
project(cpp_lib_test VERSION 1.0)
 
include_directories(${CMAKE_CURRENT_SOURCE_DIR}/inlcude)

# add_library(${PROJECT_NAME} STATIC cpp_lib.cpp)
set(DYNAMIC_LIB_NAME ${PROJECT_NAME}_dynamic)
set(STATIC_LIB_NAME ${PROJECT_NAME}_static)

add_library(${DYNAMIC_LIB_NAME} SHARED ${CMAKE_CURRENT_SOURCE_DIR}/src/cpplib.cpp)
add_library(${STATIC_LIB_NAME} STATIC ${CMAKE_CURRENT_SOURCE_DIR}/src/cpplib.cpp)

# 设置动态库的版本号
set_target_properties(${DYNAMIC_LIB_NAME} PROPERTIES
    VERSION 1.0
    SOVERSION 1
)

# 为动态库添加公开头文件目录
target_include_directories(${DYNAMIC_LIB_NAME} PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/include
)

# 设置动态库的版本号
set_target_properties(${STATIC_LIB_NAME} PROPERTIES
    VERSION 1.0
    SOVERSION 1
)

# 为动态库添加公开头文件目录
target_include_directories(${STATIC_LIB_NAME} PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/include
)