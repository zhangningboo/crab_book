#ifndef CPP_LIB_HPP
#define CPP_LIB_HPP

int add(int, int);

extern "C" {
    void cpp_add(int, int);
}

#endif