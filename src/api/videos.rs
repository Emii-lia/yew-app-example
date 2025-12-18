use gloo_net::http::Request;
use yew::platform::spawn_local;
use crate::types::Video;

pub async  fn fetch_videos() -> Vec<Video> {
  Request::get("/tutorial/data.json")
    .send()
    .await
    .unwrap()
    .json()
    .await
    .unwrap()
}