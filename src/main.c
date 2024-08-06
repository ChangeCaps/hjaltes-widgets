// Copyright (C) 2024  Hjalte C. Nannestad
// See end of file for license information.

#include <stdio.h>
#include <string.h>

#include "config_menu.h"
#include "style.h"
#include "volume_popup.h"

int main(int argc, char **argv) {
    if (argc < 2) {
        printf("Usage: %s <command> [options]\n", argv[0]);
        printf("\n");
        printf("Commands:\n");
        printf("  volume-popup\n");
        printf("  config-menu\n");
        printf("\n");
        printf("Options:\n");
        printf("  --style <path>  Path to the CSS style file\n");
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

    if (strcmp(argv[1], "volume-popup") == 0)
        return volume_popup();
    else if (strcmp(argv[1], "config-menu") == 0)
        return config_menu();

    printf("Unknown command: %s\n", argv[1]);
    return 1;
}

// This file is part of Hjalte's Widgets.
// Copyright (C) 2024  Hjalte C. Nannestad
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
