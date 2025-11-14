use std::{cell::RefCell, collections::HashMap, path::PathBuf, sync::Arc, thread};

use hyprland::{
    data::{Client, Clients, Monitor, Monitors, Workspace, Workspaces},
    dispatch::{Dispatch, DispatchType},
    event_listener::EventListener,
    shared::HyprData,
};
use ori::prelude::*;

use crate::Data;

pub struct HyprlandData {
    pub workspace_mode: WorkspaceMode,
    pub workspaces: Vec<Option<Workspace>>,
    pub monitors: Vec<Monitor>,
    pub clients: Vec<Client>,
}

impl HyprlandData {
    pub fn new() -> Self {
        Self {
            workspace_mode: WorkspaceMode::Line,
            workspaces: Self::get_workspaces(),
            monitors: Self::get_monitors(),
            clients: Self::get_clients(),
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

    fn get_monitors() -> Vec<Monitor> {
        Monitors::get().unwrap().into_iter().collect()
    }

    fn get_clients() -> Vec<Client> {
        Clients::get().unwrap().into_iter().collect()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WorkspaceMode {
    Icons,
    Line,
}

pub fn workspaces(data: &HyprlandData, monitor: &Monitor) -> impl View<HyprlandData> + use<> {
    match data.workspace_mode {
        WorkspaceMode::Icons => any(vbox(
            data.workspaces
                .iter()
                .enumerate()
                .map(|(i, w)| match w {
                    Some(w) => any(workspace_icons(data, monitor, w)),
                    None => any(empty_workspace_icons(i as i32 + 1)),
                })
                .collect::<Vec<_>>(),
        )
        .spacing(10)
        .css_class("workspaces")),

        WorkspaceMode::Line => any(vbox(
            data.workspaces
                .iter()
                .enumerate()
                .map(|(i, w)| match w {
                    Some(w) => any(workspace_line(data, monitor, w)),
                    None => any(empty_workspace_line(i as i32 + 1)),
                })
                .collect::<Vec<_>>(),
        )
        .css_class("workspaces")),
    }
}

fn workspace_classes(
    data: &HyprlandData,
    monitor: &Monitor,
    workspace: &Workspace,
    base: &str,
) -> String {
    if monitor.active_workspace.id == workspace.id {
        format!("{base} active")
    } else if data
        .monitors
        .iter()
        .any(|m| m.active_workspace.id == workspace.id)
    {
        format!("{base} other")
    } else if workspace.windows > 0 {
        format!("{base} used")
    } else {
        String::from(base)
    }
}

fn empty_workspace_line(id: i32) -> impl View<HyprlandData> + use<> {
    clamp_width(
        0,
        button(hbox(()), move |_| {
            Dispatch::call(DispatchType::Custom(
                "focusworkspaceoncurrentmonitor",
                &format!("{id}"),
            ))
            .unwrap();

            Action::new()
        })
        .css_class("workspace-line"),
    )
}

fn workspace_line(
    data: &HyprlandData,
    monitor: &Monitor,
    workspace: &Workspace,
) -> impl View<HyprlandData> + use<> {
    let classes = workspace_classes(data, monitor, workspace, "workspace-line");
    let workspace_id = workspace.id;

    clamp_width(
        0,
        button(hbox(()), move |_| {
            Dispatch::call(DispatchType::Custom(
                "focusworkspaceoncurrentmonitor",
                &format!("{workspace_id}"),
            ))
            .unwrap();

            Action::new()
        })
        .css_class(classes),
    )
}

fn empty_workspace_icons(id: i32) -> impl View<HyprlandData> + use<> {
    let content = vbox(()).width_request(28).height_request(28);

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

fn workspace_icons(
    data: &HyprlandData,
    monitor: &Monitor,
    workspace: &Workspace,
) -> impl View<HyprlandData> + use<> {
    thread_local! {
        static CACHE: RefCell<HashMap<String, Option<PathBuf>>> = Default::default();
    }

    let clients = data
        .clients
        .iter()
        .filter(|client| client.workspace.id == workspace.id)
        .collect::<Vec<_>>();

    let mut icons = clients
        .iter()
        .filter_map(|client| {
            CACHE.with_borrow_mut(|cache| {
                cache
                    .entry(client.class.clone())
                    .or_insert_with(|| {
                        freedesktop_icons::lookup(&client.class)
                            .with_cache()
                            .with_size(48)
                            .with_scale(2)
                            .find()
                    })
                    .clone()
            })
        })
        .collect::<Vec<_>>();

    icons.sort_unstable();
    icons.dedup();

    let mut icons = icons.into_iter().map(picture).collect::<Vec<_>>();

    let content = match icons.len() {
        0 => any(hbox(())),
        1 | 2 => any(hbox(icons)),
        3 | 4 => any(vbox((
            hbox((icons.remove(0), icons.remove(0))),
            hbox(icons),
        ))),
        _ => unreachable!(),
    };

    let content = content.width_request(28).height_request(28);

    let classes = workspace_classes(data, monitor, workspace, "workspace-icons");

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

pub fn effect() -> impl Effect<Data> {
    effects((
        map(on_event(handle_event), |data: &mut Data, map| {
            map(&mut data.hyprland)
        }),
        task_with_proxy(register_listeners),
    ))
}

enum Message {
    ClientsChanged,
    WorkspacesChanged,
    MonitorsChanged,
}

fn handle_event(data: &mut HyprlandData, event: &mut Event) -> Action {
    match event.take() {
        Some(Message::WorkspacesChanged) => {
            data.workspaces = HyprlandData::get_workspaces();
            Action::rebuild()
        }

        Some(Message::MonitorsChanged) => {
            data.monitors = HyprlandData::get_monitors();
            Action::rebuild()
        }

        Some(Message::ClientsChanged) => {
            data.clients = HyprlandData::get_clients();
            Action::rebuild()
        }

        None => Action::new(),
    }
}

async fn register_listeners(proxy: impl Proxy) {
    thread::spawn(move || {
        let proxy = Arc::new(proxy);

        let mut listener = EventListener::new();

        listener.add_workspace_changed_handler({
            let proxy = proxy.clone();
            move |_| {
                proxy.event(Event::new(Message::WorkspacesChanged, None));
                proxy.event(Event::new(Message::MonitorsChanged, None));
            }
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
