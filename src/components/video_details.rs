use yew::{component, html, Html, Properties};
use crate::types::Video;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub(crate) video: Video
}
#[component]
pub fn VideoDetails(Props { video }: &Props) -> Html {
  html! {
    <div class="VideoDetails">
      <h2 class="video-title">{ &video.title }</h2>
      <video controls={true} src={&video.url} class="video-player"/>
    </div>
  }
}