mod types;
mod features;
mod data;
mod components;
mod api;

use yew::{component, html, use_effect_with, use_state, Callback, Html};
use yew::platform::spawn_local;
use crate::api::fetch_videos;
use crate::components::VideoDetails;
use crate::features::VideoList;
use crate::types::Video;

#[component]
fn App() -> Html {
  let videos = use_state(|| vec![]);
  let selected_video = use_state(|| None);

  let on_select = {
    let selected_video = selected_video.clone();
    Callback::from(move |video: Video| {
      selected_video.set(Some(video));
    })
  };

  {
    let videos = videos.clone();
    use_effect_with((), move |_| {
      let videos = videos.clone();
      spawn_local(async move {
        let fetched_videos: Vec<Video> = fetch_videos().await;
        videos.set(fetched_videos);
      }) ;
      || ()
    });
  }

  html! {
    <div>
      <h1>{ "Videos" }</h1>
      <VideoList videos={(*videos).clone()} on_click={on_select}/>
      if let Some(video) = &*selected_video {
        <VideoDetails video={video.clone()}/>
      }
    </div>
  }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
