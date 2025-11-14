use ori::prelude::*;

use crate::Data;

pub fn menu_button(data: &Data, idx: usize) -> impl View<Data> + use<> {
    button(
        icon(include_str!("icon/bars.svg")),
        move |data: &mut Data| {
            data.menus_open[idx] = !data.menus_open[idx];
        },
    )
    .css_class(if data.menus_open[idx] {
        "menu-button open"
    } else {
        "menu-button closed"
    })
}

pub fn menu(data: &Data) -> impl View<Data> + use<> {
    vbox(
        vbox(map(
            crate::media::media(&data.media),
            |data: &mut Data, map| map(&mut data.media),
        ))
        .css_class("sub-menu"),
    )
    .css_class("menu")
    .hexpand(true)
}
