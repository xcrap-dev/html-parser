use crate::types::HTMLElement;
use scraper::Html;
use std::rc::Rc;
use sxd_document::Package;
use sxd_xpath::evaluate_xpath;

pub fn select_first_by_css(document: &Rc<Html>, query: String) -> Option<HTMLElement> {
    let selector = scraper::Selector::parse(&query).ok()?;

    return document
        .select(&selector)
        .next()
        .map(|element| HTMLElement::from_ref(document, element));
}

pub fn select_first_by_xpath(document_xpath: &Package, query: String) -> Option<HTMLElement> {
    let document = document_xpath.as_document();
    let result = evaluate_xpath(&document, &query).ok()?;

    if let sxd_xpath::Value::Nodeset(nodeset) = result {
        return nodeset.document_order_first().map(|node| HTMLElement::from_html(node.string_value()));
    }

    return None;
}

pub fn select_many_by_css(document: &Rc<Html>, query: String, limit: Option<i32>) -> Vec<HTMLElement> {
    let selector = match scraper::Selector::parse(&query) {
        Ok(sel) => sel,
        Err(_) => return vec![],
    };

    let selection = document.select(&selector);

    let iter = selection.map(|element| HTMLElement::from_ref(document, element));

    if let Some(l) = limit {
        if l > 0 {
            return iter.take(l as usize).collect();
        }
    }

    return iter.collect();
}

pub fn select_many_by_xpath(
    package: &Package,
    query: String,
    limit: Option<i32>,
) -> Vec<HTMLElement> {
    let document = package.as_document();

    let result = match evaluate_xpath(&document, &query) {
        Ok(res) => res,
        Err(_) => return vec![],
    };

    if let sxd_xpath::Value::Nodeset(nodeset) = result {
        let nodes = nodeset.document_order();

        let iter = nodes.iter().map(|node| HTMLElement::from_html(node.string_value()));

        if let Some(l) = limit {
            if l > 0 {
                return iter.take(l as usize).collect();
            }
        }

        return iter.collect();
    }

    return vec![];
}
