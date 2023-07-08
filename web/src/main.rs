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

    let copy_text = {
        let output = output.clone();

        Callback::from(move |_: MouseEvent| {
            let window = web_sys::window().unwrap();
            let navigator = window.navigator();
            let clipboard = navigator.clipboard().unwrap();
            let _ = clipboard.write_text(output.as_str());
        })
    };

    html! {
        <div class="container">
            <textarea class="left" oninput={on_text_input} placeholder="# This is a heading\n## This is a subheading\n[Here goes the link text](https://here-goes-the-url.com)\n<https://this-is-a-plain-url.com>\n\n- here\n- is\n- a\n- list\n\n1. And\n2. this\n3. is\n4. a\n5. numbered\n6. list\n" />
            <textarea class="right" value={output.to_string()} readonly=true />
            <button class="copy_button" onclick={copy_text}>
                <svg class="copy_icon" xmlns="http://www.w3.org/2000/svg" height="1.5em" viewBox="0 0 512 512">
                    <path d="M272 0H396.1c12.7 0 24.9 5.1 33.9 14.1l67.9 67.9c9 9 14.1 21.2 14.1 33.9V336c0 26.5-21.5 48-48 48H272c-26.5 0-48-21.5-48-48V48c0-26.5 21.5-48 48-48zM48 128H192v64H64V448H256V416h64v48c0 26.5-21.5 48-48 48H48c-26.5 0-48-21.5-48-48V176c0-26.5 21.5-48 48-48z"/>
                </svg>
            </button>
        </div>
    }
}
