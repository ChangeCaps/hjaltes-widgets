// Copyright (C) 2024  Hjalte C. Nannestad
// See end of file for license information.

#include <gtk/gtk.h>
#include <gtk4-layer-shell.h>
#include <pthread.h>
#include <pulse/pulseaudio.h>
#include <time.h>

#include "style.h"
#include "volume_popup.h"

#define POPUP_TIMEOUT 0.5

typedef struct {
    GtkWindow *window;
    GtkWidget *progress;
    GtkWidget *label;
    struct timespec last_update;
} Data;

static GtkWidget *build_ui(Data *data) {
    GtkWidget *box = gtk_box_new(GTK_ORIENTATION_HORIZONTAL, 16);

    data->progress = gtk_progress_bar_new();
    gtk_progress_bar_set_fraction(GTK_PROGRESS_BAR(data->progress), 0.5);
    gtk_widget_set_size_request(data->progress, 200, 0);
    gtk_widget_set_valign(data->progress, GTK_ALIGN_CENTER);
    gtk_widget_set_name(data->progress, "volume-bar");
    gtk_box_append(GTK_BOX(box), data->progress);

    data->label = gtk_label_new(" ");
    gtk_widget_set_name(data->label, "volume-icon");
    gtk_box_append(GTK_BOX(box), data->label);

    return box;
}

static void init_window(GtkWindow *window) {
    // initialize the gtk layer shell
    gtk_layer_init_for_window(window);
    gtk_layer_set_layer(window, GTK_LAYER_SHELL_LAYER_OVERLAY);

    // set the margin and anchor
    gtk_layer_set_anchor(window, GTK_LAYER_SHELL_EDGE_TOP, true);
    gtk_layer_set_margin(window, GTK_LAYER_SHELL_EDGE_TOP, 20);

    gtk_widget_set_name(GTK_WIDGET(window), "volume-popup");

    gtk_widget_set_valign(GTK_WIDGET(window), GTK_ALIGN_CENTER);
    gtk_widget_set_halign(GTK_WIDGET(window), GTK_ALIGN_CENTER);
}

static void activate(GtkApplication *app, Data *data) {
    // create the gtk window
    data->window = GTK_WINDOW(gtk_application_window_new(app));
    init_window(data->window);

    // add the css provider
    add_css_provider();

    // build the ui
    GtkWidget *ui = build_ui(data);

    // add the ui to the window and present it
    gtk_window_set_child(data->window, ui);
    gtk_window_present(data->window);

    // initially hide the window
    gtk_widget_set_visible(GTK_WIDGET(data->window), false);
}

// callback for the volume popup timeout
static void volume_timeout(void *userdata) {
    Data *data = userdata;

    // calculate the time since the last update
    struct timespec now;
    clock_gettime(CLOCK_MONOTONIC, &now);

    float diff = (now.tv_sec - data->last_update.tv_sec);
    diff += (now.tv_nsec - data->last_update.tv_nsec) / 1e9;

    // if the time since the last update is greater than the timeout,
    // hide the window
    if (diff >= POPUP_TIMEOUT) {
        gtk_widget_set_visible(GTK_WIDGET(data->window), false);
    }
}

// callback for the pulseaudio sink info
static void volume_callback(pa_context *cx, const pa_sink_info *info, int eol,
                            void *userdata) {
    (void)cx;
    (void)eol;

    Data *data = userdata;

    // if info is NULL, or the progress bar doesn't exist, we can't do anything
    // with the data
    if (!info || !data->progress) {
        return;
    }

    // get the progress bar
    GtkProgressBar *progress = GTK_PROGRESS_BAR(data->progress);

    // calculate the volume
    double volume = (double)info->volume.values[0] / PA_VOLUME_NORM;
    double current = gtk_progress_bar_get_fraction(progress);

    // if the volume hasn't changed, we don't show the popup
    if (fabs(volume - current) < 0.01) {
        return;
    }

    // update the progress bar
    gtk_progress_bar_set_fraction(progress, volume);

    // update the last update time
    clock_gettime(CLOCK_MONOTONIC, &data->last_update);

    // show the window and start the timeout
    gtk_widget_set_visible(GTK_WIDGET(data->window), true);
    g_timeout_add_once(500, volume_timeout, data);
}

static void subscribe_callback(pa_context *cx, pa_subscription_event_type_t t,
                               uint32_t idx, void *userdata) {
    Data *data = userdata;

    if ((t & PA_SUBSCRIPTION_EVENT_FACILITY_MASK) ==
        PA_SUBSCRIPTION_EVENT_SINK) {
        pa_operation *op =
            pa_context_get_sink_info_by_index(cx, idx, volume_callback, data);

        if (op) {
            pa_operation_unref(op);
        }
    }
}

static void state_callback(pa_context *cx, void *userdata) {
    switch (pa_context_get_state(cx)) {
    case PA_CONTEXT_READY:
        // subscribe to sink events
        pa_context_set_subscribe_callback(cx, subscribe_callback, userdata);
        pa_operation *op =
            pa_context_subscribe(cx, PA_SUBSCRIPTION_MASK_SINK, NULL, NULL);

        if (op) {
            pa_operation_unref(op);
        }

        break;
    case PA_CONTEXT_FAILED:
    case PA_CONTEXT_TERMINATED:
        printf("Failed to connect to PulseAudio\n");
        break;
    default:
        break;
    }
}

// pulseaudio mainloop thread function
static void *mainloop_thread(void *arg) {
    // we just run the mainloop
    pa_mainloop *mainloop = arg;
    pa_mainloop_run(mainloop, NULL);
    return NULL;
}

int run_gtk_app(Data *data) {
    // create the gtk application
    GtkApplication *app = gtk_application_new("org.hjalte.widgets.volume-popup",
                                              G_APPLICATION_DEFAULT_FLAGS);

    // connect the activate signal and run the application
    g_signal_connect(app, "activate", G_CALLBACK(activate), data);
    int status = g_application_run(G_APPLICATION(app), 0, NULL);
    g_object_unref(app);

    return status;
}

int volume_popup() {
    // initialize the pulseaudio mainloop and context
    pa_mainloop *mainloop = pa_mainloop_new();
    pa_mainloop_api *mainloop_api = pa_mainloop_get_api(mainloop);
    pa_context *context = pa_context_new(mainloop_api, "hjaltes-widgets");

    // initialize the data struct
    Data data = {0};

    // setup the pulseaudio callbacks
    pa_context_set_state_callback(context, state_callback, &data);
    if (pa_context_connect(context, NULL, PA_CONTEXT_NOFLAGS, NULL) < 0) {
        printf("Failed to connect to PulseAudio\n");
        return 1;
    }

    // start the pulseaudio mainloop in a separate thread
    pthread_t thread;
    pthread_create(&thread, NULL, mainloop_thread, mainloop);

    // run the gtk application
    int status = run_gtk_app(&data);

    // quit the pulseaudio mainloop, and join the thread
    pa_mainloop_quit(mainloop, 0);
    pthread_join(thread, NULL);

    // cleanup the pulseaudio context and mainloop
    pa_context_unref(context);
    pa_mainloop_free(mainloop);

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
