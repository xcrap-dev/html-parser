import test from "ava"

import { HTMLParser, HTMLElement, css } from "../index"
import { HTML } from "./fixtures"

function select(query: string): HTMLElement {
    return new HTMLParser(HTML).selectFirst({ query: css(query) }) as HTMLElement
}

test("parent — returns the parent element", (t) => {
    t.is(select("#nested-child-1").parent?.id, "nested-parent")
})

test("parent — returns null at the root", (t) => {
    t.is(select("html").parent, null)
})

test("parent — works on elements found via selectMany", (t) => {
    const tags = new HTMLParser(HTML).selectMany({ query: css(".tag") })
    t.is(tags[2].parent?.id, "tag-list")
})

test("parent — works on elements found through another element", (t) => {
    const child = select("#nested").selectFirst({ query: css("p") }) as HTMLElement
    t.is(child.parent?.id, "nested-parent")
})

test("children — returns direct child elements", (t) => {
    const children = select("#nested-parent").children
    t.is(children.length, 2)
    t.is(children[1].id, "nested-child-2")
})

test("firstChild/lastChild — keep navigation context", (t) => {
    const list = select("#tag-list")
    t.is(list.firstChild?.parent?.id, "tag-list")
    t.is(list.lastChild?.text.trim(), "wasm")
})

test("nextSibling / previousSibling", (t) => {
    const child = select("#nested-child-1")
    t.is(child.nextSibling?.id, "nested-child-2")
    t.is(child.previousSibling, null)
    t.is(child.nextSibling?.previousSibling?.id, "nested-child-1")
})

test("siblings — excludes the element itself", (t) => {
    const siblings = select("#post-2").siblings
    t.deepEqual(siblings.map((s) => s.id), ["post-1", "post-3"])
})

test("ancestors — closest first", (t) => {
    const tags = select("#nested-child-1").ancestors.map((a) => a.tagName)
    t.deepEqual(tags.slice(0, 2), ["DIV", "SECTION"])
})

test("closest — matches ancestor, self, or null", (t) => {
    const title = select("#post-1 .post-title")
    t.is(title.closest("article")?.id, "post-1")
    t.is(title.closest("h2")?.tagName, "H2")
    t.is(title.closest("table"), null)
})
