use hyprland::{
    data::{Workspace, Workspaces},
    shared::HyprData,
};
use ori::prelude::*;

pub fn run() -> anyhow::Result<()> {
    App::new()
        .css(include_css!("bar.css"))
        .theme("Adwaita-dark")
        .run(Data::new(), ui)?;

    Ok(())
}

struct Data {
    workspaces: Vec<Workspace>,
}

impl Data {
    fn new() -> Self {
        Self {
            workspaces: Vec::new(),
        }
    }
}

enum Message {
    Workspaces(Vec<Workspace>),
}

fn ui(data: &mut Data) -> impl Effect<Data> + use<> {
    let mut workspaces = vline(Vec::new());

    for workspace in &data.workspaces {
        workspaces.push(any(self::workspace(workspace)));
    }

    let window = window(workspaces)
        .title("Bar")
        .is_layer_shell(true)
        .exclusive_zone(Exclusive::Auto)
        .anchor_top(true)
        .anchor_left(true)
        .anchor_bottom(true);

    effects((window, task(get_workspaces()), on_event(handle_event)))
}

fn workspace(workspace: &Workspace) -> impl View<Data> + use<> {
    vline(())
}

fn handle_event(data: &mut Data, event: &mut Event) -> Action {
    match event.take() {
        Some(Message::Workspaces(workspaces)) => {
            data.workspaces = workspaces;

            Action::rebuild()
        }

        None => Action::new(),
    }
}

async fn get_workspaces() -> Event {
    let workspaces = Workspaces::get().unwrap().into_iter().collect();

    Event::new(Message::Workspaces(workspaces), None)
}
