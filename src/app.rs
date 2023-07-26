use leptos::*;
use leptos_meta::*;

mod components;
use components::chat_area::ChatArea;
use components::type_area::TypeArea;

use crate::api::converse;
use crate::model::conversation::{Conversation, Message};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    let (conversation, set_conversation) = create_signal(cx, Conversation::new());

    let send = create_action(cx, move |new_message: &String| {
        let user_message = Message {
            text: new_message.clone(),
            user: true,
        };
        set_conversation.update(move |c| {
            c.messages.push(user_message);
        });
        converse(cx, conversation.get())
    });

    create_effect(cx, move |_| {
        if let Some(_) = send.input().get() {
            let model_message = Message {
                text: String::from("..."),
                user: false,
            };

            set_conversation.update(move |c| {
                c.messages.push(model_message);
            });
        }
    });

    create_effect(cx, move |_| {
        if let Some(Ok(response)) = send.value().get() {
            set_conversation.update(move |c| {
                c.messages.last_mut().unwrap().text = response;
            });
        }
    });

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Rusty Llama"/>
        <ChatArea conversation/>
        <TypeArea send/>
    }
}

///// Renders the home page of your application.
// #[component]
// fn HomePage(cx: Scope) -> impl IntoView {
//     // Creates a reactive value to update the button
//     let (count, set_count) = create_signal(cx, 0);
//     let on_click = move |_| set_count.update(|count| *count += 1);

//     view! { cx,
//         <h1>"Welcome to Rusty Chatbot MVP. I know leptos is slow or may be smooth or may be it wants us to have patience."</h1>
//         <button on:click=on_click>"Click Me: " {count}</button>
//     }
// }

///// 404 - Not Found
// #[component]
// fn NotFound(cx: Scope) -> impl IntoView {
//     // set an HTTP status code 404
//     // this is feature gated because it can only be done during
//     // initial server-side rendering
//     // if you navigate to the 404 page subsequently, the status
//     // code will not be set because there is not a new HTTP request
//     // to the server
//     #[cfg(feature = "ssr")]
//     {
//         // this can be done inline because it's synchronous
//         // if it were async, we'd use a server function
//         let resp = expect_context::<leptos_actix::ResponseOptions>(cx);
//         resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
//     }

//     view! { cx,
//         <h1>"Not Found"</h1>
//     }
// }
