use yew::{component, html, Html, Properties};
use crate::types::Video;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub(crate) video: Video
}
#[component]
pub fn VideoDetails(Props { video }: &Props) -> Html {
  html! {
    <div>
      <h2>{ &video.title }</h2>
      <video controls={true} src={&video.url} />
    </div>
  }
}