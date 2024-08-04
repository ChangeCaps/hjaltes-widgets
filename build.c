#include <lute/lute.h>

void build(Build *build) {
    Target *exe = target(build, "hjaltes-widgets", BINARY);

    exe->warn = Wall | Wextra;

    package(exe, "gtk4");
    package(exe, "gtk4-layer-shell-0");
    package(exe, "libpulse");

    source(exe, "src");
}
