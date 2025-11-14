use ::hyprland::data::Monitor;
use chrono::{DateTime, Local};
use ori::prelude::*;

use crate::{hyprland::HyprlandData, media::MediaData};

mod hyprland;
mod media;
mod menu;
mod time;

fn main() -> anyhow::Result<()> {
    App::new()
        .css(include_css!("main.css"))
        .theme("Adwaita-dark")
        .run(Data::new(), ui)?;

    Ok(())
}

struct Data {
    menus_open: Vec<bool>,
    time: DateTime<Local>,

    hyprland: HyprlandData,
    media: MediaData,
}

impl Data {
    fn new() -> Self {
        let hyprland = HyprlandData::new();
        let media = MediaData::new();

        Self {
            menus_open: vec![false; hyprland.monitors.len()],
            time: Local::now(),

            hyprland,
            media,
        }
    }

    fn menu_open(&self) -> bool {
        self.menus_open.iter().any(|open| *open)
    }
}

fn ui(data: &mut Data) -> impl Effect<Data> + use<> {
    effects((
        monitors(data),
        time::effect(),
        hyprland::effect(),
        media::effect(),
    ))
}

fn monitors(data: &Data) -> impl Effect<Data> + use<> {
    effects(
        data.hyprland
            .monitors
            .iter()
            .enumerate()
            .map(|(i, m)| monitor(data, m, i))
            .collect::<Vec<_>>(),
    )
}

fn monitor(data: &Data, monitor: &Monitor, idx: usize) -> impl Effect<Data> + use<> {
    window(hbox((
        data.menus_open[idx].then(|| menu::menu(data)),
        bar(data, monitor, idx),
    )))
    .title("Bar")
    .width(1)
    .is_layer_shell(true)
    .exclusive_zone(Exclusive::Auto)
    .monitor(Some(monitor.id as u32))
    .anchor_top(true)
    .anchor_left(true)
    .anchor_bottom(true)
}

fn bar(data: &Data, monitor: &Monitor, idx: usize) -> impl View<Data> + use<> {
    vbox((
        vbox(menu::menu_button(data, idx)).vexpand(true),
        map(
            hyprland::workspaces(&data.hyprland, monitor)
                .valign(Align::Center)
                .vexpand(true),
            |data: &mut Data, map| map(&mut data.hyprland),
        ),
        time::date_time(data.time).valign(Align::End).vexpand(true),
    ))
    .css_class("bar")
}
