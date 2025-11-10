use ori::prelude::*;

pub fn run() -> anyhow::Result<()> {
    App::new()
        .css(include_css!("bar.css"))
        .theme("Adwaita-dark")
        .run(Data::new(), ui)?;

    Ok(())
}

struct Data {}

impl Data {
    fn new() -> Self {
        Self {}
    }
}

fn ui(_data: &mut Data) -> impl Effect<Data> + use<> {
    window(label("app bar"))
        .title("Bar")
        .is_layer_shell(true)
        .exclusive_zone(Exclusive::Auto)
        .anchor_top(true)
        .anchor_left(true)
        .anchor_right(true)
}
