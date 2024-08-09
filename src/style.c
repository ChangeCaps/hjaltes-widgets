// Copyright (C) 2024  Hjalte C. Nannestad
// See end of file for license information.

#include <gtk/gtk.h>

#include "style.h"

#define BUFFER_SIZE 1024

const char *style_path = NULL;

static bool get_style_path(char *path, int max_length) {
    if (style_path) {
        return snprintf(path, max_length, "%s", style_path);
    } else if (getenv("XDG_CONFIG_HOME")) {
        return snprintf(path, max_length, "%s/hjaltes-widgets/style.css",
                        getenv("XDG_CONFIG_HOME"));
    } else if (getenv("HOME")) {
        return snprintf(path, max_length,
                        "%s/.config/hjaltes-widgets/style.css", getenv("HOME"));
    } else {
        return false;
    }
}

static void file_watcher_cb(GFileMonitor *monitor, GFile *file, GFile *other,
                            GFileMonitorEvent event_type,
                            GtkCssProvider *provider) {
    (void)monitor;
    (void)file;
    (void)other;

    if (event_type == G_FILE_MONITOR_EVENT_CHANGES_DONE_HINT) {
        gtk_css_provider_load_from_path(provider, style_path);
    }
}

void add_css_provider() {
    // this function does leak memory, but it's not a big deal

    char new_path[BUFFER_SIZE];

    if (!get_style_path(new_path, BUFFER_SIZE)) {
        printf("Could not find style file\n");
        return;
    }

    style_path = strdup(new_path);
    GtkCssProvider *provider = gtk_css_provider_new();
    gtk_css_provider_load_from_path(provider, style_path);
    gtk_style_context_add_provider_for_display(
        gdk_display_get_default(), GTK_STYLE_PROVIDER(provider),
        GTK_STYLE_PROVIDER_PRIORITY_USER);

    GFile *file = g_file_new_for_path(style_path);
    GFileMonitor *monitor =
        g_file_monitor_file(file, G_FILE_MONITOR_NONE, NULL, NULL);

    g_signal_connect(monitor, "changed", G_CALLBACK(file_watcher_cb), provider);
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
