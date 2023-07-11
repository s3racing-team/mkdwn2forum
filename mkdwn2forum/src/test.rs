fn check(input: &str, expected: &str) {
    let output = crate::convert(input);
    assert_eq!(expected, output);
}

#[test]
fn titles() {
    check("# Title1\n", "[size=200]Title1[/size]\n");
    check("## Title2\n", "[size=150]Title2[/size]\n");
    check("### Title3\n", "[size=120]Title3[/size]\n");
    check("#### Title4\n", "[size=110]Title4[/size]\n");
    check("##### Title5\n", "[size]Title5[/size]\n");
}

#[test]
fn simple_list() {
    check(
        "
- a
- b
- c
",
        "
[list]
[*]a
[*]b
[*]c
[/list]
",
    );
}

#[test]
fn nested_list() {
    check(
        "
- a
    - a1
    - a2
- b
- c
",
        "
[list]
[*]a
    [list]
    [*]a1
    [*]a2
    [/list]
[*]b
[*]c
[/list]
",
    );
}

#[test]
fn numbered_list() {
    check(
        "
1. a
2. b
3. c
",
        "
[list=1]
[*]a
[*]b
[*]c
[/list]
",
    );
}

#[test]
fn nested_numbered_list() {
    check(
        "
- a
    2. a1
    3. a2
- b
- c
",
        "
[list]
[*]a
    [list=1]
    [*]a1
    [*]a2
    [/list]
[*]b
[*]c
[/list]
",
    );
}

#[test]
fn code_block() {
    check(
        "\
```
fn main() {
    println!(\"hello world]\");
}
```
",
        "\
[code]
fn main() {
    println!(\"hello world]\");
}
[/code]
",
    )
}

#[test]
fn unclosed_code_block() {
    check(
        "\
```
fn main() {
    println!(\"hello world]\");
}
",
        "\
[code]
fn main() {
    println!(\"hello world]\");
}
[/code]
",
    )
}

#[test]
fn url_with_alt_text() {
    check(
        "Click this [ link ](https://pointerpointer.com) now\n",
        "Click this [url=https://pointerpointer.com]link[/url] now\n",
    );
}

#[test]
fn plain_url() {
    check(
        "Look a url: <https://pointerpointer.com> hmmmm\n",
        "Look a url: [url]https://pointerpointer.com[/url] hmmmm\n",
    );
}

#[test]
fn bold() {
    check("Some **bold** text\n", "Some [b]bold[/b] text\n");
}

#[test]
fn bold_not_properly_closed() {
    check(
        "Some **not properly closed bold* text\n",
        "Some [b]not properly closed bold[/b] text\n",
    );
}

#[test]
fn italic() {
    check(
        "What are they *doin* ova der\n",
        "What are they [i]doin[/i] ova der\n",
    );
}

#[test]
fn underlined() {
    check(
        "Some __underlined text__, hmmm\n",
        "Some [u]underlined text[/u], hmmm\n",
    );
}

#[test]
fn underlined_not_properly_closed() {
    check(
        "Some __underlined text_, hmmm\n",
        "Some [u]underlined text[/u], hmmm\n",
    );
}
