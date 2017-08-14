// Copyright (C) 2017 Chris Liebert

#include <assert.h>
#include <stdbool.h>
#include "example.h"

int main(int argc, char** argv) {
    print_hello();
    print_cstr("This is a c string");
    print_int(add_ints(5, 6));
    print_double(add_doubles(22.5, 3.34));
    IntVector3 iv3a;
    iv3a.x = 234;
    iv3a.y = 43;
    iv3a.z = 65;
    IntVector3 iv3b;
    iv3b.x = 743;
    iv3b.y = 34;
    iv3b.z = 64;
    IntVector3 iv3sum = add_int_vector3s(&iv3a, &iv3b);
    print_int(iv3sum.x);
    print_int(iv3sum.y);
    print_int(iv3sum.z);
    MyChoice choice = OPTION3;
    print_my_choice(&choice);
    return 0;
}
