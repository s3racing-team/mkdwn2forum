fn check(input: &str, expected: &str) {
    let output = crate::convert(input);
    assert_eq!(expected, output);
}

#[test]
fn titles() {
    check("# Title1\n", "[size=200]Title1[/size]\n");
    check("## Title2\n", "[size=150]Title2[/size]\n");
    check("### Title3\n", "[size]Title3[/size]\n");
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
