// Copyright (C) 2017 Chris Liebert

#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include "example.h"

int main(int argc, char** argv) {
    assert(add_ints(3, 7) == 10);
    assert(add_ints(-5, 4) == -1);
    assert(add_doubles(-10.0, 10.0) == 0.0);
    IntVector3 iv3a;
    iv3a.x = 5;
    iv3a.y = 6;
    iv3a.z = 7;
    IntVector3 iv3b;
    iv3b.x = 8;
	iv3b.y = 9;
	iv3b.z = 10;
	IntVector3 iv3sum = add_int_vector3s(&iv3a, &iv3b);
	assert(iv3sum.x = 13);
	assert(iv3sum.y = 15);
	assert(iv3sum.z = 17);
	MyChoice choice = OPTION1;
	assert(!is_option2(&choice));
	choice = OPTION2;
	assert(is_option2(&choice));
	choice = OPTION3;
	assert(!is_option2(&choice));
    puts("Success\n");
    return 0;
}
