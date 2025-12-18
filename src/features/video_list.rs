use yew::{component, html, Callback, Html, Properties};
use crate::types::Video;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub(crate) videos: Vec<Video>,
  pub(crate) on_click: Callback<Video>
}

#[component]
pub fn VideoList(Props { videos, on_click }: &Props) -> Html {
  let on_select = |video: &Video| {
    let on_click = on_click.clone();
    let video = video.clone();
    Callback::from(move |_| {
      on_click.emit(video.clone());
    })
  };

  html! {
    <ul class="video-list">
      { for videos.iter().map(|video| html! {
        <li key={video.id} onclick={on_select(video)}>
          {format!("{} - {}", video.title, video.speaker)}
        </li>
      }) }
    </ul>
  }
}