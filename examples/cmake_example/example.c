// Copyright (C) 2017 Chris Liebert

#include <assert.h>
#include "example.h"

int main(int argc, char** argv) {
    print_hello();
    print_cstr("This is a c string");
    print_int(add_ints(5, 6));
    print_double(add_doubles(22.5, 3.34));
    return 0;
}
