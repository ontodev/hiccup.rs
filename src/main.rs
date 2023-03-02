use serde_json::Value;

fn render(element: Value, depth: usize) -> String {
    let render_element = element.clone();
    let indent = "  ".repeat(depth);
    let mut element_pointer = 0;

    let render_element = match render_element {
        Value::Array(x) => x,
        _ => panic!("Element is not a list: {:?}", element),
    };
    if render_element.is_empty() {
        panic!("Element is an empty list")
    }

    let tag = render_element[0].clone();
    element_pointer += 1;
    let tag_string = match tag {
        Value::String(x) => x,
        _ => panic!(
            "Tag '{tag}' at loc {depth} is not a string",
            tag = tag,
            depth = depth
        ),
    };

    let mut output = format!("{}<{}", indent, tag_string);

    if render_element.len() - element_pointer > 0 {
        match render_element[element_pointer].clone() {
            Value::Object(attr) => {
                element_pointer += 1;
                for (key, value) in attr {
                    if key.eq("checked") {
                        output = format!("{} {}", output, key);
                    } else {
                        output = format!("{} {}=\"{}\"", output, key, value.as_str().unwrap());
                    }
                }
            }
            _ => {}
        }
    }

    if tag_string.eq("meta") | tag_string.eq("link") | tag_string.eq("path") {
        output = format!("{}/>", output);
        return output;
    }

    output = format!("{}>", output);
    let mut spacing = String::from("");
    if render_element.len() - element_pointer > 0 {
        for child in &render_element[element_pointer..] {
            match child {
                Value::String(s) => {
                    output = format!("{}{}", output, s.as_str());
                }
                Value::Array(_v) => {
                    output = format!("{}\n{}", output, render(child.clone(), depth + 1));
                    spacing = format!("\n{}", indent);
                }
                _ => panic!(
                    "Bad type for '{tag}' child '{child}' at loc {depth}",
                    tag = tag_string,
                    child = child,
                    depth = depth + 1
                ),
            }
        }
    }
    output = format!("{}{}</{}>", output, spacing, tag_string);
    output
}

fn main() {
    let data = r#"["body", ["div", {"id": "myDiv"}, ["h1", {"class": "header"}, "Hello World!"]]]"#;

    let hiccup: Value = serde_json::from_str(data).unwrap();

    let html = render(hiccup, 0);

    println!("{}", html);
}
