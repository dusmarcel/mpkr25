use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div class={classes!("container", "max-w-screen-xl", "mx-auto", "px-4", "bg-linear-to-b", "from-stone-50", "to-stone-300")}>
            <h1 class={classes!("pt-4", "text-3xl", "font-medium")}>
                {"Migrationsrechtlicher Prozesskostenrechner (Stand 2025)"}
            </h1>
            <div>
                {"Erstellt von "}
                <a href="https://aufentha.lt" class={classes!("text-blue-600", "hover:underline", "hover:text-violet-600")}>{"Marcel Keienborg"}</a>
                {". Bitte beachte unbedingt auch die Hinweise unten auf dieser Seite."}
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
