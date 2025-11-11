use std::{sync::Arc, thread, time::Duration};

use chrono::{DateTime, Local};
use futures_timer::Delay;
use hyprland::{
    data::{Client, Clients, Monitor, Monitors, Workspace, Workspaces},
    dispatch::{Dispatch, DispatchType},
    event_listener::EventListener,
    shared::HyprData,
};
use ori::prelude::*;

fn main() -> anyhow::Result<()> {
    App::new()
        .css(include_css!("bar.css"))
        .theme("Adwaita-dark")
        .run(Data::new(), ui)?;

    Ok(())
}

struct Data {
    workspaces: Vec<Option<Workspace>>,
    monitors: Vec<Monitor>,
    clients: Vec<Client>,
    time: DateTime<Local>,
}

impl Data {
    fn new() -> Self {
        Self {
            workspaces: Self::get_workspaces(),
            monitors: Monitors::get().unwrap().into_iter().collect(),
            clients: Clients::get().unwrap().into_iter().collect(),
            time: Local::now(),
        }
    }

    fn get_workspaces() -> Vec<Option<Workspace>> {
        let mut actual: Vec<_> = Workspaces::get().unwrap().into_iter().collect();
        let mut workspaces = Vec::new();

        for id in 1..=10 {
            if let Some(i) = actual.iter().position(|w| w.id == id) {
                workspaces.push(Some(actual.remove(i)));
            } else {
                workspaces.push(None);
            }
        }

        workspaces
    }
}

enum Message {
    ClientsChanged,
    MonitorsChanged,
    UpdateTime,
}

fn ui(data: &mut Data) -> impl Effect<Data> + use<> {
    let monitors = data
        .monitors
        .iter()
        .map(|m| monitor(data, m))
        .collect::<Vec<_>>();

    effects((
        effects(monitors),
        on_event(handle_event),
        task_with_proxy(update_time),
        task_with_proxy(register_listeners),
    ))
}

fn monitor(data: &Data, monitor: &Monitor) -> impl Effect<Data> + use<> {
    window(vline((
        vline(()).vexpand(true),
        workspaces(data, monitor)
            .valign(Align::Center)
            .vexpand(true),
        date_time(data.time).valign(Align::End).vexpand(true),
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

fn workspaces(data: &Data, monitor: &Monitor) -> impl View<Data> + use<> {
    vline(
        data.workspaces
            .iter()
            .enumerate()
            .map(|(i, w)| match w {
                Some(w) => any(workspace(data, monitor, w)),
                None => any(empty_workspace(i as i32 + 1)),
            })
            .collect::<Vec<_>>(),
    )
    .spacing(10)
}

fn empty_workspace(id: i32) -> impl View<Data> + use<> {
    let content = vline(()).width_request(28).height_request(28);

    button(clamp_width(28, clamp_height(28, content)), move |_| {
        Dispatch::call(DispatchType::Custom(
            "focusworkspaceoncurrentmonitor",
            &format!("{id}"),
        ))
        .unwrap();

        Action::new()
    })
    .css_class("workspace")
}

fn workspace(data: &Data, monitor: &Monitor, workspace: &Workspace) -> impl View<Data> + use<> {
    let clients = data
        .clients
        .iter()
        .filter(|client| client.workspace.id == workspace.id)
        .collect::<Vec<_>>();

    let mut icons = clients
        .iter()
        .filter_map(|client| {
            freedesktop_icons::lookup(&client.class)
                .with_cache()
                .with_size(48)
                .with_scale(2)
                .find()
        })
        .collect::<Vec<_>>();

    icons.sort_unstable();
    icons.dedup();

    let mut icons = icons.into_iter().map(picture).collect::<Vec<_>>();

    let content = match icons.len() {
        0 => any(hline(())),
        1 | 2 => any(hline(icons)),
        3 | 4 => any(vline((
            hline((icons.remove(0), icons.remove(0))),
            hline(icons),
        ))),
        _ => unreachable!(),
    };

    let content = content.width_request(28).height_request(28);

    let classes = if monitor.active_workspace.id == workspace.id {
        "workspace active"
    } else if data
        .monitors
        .iter()
        .any(|m| m.active_workspace.id == workspace.id)
    {
        "workspace other"
    } else {
        "workspace"
    };

    let workspace_id = workspace.id;

    button(clamp_width(28, clamp_height(28, content)), move |_| {
        Dispatch::call(DispatchType::Custom(
            "focusworkspaceoncurrentmonitor",
            &format!("{workspace_id}"),
        ))
        .unwrap();

        Action::new()
    })
    .css_class(classes)
}

fn date_time(time: DateTime<Local>) -> impl View<Data> + use<> {
    vline((label(time.format("%H")), label(time.format("%M")))).css_class("time")
}

fn handle_event(data: &mut Data, event: &mut Event) -> Action {
    match event.take() {
        Some(Message::MonitorsChanged) => {
            data.monitors = Monitors::get().unwrap().into_iter().collect();
            Action::rebuild()
        }

        Some(Message::ClientsChanged) => {
            data.clients = Clients::get().unwrap().into_iter().collect();
            Action::rebuild()
        }

        Some(Message::UpdateTime) => {
            data.time = Local::now();
            Action::rebuild()
        }

        None => Action::new(),
    }
}

async fn update_time(proxy: impl Proxy) {
    loop {
        Delay::new(Duration::from_secs(1)).await;
        proxy.event(Event::new(Message::UpdateTime, None));
    }
}

async fn register_listeners(proxy: impl Proxy) {
    thread::spawn(move || {
        let proxy = Arc::new(proxy);

        let mut listener = EventListener::new();

        listener.add_workspace_changed_handler({
            let proxy = proxy.clone();
            move |_| proxy.event(Event::new(Message::MonitorsChanged, None))
        });

        listener.add_window_opened_handler({
            let proxy = proxy.clone();
            move |_| proxy.event(Event::new(Message::ClientsChanged, None))
        });

        listener.add_window_moved_handler({
            let proxy = proxy.clone();
            move |_| proxy.event(Event::new(Message::ClientsChanged, None))
        });

        listener.add_window_closed_handler({
            let proxy = proxy.clone();
            move |_| proxy.event(Event::new(Message::ClientsChanged, None))
        });

        let _ = listener.start_listener();
    });
}
