use dioxus::prelude::*;

use crate::components::story_listing::*;
use crate::components::models::*;
use crate::api::hackernews::*;

pub fn Stories(cx: Scope) -> Element {
    // Fetch the top 10 stories on Hackernews
    let stories = use_future(cx, (), |_| get_stories(10));

    // check if the future is resolved
    match stories.value() {
        Some(Ok(list)) => {
            // if it is, render the stories
            render! {
                div {
                    // iterate over the stories with a for loop
                    for story in list {
                        // render every story with the StoryListing component
                        StoryListing { story: story.clone() }
                    }
                }
            }
        }
        Some(Err(err)) => {
            // if there was an error, render the error
            render! {"An error occurred while fetching stories {err}"}
        }
        None => {
            // if the future is not resolved yet, render a loading message
            render! {"Loading items"}
        }
    }
}

pub fn Preview(cx: Scope) -> Element {
    let preview_state = use_shared_state::<PreviewState>(cx)?;
    match &*preview_state.read()  {
        PreviewState::Unset => render! {
            "Hover over a story to preview it here"
        },
        PreviewState::Loading => render! {
            "Loading..."
        },
        PreviewState::Loaded(story) => {
            let title = &story.item.title;
            let url = story.item.url.as_deref().unwrap_or_default();
            let text = story.item.text.as_deref().unwrap_or_default();
            render! {
                div {
                    padding: "0.5rem",
                    div {
                        font_size: "1.5rem",
                        a {
                            href: "{url}",
                            "{title}"
                        }
                    }
                    div {
                        dangerous_inner_html: "{text}",
                    }
                    for comment in &story.comments {
                        Comment { comment: comment.clone() }
                    }
                }
            }
        }
    }
}

#[component(no_case_check)]
fn Comment(cx: Scope, comment: Comment) -> Element<'a> {
    render! {
        div {
            padding: "0.5rem",
            div {
                color: "gray",
                "by {comment.by}"
            }
            div {
                dangerous_inner_html: "{comment.text}"
            }
            for kid in &comment.sub_comments {
                Comment { comment: kid.clone() }
            }
        }
    }
}
