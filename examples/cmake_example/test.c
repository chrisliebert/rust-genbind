// Copyright (C) 2017 Chris Liebert

#include <assert.h>
#include <stdio.h>
#include "example.h"

int main(int argc, char** argv) {
    assert(add_ints(3, 7) == 10);
    assert(add_ints(-5, 4) == -1);
    puts("Success\n");
    return 0;
}
