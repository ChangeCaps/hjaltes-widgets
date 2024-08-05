#include <stdio.h>
#include <string.h>

#include "style.h"
#include "volume_popup.h"

int main(int argc, char **argv) {
    if (argc < 2) {
        printf("Usage: %s <command>\n", argv[0]);
        printf("Commands:\n");
        printf("  volume-popup\n");
        return 1;
    }

    for (int i = 0; i < argc; i++) {
        if (strcmp(argv[i], "--style") == 0) {
            if (i + 1 < argc) {
                argument_style_path = argv[i + 1];
            } else {
                printf("Missing argument for --style\n");
                return 1;
            }
        }
    }

    if (strcmp(argv[1], "volume-popup") == 0) {
        return volume_popup();
    }
}
