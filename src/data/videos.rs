use std::convert::Into;
use crate::types::Video;

pub fn get_sample_videos() -> Vec<Video> {
    vec![
        Video {
            id: 1,
            title: "Understanding Rust Lifetimes".into(),
            speaker: "Alice Johnson".into(),
            url: "https://example.com/rust-lifetimes".into(),
        },
        Video {
            id: 2,
            title: "Advanced Yew Framework Techniques".into(),
            speaker: "Bob Smith".into(),
            url: "https://example.com/yew-advanced".into(),
        },
        Video {
            id: 3,
            title: "Building Web Apps with Rust and Yew".into(),
            speaker: "Carol Davis".into(),
            url: "https://example.com/rust-yew-webapps".into(),
        },
    ]
}