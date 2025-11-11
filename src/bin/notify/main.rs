mod bus;
mod dbus;

use ori::prelude::*;

use dbus::{Message, Notification, Urgency};

use dbus::CloseReason;

fn main() -> anyhow::Result<()> {
    App::new()
        .css(include_css!("notify.css"))
        .theme("Adwaita-dark")
        .run(Data::new(), ui)?;

    Ok(())
}

struct Data {
    connection: Option<dbus::Connection>,
    notifications: Vec<Notification>,
}

impl Data {
    fn new() -> Self {
        Self {
            connection: None,
            notifications: Vec::new(),
        }
    }
}

fn ui(data: &mut Data) -> impl Effect<Data> + use<> {
    let view = vline(
        data.notifications
            .iter()
            .map(notification)
            .collect::<Vec<_>>(),
    );

    let on_event = on_event(|data: &mut Data, event| match event.take::<Message>() {
        Some(Message::Connected(connection)) => {
            data.connection = Some(connection);

            Action::new()
        }

        Some(Message::Notification(notification)) => {
            if let Some(n) = data
                .notifications
                .iter_mut()
                .find(|n| n.id == notification.id)
            {
                *n = *notification;
            } else {
                data.notifications.insert(0, *notification);
            }

            Action::rebuild()
        }

        Some(Message::Close(id, generation, reason)) => {
            let index = data.notifications.iter().position(|n| {
                let is_id = n.id == id;
                let is_gen = generation.is_none_or(|g| g == n.generation);

                is_id && is_gen
            });

            if let Some(index) = index {
                data.notifications.remove(index);

                if let Some(ref conn) = data.connection {
                    conn.notification_closed(id, reason);
                }

                Action::rebuild()
            } else {
                Action::new()
            }
        }

        None => Action::new(),
    });

    let task = task_with_proxy(dbus::task);

    let window = window(view)
        .visible(!data.notifications.is_empty())
        .is_layer_shell(true)
        .layer(Layer::Overlay)
        .anchor_top(true)
        .anchor_right(true);

    effects((on_event, task, window))
}

fn notification(notification: &Notification) -> impl View<Data> + use<> {
    let mut view = hline(Vec::new());

    if let Some(ref hint_image) = notification.hint_image {
        let app_icon = clamp_width(75, picture(hint_image.clone()).css_class("hint-image"));
        view.push(any(app_icon));
    }

    view.push(any(notification_header(notification)
        .valign(Align::Start)
        .hexpand(true)));

    let mut actions = hline(Vec::new()).spacing(10);

    for action in &notification.actions {
        let action = button(label(&action[1]), {
            let id = notification.id;
            let key = action[0].clone();

            move |data: &mut Data| {
                let mut action = Action::new();

                if let Some(ref conn) = data.connection {
                    let message = Message::Close(id, None, CloseReason::Dismissed);
                    action.add_event(Event::new(message, None));

                    conn.action_invoked(id, key.clone());
                }
            }
        })
        .css_class("action")
        .halign(Align::Fill)
        .hexpand(true);

        actions.push(any(action));
    }

    let view = vline((view, actions));

    let classes = match notification.urgency {
        Urgency::Low => "notification low",
        Urgency::Normal => "notification normal",
        Urgency::Critical => "notification critical",
    };

    clamp_width(400, view.css_class(classes)).width_request(400)
}

fn notification_header(notification: &Notification) -> impl View<Data> + use<> {
    let mut app_line = hline(Vec::new());

    if let Some(ref app_icon) = notification.app_icon {
        let app_icon = picture(app_icon.clone()).css_class("app-icon");
        app_line.push(any(clamp_width(10, app_icon)));
    }

    let app_name = label(&notification.app_name)
        .css_class("app-name")
        .valign(Align::Center);

    app_line.push(any(app_name));

    let summary = label(&notification.summary)
        .markup(true)
        .ellipsize(Ellipsize::End)
        .css_class("summary");

    let time = label(notification.time.format("%H:%M"))
        .css_class("time")
        .valign(Align::Center);

    let header = vline((
        hline((
            app_line.halign(Align::Start),
            time.halign(Align::End).hexpand(true),
        )),
        summary.halign(Align::Start),
    ));

    let body = label(&notification.body)
        .markup(true)
        .wrap(Wrap::Word)
        .css_class("body");

    vline((header, body.halign(Align::Start))).css_class("header")
}
