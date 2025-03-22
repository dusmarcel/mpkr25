use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <h1>{"Migrationsrechtlicher Prozesskostenrechner (Stand 2025)"}</h1>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
