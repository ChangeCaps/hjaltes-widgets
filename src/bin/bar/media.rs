use std::{collections::HashSet, sync::Arc, thread, time::Duration};

use async_compat::Compat;
use futures_timer::Delay;
use mpris::{Metadata, PlaybackStatus, Player, PlayerFinder};
use ori::{core::AsyncContext, prelude::*};

use crate::Data;

pub struct MediaData {
    players: Vec<MediaPlayer>,
}

impl MediaData {
    pub fn new() -> Self {
        Self {
            players: Vec::new(),
        }
    }
}

struct MediaPlayer {
    mpris: Player,
    meta: Metadata,
    status: PlaybackStatus,
    position: Duration,
}

impl MediaPlayer {
    fn new(mpris: Player) -> Self {
        MediaPlayer {
            meta: mpris.get_metadata().unwrap(),
            status: mpris.get_playback_status().unwrap(),
            position: mpris.get_position().unwrap(),

            mpris,
        }
    }
}

pub fn media(data: &MediaData) -> impl View<MediaData> + use<> {
    let players: Vec<_> = data
        .players
        .iter()
        .enumerate()
        .filter(|(_, p)| p.meta.length().is_some())
        .flat_map(|(i, p)| {
            media_player(p).map(move |view| {
                map(view, move |data: &mut MediaData, map| {
                    if let Some(player) = data.players.get_mut(i) {
                        map(player);
                    }
                })
            })
        })
        .collect();

    vbox(players).spacing(16)
}

fn media_player(player: &MediaPlayer) -> Option<impl View<MediaPlayer> + use<>> {
    Some(
        vbox((
            hbox((
                art(&player.meta),
                vbox((title(&player.meta), artists(&player.meta))).valign(Align::Center),
            ))
            .valign(Align::Center)
            .hexpand(true),
            media_progress(player),
            hbox((
                hbox(current_time(player))
                    .hexpand(true)
                    .halign(Align::Start),
                controls(player).hexpand(true).halign(Align::Center),
                hbox(length(&player.meta)).hexpand(true).halign(Align::End),
            )),
        ))
        .css_class("media-player"),
    )
}

fn title(meta: &Metadata) -> Option<impl View<MediaPlayer> + use<>> {
    Some(
        label(meta.title()?)
            .ellipsize(Ellipsize::End)
            .css_class("media-title")
            .halign(Align::Start),
    )
}

fn artists(meta: &Metadata) -> Option<impl View<MediaPlayer> + use<>> {
    Some(
        label(meta.artists()?.join(", "))
            .ellipsize(Ellipsize::End)
            .css_class("media-artists")
            .halign(Align::Start),
    )
}

fn art(meta: &Metadata) -> Option<impl View<MediaPlayer> + use<>> {
    let url = meta.art_url()?.to_string();

    Some(memo(url.clone(), move |_| {
        let art = suspense(hbox(()), async move {
            match url.strip_prefix("file://") {
                Some(path) => picture(path),
                None => {
                    let bytes = Compat::new(reqwest::get(&url))
                        .await
                        .unwrap()
                        .bytes()
                        .await
                        .unwrap();

                    picture(image::load_from_memory(&bytes).unwrap().to_rgba8())
                }
            }
        })
        .css_class("media-art");

        clamp_width(0, clamp_height(0, art))
    }))
}

fn controls(player: &MediaPlayer) -> impl View<MediaPlayer> + use<> {
    hbox((
        backward_button(player),
        play_pause_button(player.status),
        forward_button(player),
    ))
}

fn media_progress(player: &MediaPlayer) -> impl View<MediaPlayer> + use<> {
    let fraction = player.position.as_secs_f32()
        / player
            .meta
            .length()
            .unwrap_or(player.position)
            .as_secs_f32();

    progress(fraction)
}

fn current_time(player: &MediaPlayer) -> Option<impl View<MediaPlayer> + use<>> {
    let position = chrono::Duration::from_std(player.position).ok()?;

    Some(
        label(format!(
            "{:02}:{:02}",
            position.num_minutes() % 60,
            position.num_seconds() % 60,
        ))
        .valign(Align::Start)
        .css_class("media-time current"),
    )
}

fn length(meta: &Metadata) -> Option<impl View<MediaPlayer> + use<>> {
    let position = chrono::Duration::from_std(meta.length()?).ok()?;

    Some(
        label(format!(
            "{:02}:{:02}",
            position.num_minutes() % 60,
            position.num_seconds() % 60,
        ))
        .valign(Align::Start)
        .css_class("media-time length"),
    )
}

fn play_pause_button(status: PlaybackStatus) -> impl View<MediaPlayer> + use<> {
    match status {
        PlaybackStatus::Playing => button(
            icon(include_str!("icon/pause.svg")),
            |data: &mut MediaPlayer| {
                let _ = data.mpris.pause();
            },
        ),

        PlaybackStatus::Paused => button(
            icon(include_str!("icon/play.svg")),
            |data: &mut MediaPlayer| {
                let _ = data.mpris.play();
            },
        ),

        PlaybackStatus::Stopped => button(icon(include_str!("icon/pause.svg")), |_| {}),
    }
}

fn backward_button(player: &MediaPlayer) -> Option<impl View<MediaPlayer> + use<>> {
    if !matches!(player.mpris.can_go_previous(), Ok(true)) {
        return None;
    }

    Some(button(
        icon(include_str!("icon/backward.svg")),
        |data: &mut MediaPlayer| {
            let _ = data.mpris.previous();
        },
    ))
}

fn forward_button(player: &MediaPlayer) -> Option<impl View<MediaPlayer> + use<>> {
    if !matches!(player.mpris.can_go_next(), Ok(true)) {
        return None;
    }

    Some(button(
        icon(include_str!("icon/forward.svg")),
        |data: &mut MediaPlayer| {
            let _ = data.mpris.next();
        },
    ))
}

enum Message {
    PollPlayers,
    Status(String, PlaybackStatus),
    Metadata(String, Metadata),
    Position(String, Duration),
}

pub fn effect() -> impl Effect<Data> + use<> {
    effects((
        build_with_context(|cx: &mut ori::Context, _| {
            let proxy: Arc<dyn Proxy> = Arc::new(cx.proxy());

            on_event(move |data, event| handle_event(data, event, &proxy))
        }),
        task_with_proxy(poll_players),
    ))
}

fn handle_event(data: &mut Data, event: &mut Event, proxy: &Arc<dyn Proxy>) -> Action {
    match event.take() {
        Some(Message::PollPlayers) => {
            let players = PlayerFinder::new().unwrap().iter_players().unwrap();

            // get unique name of all current players
            let mut names = data
                .media
                .players
                .iter()
                .map(|p| p.mpris.unique_name().to_string())
                .collect::<HashSet<_>>();

            for mpris in players {
                let mpris = mpris.unwrap();
                let name = mpris.unique_name().to_string();

                // mark player as present
                names.remove(&name);

                if data
                    .media
                    .players
                    .iter()
                    .any(|p| p.mpris.unique_name() == name)
                {
                    continue;
                }

                thread::spawn({
                    let proxy = proxy.clone();
                    let id = mpris.identity().to_string();

                    move || {
                        let mpris = PlayerFinder::new().unwrap().find_by_name(&id).unwrap();

                        for event in mpris.events().into_iter().flatten() {
                            let Ok(event) = event else { break };

                            match event {
                                mpris::Event::Paused
                                | mpris::Event::Playing
                                | mpris::Event::Stopped => {
                                    if let Ok(status) = mpris.get_playback_status() {
                                        proxy.event(Event::new(
                                            Message::Status(id.clone(), status),
                                            None,
                                        ));
                                    }

                                    if let Ok(meta) = mpris.get_metadata() {
                                        proxy.event(Event::new(
                                            Message::Metadata(id.clone(), meta),
                                            None,
                                        ));
                                    }
                                }

                                mpris::Event::PlaybackRateChanged(_)
                                | mpris::Event::TrackAdded(_)
                                | mpris::Event::TrackRemoved(_)
                                | mpris::Event::TrackChanged(_)
                                | mpris::Event::TrackMetadataChanged { .. }
                                | mpris::Event::TrackListReplaced => {
                                    if let Ok(meta) = mpris.get_metadata() {
                                        proxy.event(Event::new(
                                            Message::Metadata(id.clone(), meta),
                                            None,
                                        ));
                                    }
                                }

                                _ => {}
                            }
                        }
                    }
                });

                thread::spawn({
                    let proxy = proxy.clone();
                    let id = mpris.identity().to_string();

                    move || {
                        let player = PlayerFinder::new().unwrap().find_by_name(&id).unwrap();

                        loop {
                            thread::sleep(Duration::from_millis(50));

                            let Ok(position) = player.get_position() else {
                                break;
                            };

                            proxy.event(Event::new(Message::Position(id.clone(), position), None));
                        }
                    }
                });

                data.media.players.push(MediaPlayer::new(mpris));
            }

            // remove all players that aren't present
            data.media.players.retain(|p| {
                // check if name hasn't been updated
                !names.contains(p.mpris.unique_name())
            });

            Action::new().with_rebuild(data.menu_open())
        }

        Some(Message::Status(id, status)) => {
            for player in &mut data.media.players {
                if player.mpris.identity() == id {
                    player.status = status;
                    break;
                }
            }

            Action::new().with_rebuild(data.menu_open())
        }

        Some(Message::Metadata(id, meta)) => {
            for player in &mut data.media.players {
                if player.mpris.identity() == id {
                    player.meta = meta;
                    break;
                }
            }

            Action::new().with_rebuild(data.menu_open())
        }

        Some(Message::Position(id, position)) => {
            for player in &mut data.media.players {
                if player.mpris.identity() == id {
                    player.position = position;
                    break;
                }
            }

            Action::new().with_rebuild(data.menu_open())
        }

        None => Action::new(),
    }
}

async fn poll_players(proxy: impl Proxy) {
    loop {
        proxy.event(Event::new(Message::PollPlayers, None));
        Delay::new(Duration::from_secs(3)).await;
    }
}
