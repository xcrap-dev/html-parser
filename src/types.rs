use napi_derive::napi;
use scraper::{ElementRef, Html};
use std::rc::Rc;

use crate::engines::{
    select_first_by_css, select_first_by_xpath, select_many_by_css, select_many_by_xpath,
};

#[napi]
pub enum QueryType {
    CSS,
    XPath,
}

#[napi(object)]
pub struct QueryConfig {
    pub query: String,
    #[napi(js_name = "type")]
    pub query_type: QueryType,
}

#[napi(js_name = "HTMLElement")]
pub struct HTMLElement {
    #[napi(js_name = "outerHTML")]
    pub outer_html: String,
    pub(crate) context: Option<NodeContext>,
}

/// Keeps a reference to the document an element came from, enabling tree navigation.
#[derive(Clone)]
pub(crate) struct NodeContext {
    document: Rc<Html>,
    id: ego_tree::NodeId,
}

impl HTMLElement {
    pub(crate) fn from_ref(document: &Rc<Html>, element: ElementRef) -> HTMLElement {
        return HTMLElement {
            outer_html: element.html(),
            context: Some(NodeContext {
                document: Rc::clone(document),
                id: element.id(),
            }),
        };
    }

    pub(crate) fn from_html(outer_html: String) -> HTMLElement {
        return HTMLElement {
            outer_html,
            context: None,
        };
    }

    fn node(&self) -> Option<ElementRef<'_>> {
        let context = self.context.as_ref()?;
        return context
            .document
            .tree
            .get(context.id)
            .and_then(ElementRef::wrap);
    }

    fn wrap_related(&self, element: ElementRef) -> HTMLElement {
        return match &self.context {
            Some(context) => HTMLElement::from_ref(&context.document, element),
            None => HTMLElement::from_html(element.html()),
        };
    }

    fn get_real_element<'a>(&self, fragment: &'a Html) -> Option<ElementRef<'a>> {
        let element = fragment.root_element();

        return element.children().filter_map(ElementRef::wrap).next();
    }
}

#[napi]
impl HTMLElement {
    #[napi(getter, js_name = "innerHTML")]
    pub fn inner_html(&self) -> String {
        let fragment = Html::parse_fragment(&self.outer_html);
        let element = self.get_real_element(&fragment).unwrap();
        return element.inner_html();
    }

    #[napi(getter)]
    pub fn text(&self) -> String {
        let fragment = Html::parse_fragment(&self.outer_html);
        let element = fragment.root_element();
        return element.text().collect::<Vec<_>>().join(" ");
    }

    #[napi(getter)]
    pub fn id(&self) -> Option<String> {
        let fragment = Html::parse_fragment(&self.outer_html);
        return self
            .get_real_element(&fragment)
            .and_then(|el| el.value().id())
            .map(|id_str| id_str.to_string());
    }

    #[napi(getter)]
    pub fn tag_name(&self) -> String {
        let fragment = Html::parse_fragment(&self.outer_html);

        return match self.get_real_element(&fragment) {
            Some(el) => el.value().name().to_uppercase(),
            None => "UNKNOWN".to_string(),
        };
    }

    #[napi]
    pub fn get_attribute(&self, name: String) -> Option<String> {
        let fragment = Html::parse_fragment(&self.outer_html);

        return self
            .get_real_element(&fragment)
            .and_then(|el| el.value().attr(&name))
            .map(|v| v.to_string());
    }

    #[napi(getter)]
    pub fn attributes(&self) -> std::collections::HashMap<String, String> {
        let fragment = Html::parse_fragment(&self.outer_html);
        let mut attr_map = std::collections::HashMap::new();

        if let Some(el) = self.get_real_element(&fragment) {
            for (key, value) in el.value().attrs() {
                attr_map.insert(key.to_string(), value.to_string());
            }
        }

        return attr_map;
    }

    #[napi(getter)]
    pub fn class_name(&self) -> String {
        let fragment = Html::parse_fragment(&self.outer_html);

        return self
            .get_real_element(&fragment)
            .and_then(|el| el.value().attr("class"))
            .unwrap_or("")
            .to_string();
    }

    #[napi(getter)]
    pub fn class_list(&self) -> Vec<String> {
        let fragment = Html::parse_fragment(&self.outer_html);

        return match self
            .get_real_element(&fragment)
            .and_then(|el| el.value().attr("class"))
        {
            Some(classes) => classes.split_whitespace().map(|s| s.to_string()).collect(),
            None => vec![],
        };
    }

    #[napi]
    pub fn select_first(&self, options: SelectFirstOptions) -> Option<HTMLElement> {
        let query_config = options.query;

        match query_config.query_type {
            QueryType::CSS => {
                if let Some(node) = self.node() {
                    let selector = scraper::Selector::parse(&query_config.query).ok()?;
                    return node
                        .select(&selector)
                        .next()
                        .map(|el| self.wrap_related(el));
                }

                let document = Rc::new(Html::parse_fragment(&self.outer_html));
                return select_first_by_css(&document, query_config.query);
            }
            QueryType::XPath => {
                let package = sxd_html::parse_html(&self.outer_html);
                return select_first_by_xpath(&package, query_config.query);
            }
        }
    }

    #[napi]
    pub fn select_many(&self, options: SelectManyOptions) -> Vec<HTMLElement> {
        let query_config = options.query;

        match query_config.query_type {
            QueryType::CSS => {
                if let Some(node) = self.node() {
                    let selector = match scraper::Selector::parse(&query_config.query) {
                        Ok(sel) => sel,
                        Err(_) => return vec![],
                    };

                    let limit = match options.limit {
                        Some(l) if l > 0 => l as usize,
                        _ => usize::MAX,
                    };

                    return node
                        .select(&selector)
                        .take(limit)
                        .map(|el| self.wrap_related(el))
                        .collect();
                }

                let document = Rc::new(Html::parse_fragment(&self.outer_html));
                return select_many_by_css(&document, query_config.query, options.limit);
            }
            QueryType::XPath => {
                let package = sxd_html::parse_html(&self.outer_html);
                return select_many_by_xpath(&package, query_config.query, options.limit);
            }
        }
    }

    #[napi(getter)]
    pub fn first_child(&self) -> Option<HTMLElement> {
        if let Some(node) = self.node() {
            return node
                .children()
                .find_map(ElementRef::wrap)
                .map(|el| self.wrap_related(el));
        }

        let fragment = Html::parse_fragment(&self.outer_html);
        let real_element = self.get_real_element(&fragment)?;
        let child = real_element.children().find_map(ElementRef::wrap)?;

        return Some(HTMLElement::from_html(child.html()));
    }

    #[napi(getter)]
    pub fn last_child(&self) -> Option<HTMLElement> {
        if let Some(node) = self.node() {
            return node
                .children()
                .rev()
                .find_map(ElementRef::wrap)
                .map(|el| self.wrap_related(el));
        }

        let fragment = Html::parse_fragment(&self.outer_html);
        let real_element = self.get_real_element(&fragment)?;
        let child = real_element.children().rev().find_map(ElementRef::wrap)?;

        return Some(HTMLElement::from_html(child.html()));
    }

    #[napi(getter)]
    pub fn children(&self) -> Vec<HTMLElement> {
        if let Some(node) = self.node() {
            return node
                .children()
                .filter_map(ElementRef::wrap)
                .map(|el| self.wrap_related(el))
                .collect();
        }

        let fragment = Html::parse_fragment(&self.outer_html);

        return match self.get_real_element(&fragment) {
            Some(el) => el
                .children()
                .filter_map(ElementRef::wrap)
                .map(|child| HTMLElement::from_html(child.html()))
                .collect(),
            None => vec![],
        };
    }

    /// Parent element, or `null` for the root or elements without document context (e.g. XPath results).
    #[napi(getter)]
    pub fn parent(&self) -> Option<HTMLElement> {
        let parent = self.node()?.parent().and_then(ElementRef::wrap)?;
        return Some(self.wrap_related(parent));
    }

    /// All ancestors, from the closest parent up to the root element.
    #[napi(getter)]
    pub fn ancestors(&self) -> Vec<HTMLElement> {
        return match self.node() {
            Some(node) => node
                .ancestors()
                .filter_map(ElementRef::wrap)
                .map(|el| self.wrap_related(el))
                .collect(),
            None => vec![],
        };
    }

    #[napi(getter)]
    pub fn next_sibling(&self) -> Option<HTMLElement> {
        let sibling = self.node()?.next_siblings().find_map(ElementRef::wrap)?;
        return Some(self.wrap_related(sibling));
    }

    #[napi(getter)]
    pub fn previous_sibling(&self) -> Option<HTMLElement> {
        let sibling = self.node()?.prev_siblings().find_map(ElementRef::wrap)?;
        return Some(self.wrap_related(sibling));
    }

    /// Sibling elements (excluding this element), in document order.
    #[napi(getter)]
    pub fn siblings(&self) -> Vec<HTMLElement> {
        let node = match self.node() {
            Some(node) => node,
            None => return vec![],
        };

        let parent = match node.parent() {
            Some(parent) => parent,
            None => return vec![],
        };

        return parent
            .children()
            .filter(|child| child.id() != node.id())
            .filter_map(ElementRef::wrap)
            .map(|el| self.wrap_related(el))
            .collect();
    }

    /// Closest ancestor (or the element itself) matching the CSS selector.
    #[napi]
    pub fn closest(&self, selector: String) -> Option<HTMLElement> {
        let node = self.node()?;
        let selector = scraper::Selector::parse(&selector).ok()?;

        let found = std::iter::once(node)
            .chain(node.ancestors().filter_map(ElementRef::wrap))
            .find(|el| selector.matches(el))?;

        return Some(self.wrap_related(found));
    }

    #[napi]
    pub fn to_string(&self) -> &String {
        return &self.outer_html;
    }
}

#[napi(object)]
pub struct SelectFirstOptions {
    pub query: QueryConfig,
}

#[napi(object)]
pub struct SelectManyOptions {
    pub query: QueryConfig,
    pub limit: Option<i32>,
}
