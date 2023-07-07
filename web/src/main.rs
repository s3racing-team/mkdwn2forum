use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    let output = use_state(|| String::new());

    let on_text_input = {
        let output = output.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            let text = mkdwn2forum::convert(&input.value());
            output.set(text);
        })
    };

    html! {
        <div class="container">
            <textarea class="left" oninput={on_text_input} />
            <textarea value={output.to_string()} class="right" readonly=true />
        </div>
    }
}
