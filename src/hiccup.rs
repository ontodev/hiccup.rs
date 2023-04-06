use serde_json::json;
use serde_json::Value;

/// Add 'href' attributes to each 'a' tag that has a 'resource', but not an 'href'.
/// Return the updated list.
///
/// * param `element` - hiccup-style list to add 'href' attributes to
/// * param `resource` - target 'resource' for which href attributes are added
/// * param `pattern` - pattern for href where the substring "{curie}" is replaced with the target resource
/// * param `depth` - list depth of current element
/// * return - copy of element with added 'href'
pub fn insert_href(element: &Value, resource: &str, pattern: &str, depth: usize) -> Value {
    let mut element_pointer = 0;
    let render_element = element.clone();
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

    let mut output = vec![json!(tag_string.clone())];

    if render_element.len() - element_pointer > 0 {
        match render_element[element_pointer].clone() {
            Value::Object(mut attr) => {
                element_pointer += 1;
                if tag_string.eq("a") & !attr.contains_key("href") & attr.contains_key("resource") {
                    attr.insert(
                        String::from("href"),
                        json!(pattern.replace("{curie}", attr["resource"].as_str().unwrap())),
                    );
                }
                output.push(Value::Object(attr.clone()));
            }
            _ => {}
        }
    }

    if render_element.len() - element_pointer > 0 {
        for i in element_pointer..render_element.len() {
            let child = render_element[i].clone();
            match child {
                Value::String(x) => {
                    output.push(json!(x));
                }
                Value::Array(x) => {
                    output.push(insert_href(&json!(x), resource, pattern, depth + 1));
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

    Value::Array(output)
}

/// Render hiccup-style HTML vector as HTML.
/// * param `element` - hiccup-style list
/// * param `depth` - list depth of current element
/// * return - HTML string
pub fn render(element: &Value, depth: usize) -> String {
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
                    output = format!("{}\n{}", output, render(&child.clone(), depth + 1));
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
