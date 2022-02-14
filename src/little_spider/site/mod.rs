pub mod dlsite;
pub mod javbus;
pub mod wikipedia;

use scraper::{ElementRef, Selector};

fn join_all_a_elements_inner_html(element: &ElementRef) -> String {
    if let Ok(a_selector) = Selector::parse("a") {
        return join_elements_inner_html(element.select(&a_selector).collect::<Vec<_>>());
    }
    return String::new();
}

fn join_elements_inner_html(elements: Vec<ElementRef>) -> String {
    return get_elements_inner_html(elements).join(",");
}

fn get_elements_attr(elements: Vec<ElementRef>, attr: &str) -> Vec<String> {
    return elements
        .iter()
        .map(|t| {
            if let Some(r) = t.value().attr(attr) {
                String::from(r)
            } else {
                String::new()
            }
        })
        .collect::<Vec<_>>();
}

fn get_elements_inner_html(elements: Vec<ElementRef>) -> Vec<String> {
    return elements.iter().map(|t| t.inner_html()).collect::<Vec<_>>();
}
