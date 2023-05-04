use crate::pages::Home::*;
use crate::pages::NominationList::*;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Kitsu Anime Awards Form"/>

        // content for this welcome page
        <Router>
            <main>
                <div id="content">
                <Routes>
                    <Route path="" view=|cx| view! { cx, <Home />}/>
                    <Route path="/nominations" view=|cx| view! { cx, <NominationList category="Best Anime".to_string() /> }/>
                </Routes>
                </div>
            </main>
        </Router>
    }
}
