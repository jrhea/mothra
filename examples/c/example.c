#include "mothra.h"
#include <stdio.h>

int main (int argc, char** argv) {
    printf("FOO1\n");
    libp2p_start_bind(argv,argc);
}