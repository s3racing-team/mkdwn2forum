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

    let markdown_placeholder = "\
# Heading 1
## Heading 2
### Heading 3
#### Heading 4

**This will be bold**
*This is italic*
_And this will be underlined_

[Here goes the link text](https://here-goes-the-url.com)
<https://this-is-a-plain-url.com>

```py
# were inside a code block
for i in range(0, 25):
    print(i**2)
```

- here
- is
- a
- list

1. And
2. this
3. is
4. a
5. numbered
6. list

- Hey
    - look
    - we
        - can
        - even
    -indent
- items
";
    let forum_placeholder = mkdwn2forum::convert(markdown_placeholder);

    html! {
        <div class="container">
            <textarea class="input" oninput={on_text_input} placeholder={markdown_placeholder} />
            <div class="output-container">
                <textarea class="output" value={output.to_string()} readonly=true placeholder={forum_placeholder} />
                <button class="copy_button" onclick={copy_text}>
                    <svg class="copy_icon" xmlns="http://www.w3.org/2000/svg" height="1.5em" viewBox="0 0 512 512">
                        <path d="M272 0H396.1c12.7 0 24.9 5.1 33.9 14.1l67.9 67.9c9 9 14.1 21.2 14.1 33.9V336c0 26.5-21.5 48-48 48H272c-26.5 0-48-21.5-48-48V48c0-26.5 21.5-48 48-48zM48 128H192v64H64V448H256V416h64v48c0 26.5-21.5 48-48 48H48c-26.5 0-48-21.5-48-48V176c0-26.5 21.5-48 48-48z"/>
                    </svg>
                </button>
            </div>
        </div>
    }
}
