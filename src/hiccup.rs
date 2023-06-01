use serde_json::json;
use serde_json::Value;
use std::collections::{HashMap, HashSet};

/// Add 'href' attributes to each 'a' tag that has a 'resource', but not an 'href'.
/// Return the updated list.
///
/// * param `element` - hiccup-style list to add 'href' attributes to
/// * param `href` - pattern for href where the substring "{curie}" is replaced with resource
/// * return - copy of element with added 'href'
pub fn insert_href(element: &Value, href: &str) -> Value {
    insert_href_by_depth(element, href, 0)
}

/// Add 'href' attributes to each 'a' tag that has a 'resource', but not an 'href'.
/// Return the updated list.
///
/// * param `element` - hiccup-style list to add 'href' attributes to
/// * param `href` - pattern for href where the substring "{curie}" is replaced with resource
/// * param `depth` - list depth of current element
/// * return - copy of element with added 'href'
pub fn insert_href_by_depth(element: &Value, href: &str, depth: usize) -> Value {
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
                        json!(href.replace("{curie}", attr["resource"].as_str().unwrap())),
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
                    output.push(insert_href_by_depth(&json!(x), href, depth + 1));
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

/// Add 'href' attributes to each 'a' tag that has a 'resource', but not an 'href'.
/// The 'href' attributes are specified with a map from CURIEs to the desired 'href' pattern.
/// Return the updated list.
///
/// * param `element` - hiccup-style list to add 'href' attributes to
/// * param `curie_2_href` - map from CURIEs to pattern for hrefs where the substring "{curie}" is replaced with resource
/// * return - copy of element with added 'href'
pub fn set_hrefs(element: &Value, curie_2_href : &HashMap<String,String>) -> Value {
    set_hrefs_by_depth(element, curie_2_href, 0)
}

/// Add 'href' attributes to each 'a' tag that has a 'resource', but not an 'href'.
/// The 'href' attributes are specified with a map from CURIEs to the desired 'href' pattern.
/// Return the updated list.
///
/// * param `element` - hiccup-style list to add 'href' attributes to
/// * param `curie_2_href` - map from CURIEs to pattern for hrefs where the substring "{curie}" is replaced with resource
/// * param `depth` - list depth of current element
/// * return - copy of element with added 'href'
pub fn set_hrefs_by_depth(element: &Value, curie_2_href : &HashMap<String,String>, depth: usize) -> Value {
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
                if tag_string.eq("a") & !attr.contains_key("href") & attr.contains_key("resource"){
                    if curie_2_href.contains_key(attr["resource"].as_str().unwrap()) {

                        let href = curie_2_href.get(attr["resource"].as_str().unwrap()).unwrap();
                        attr.insert(
                            String::from("href"),
                            json!(href.replace("{curie}", attr["resource"].as_str().unwrap())),
                        );
                    }
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
                    output.push(set_hrefs_by_depth(&json!(x), curie_2_href, depth + 1));
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

/// Add 'href' attributes to each 'a' tag that has a 'resource', but not an 'href' and is in
/// the list of target resources.
/// Return the updated list.
///
/// * param `element` - hiccup-style list to add 'href' attributes to
/// * param `targets` -  set of CURIEs href values are added
/// * return - copy of element with added 'href'
pub fn insert_href_for(element: &Value, href: &str, targets : &HashSet<String>) -> Value {
    insert_href_for_by_depth(element, href, targets, 0)
}

/// Add 'href' attributes to each 'a' tag that has a 'resource', but not an 'href' and is in
/// the list of target resources.
/// Return the updated list.
///
/// * param `element` - hiccup-style list to add 'href' attributes to
/// * param `targets` -  set of CURIEs href values are added
/// * param `depth` - list depth of current element
/// * return - copy of element with added 'href'
pub fn insert_href_for_by_depth(element: &Value, href: &str, targets : &HashSet<String>, depth: usize) -> Value {
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
                if tag_string.eq("a") & !attr.contains_key("href") & attr.contains_key("resource"){
                    if targets.contains(attr["resource"].as_str().unwrap()) {
                        attr.insert(
                            String::from("href"),
                            json!(href.replace("{curie}", attr["resource"].as_str().unwrap())),
                        );
                    }
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
                    output.push(insert_href_for_by_depth(&json!(x), href, targets, depth + 1));
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
/// * return - HTML string
pub fn render(element: &Value) -> String {
    render_by_depth(element, 0)
}

/// Render hiccup-style HTML vector as HTML.
/// * param `element` - hiccup-style list
/// * param `depth` - list depth of current element
/// * return - HTML string
pub fn render_by_depth(element: &Value, depth: usize) -> String {
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
                    output = format!("{}\n{}", output, render_by_depth(&child.clone(), depth + 1));
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
