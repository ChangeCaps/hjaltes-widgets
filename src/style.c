// Copyright (C) 2024  Hjalte C. Nannestad
// See end of file for license information.

#include <gtk/gtk.h>

#include "style.h"

#define BUFFER_SIZE 1024

const char *argument_style_path = NULL;

void add_css_provider() {
    char style_path[BUFFER_SIZE];

    if (argument_style_path) {
        snprintf(style_path, BUFFER_SIZE, "%s", argument_style_path);
    } else if (getenv("XDG_CONFIG_HOME")) {
        snprintf(style_path, BUFFER_SIZE, "%s/hjaltes-widgets/style.css",
                 getenv("XDG_CONFIG_HOME"));
    } else if (getenv("HOME")) {
        snprintf(style_path, BUFFER_SIZE,
                 "%s/.config/hjaltes-widgets/style.css", getenv("HOME"));
    } else {
        printf("Could not find config directory.\n");
        return;
    }

    GtkCssProvider *provider = gtk_css_provider_new();
    gtk_css_provider_load_from_path(provider, style_path);
    gtk_style_context_add_provider_for_display(
        gdk_display_get_default(), GTK_STYLE_PROVIDER(provider),
        GTK_STYLE_PROVIDER_PRIORITY_USER);
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
