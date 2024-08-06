// Copyright (C) 2024  Hjalte C. Nannestad
// See end of file for license information.

#include <gtk/gtk.h>
#include <gtk4-layer-shell.h>

#include "config_menu.h"
#include "style.h"

static void init_window(GtkWindow *window) {
    gtk_layer_init_for_window(window);
    gtk_layer_set_layer(window, GTK_LAYER_SHELL_LAYER_TOP);

    gtk_layer_set_anchor(window, GTK_LAYER_SHELL_EDGE_TOP, true);
    gtk_layer_set_anchor(window, GTK_LAYER_SHELL_EDGE_RIGHT, true);
    gtk_layer_set_margin(window, GTK_LAYER_SHELL_EDGE_TOP, 8);
    gtk_layer_set_margin(window, GTK_LAYER_SHELL_EDGE_RIGHT, 16);

    gtk_widget_set_name(GTK_WIDGET(window), "config-menu");

    gtk_widget_set_valign(GTK_WIDGET(window), GTK_ALIGN_CENTER);
    gtk_widget_set_halign(GTK_WIDGET(window), GTK_ALIGN_CENTER);
}

static void logout_clicked(GtkButton *button, void *data) {
    (void)button;
    (void)data;

    if (!system("hyprctl dispatch exit 0")) {
        g_print("Failed to log out\n");
    }
}

static void suspend_clicked(GtkButton *button, void *data) {
    (void)button;
    (void)data;

    if (!system("systemctl suspend")) {
        g_print("Failed to suspend\n");
    }
}

static void reboot_clicked(GtkButton *button, void *data) {
    (void)button;
    (void)data;

    if (!system("systemctl reboot")) {
        g_print("Failed to reboot\n");
    }
}

static void shutdown_clicked(GtkButton *button, void *data) {
    (void)button;
    (void)data;

    if (!system("systemctl poweroff")) {
        g_print("Failed to shut down\n");
    }
}

static void cancel_clicked(GtkButton *button, GtkWindow *window) {
    (void)button;

    gtk_window_close(window);
}

static GtkWidget *menu_button_new(const char *label) {
    GtkWidget *label_widget = gtk_label_new(label);
    gtk_widget_set_halign(label_widget, GTK_ALIGN_START);

    GtkWidget *button = gtk_button_new();
    gtk_button_set_child(GTK_BUTTON(button), label_widget);

    gtk_widget_set_halign(button, GTK_ALIGN_FILL);
    gtk_widget_set_valign(button, GTK_ALIGN_CENTER);

    gtk_widget_set_name(button, "config-menu-button");

    return button;
}

static GtkWidget *build_ui(GtkWindow *window) {
    GtkWidget *box = gtk_box_new(GTK_ORIENTATION_VERTICAL, 0);
    gtk_box_set_homogeneous(GTK_BOX(box), true);
    gtk_widget_set_halign(box, GTK_ALIGN_CENTER);
    gtk_widget_set_valign(box, GTK_ALIGN_CENTER);

    GtkWidget *logout = menu_button_new("Log out");
    GtkWidget *suspend = menu_button_new("Suspend");
    GtkWidget *reboot = menu_button_new("Reboot");
    GtkWidget *shutdown = menu_button_new("Shutdown");
    GtkWidget *cancel = menu_button_new("Cancel");

    g_signal_connect(logout, "clicked", G_CALLBACK(logout_clicked), NULL);
    g_signal_connect(suspend, "clicked", G_CALLBACK(suspend_clicked), NULL);
    g_signal_connect(reboot, "clicked", G_CALLBACK(reboot_clicked), NULL);
    g_signal_connect(shutdown, "clicked", G_CALLBACK(shutdown_clicked), NULL);
    g_signal_connect(cancel, "clicked", G_CALLBACK(cancel_clicked), window);

    gtk_box_append(GTK_BOX(box), logout);
    gtk_box_append(GTK_BOX(box), suspend);
    gtk_box_append(GTK_BOX(box), reboot);
    gtk_box_append(GTK_BOX(box), shutdown);
    gtk_box_append(GTK_BOX(box), cancel);

    return box;
}

static void right_click(GtkGestureClick *gesture, int n_press, double x,
                        double y, GtkWindow *window) {
    (void)gesture;
    (void)n_press;
    (void)x;
    (void)y;

    gtk_window_close(window);
}

static void activate(GtkApplication *app, void *data) {
    (void)data;

    GtkWindow *window = GTK_WINDOW(gtk_application_window_new(app));
    init_window(window);

    // add the css provider
    add_css_provider();

    GtkWidget *ui = build_ui(window);

    // we want to exit when the window is right clicked
    GtkGesture *click_gesture = gtk_gesture_click_new();
    gtk_gesture_single_set_button(GTK_GESTURE_SINGLE(click_gesture),
                                  GDK_BUTTON_SECONDARY);

    gtk_widget_add_controller(GTK_WIDGET(window),
                              GTK_EVENT_CONTROLLER(click_gesture));

    g_signal_connect(click_gesture, "pressed", G_CALLBACK(right_click), window);

    gtk_window_set_child(window, ui);
    gtk_window_present(window);
}

int config_menu() {
    GtkApplication *app = gtk_application_new("org.hjalte.widgets.config-menu",
                                              G_APPLICATION_DEFAULT_FLAGS);

    g_signal_connect(app, "activate", G_CALLBACK(activate), NULL);
    int status = g_application_run(G_APPLICATION(app), 0, NULL);
    g_object_unref(app);

    return status;
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
