use std::time::Duration;

use chrono::{DateTime, Local};
use futures_timer::Delay;
use ori::prelude::*;

use crate::Data;

pub fn date_time(time: DateTime<Local>) -> impl View<Data> + use<> {
    vbox((
        label(time.format("%H")).halign(Align::Center),
        label(time.format("%M")).halign(Align::Center),
    ))
    .css_class("time")
}

enum Message {
    UpdateTime,
}

pub fn effect() -> impl Effect<Data> + use<> {
    effects((on_event(handle_event), task_with_proxy(update_time)))
}

fn handle_event(data: &mut Data, event: &mut Event) -> Action {
    match event.take() {
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
