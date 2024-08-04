#include <stdio.h>
#include <string.h>

#include "volume_popup.h"

int main(int argc, char **argv) {
    if (argc < 2) {
        printf("Usage: %s <command>\n", argv[0]);
        printf("Commands:\n");
        printf("  volume-popup\n");
        return 1;
    }

    if (strcmp(argv[1], "volume-popup") == 0) {
        return volume_popup();
    }
}
