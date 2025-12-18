use serde::Deserialize;
use yew::AttrValue;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Video {
    pub id: usize,
    pub title: AttrValue,
    pub speaker: AttrValue,
    pub url: AttrValue,
}