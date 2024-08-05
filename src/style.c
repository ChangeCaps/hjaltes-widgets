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
