use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <h1 class={classes!("container", "max-w-screen-xl", "mx-auto", "px-4", "bg-blue-100")}>
                {"Migrationsrechtlicher Prozesskostenrechner (Stand 2025)"}
            </h1>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
