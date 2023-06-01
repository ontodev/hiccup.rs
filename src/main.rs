use serde_json::Value;

pub mod hiccup;

fn main() {
    //let data = r#"["body", ["div", {"id": "myDiv"}, ["h1", {"class": "header"}, "Hello World!"]]]"#;
    let data = r#"["body", ["div", {"id": "myDiv"}, ["h1", {"class": "header"}, ["a", {"resource":"iri:example"}, "Hello World!"]]]]"#;

    let hiccup: Value = serde_json::from_str(data).unwrap();

    let html = hiccup::render(&hiccup).unwrap();
    let html_2 = hiccup::insert_href(&hiccup, "?id={curie}").unwrap();

    println!("{}", html);
    println!("{}", html_2);
}
