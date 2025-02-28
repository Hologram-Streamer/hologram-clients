use yew::prelude::*;

#[function_component]
fn App() -> Html{
    html! {
        <div>
            <h1>{"Welcome to Hologram!"}</h1>
        </div>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
