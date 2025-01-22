#include <iostream>
#include "cpplib.hpp"

int add(int a, int b) {
    return a + b;
}

extern "C" {
    void cpp_add(int a, int b) {
        std::cout << "you are calling cpp_add" << std::endl;
        add(a, b);
    };
}