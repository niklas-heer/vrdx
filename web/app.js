"use strict";

// All record content enters the document as text; Markdown never becomes raw HTML.
const $ = (id) => document.getElementById(id);
const statuses = ["accepted", "proposed", "rejected", "deprecated", "superseded"];
const state = { graph: { decisions: {}, edges: [], findings: [] }, valid: true, status: "", tags: new Set(), query: "", view: "cards", selected: null, loaded: false };
let snapshot = "";
let loading = false;
let suggestionRequest = 0;

function element(tag, className, text) {
  const node = document.createElement(tag);
  if (className) node.className = className;
  if (text !== undefined) node.textContent = text;
  return node;
}
function button(className, text, action) {
  const node = element("button", className, text);
  node.type = "button";
  node.addEventListener("click", action);
  return node;
}
function badge(status) {
  const node = element("span", "badge");
  node.append(element("span", `status-dot ${statuses.includes(status) ? status : ""}`), document.createTextNode(status));
  return node;
}
function dateLabel(date) {
  const value = new Date(`${date}T12:00:00Z`);
  return Number.isNaN(value.getTime()) ? date : new Intl.DateTimeFormat("en", { day: "numeric", month: "short", year: "numeric", timeZone: "UTC" }).format(value);
}
function decisions() {
  return Object.values(state.graph.decisions).sort((a, b) => b.date.localeCompare(a.date) || b.id.localeCompare(a.id));
}
function filtered() {
  const words = state.query.toLowerCase().trim().split(/\s+/).filter(Boolean);
  return decisions().filter((record) => {
    if (state.status && record.status !== state.status) return false;
    if (![...state.tags].every((tag) => record.tags.some((candidate) => candidate.toLowerCase() === tag))) return false;
    const text = [record.title, record.body, record.id, record.status, ...record.tags].join(" ").toLowerCase();
    return words.every((word) => text.includes(word));
  });
}
function excerpt(body) {
  return body.replace(/^#{1,6}\s+.*$/gm, "").replace(/```[\s\S]*?```/g, "").replace(/[*_`>#]/g, "").replace(/\[([^\]]+)\]\([^)]*\)/g, "$1").replace(/\s+/g, " ").trim();
}
function resetFilters() {
  state.status = "";
  state.tags.clear();
  state.query = "";
  $("search").value = "";
  renderFilters();
  renderCollection();
}
function renderFilters() {
  const all = decisions();
  const statusContainer = $("status-filters");
  statusContainer.replaceChildren();
  for (const status of ["", ...statuses]) {
    const node = button("filter-button", "", () => {
      state.status = status;
      renderFilters();
      renderCollection();
    });
    node.setAttribute("aria-pressed", String(state.status === status));
    node.append(element("span", `status-dot ${status}`), element("span", "", status ? status[0].toUpperCase() + status.slice(1) : "All decisions"), element("span", "filter-count", String(all.filter((d) => !status || d.status === status).length)));
    statusContainer.append(node);
  }
  const tags = new Map();
  for (const record of all) for (const tag of record.tags) {
    const key = tag.toLowerCase();
    const previous = tags.get(key);
    tags.set(key, { label: previous?.label || tag, count: (previous?.count || 0) + 1 });
  }
  $("tag-filters").replaceChildren();
  for (const [tag, { label, count }] of [...tags].sort(([a], [b]) => a.localeCompare(b))) {
    const node = button("tag-button", label, () => {
      if (state.tags.has(tag)) state.tags.delete(tag); else state.tags.add(tag);
      renderFilters();
      renderCollection();
    });
    node.append(element("span", "", ` ${count}`));
    node.setAttribute("aria-pressed", String(state.tags.has(tag)));
    $("tag-filters").append(node);
  }
  if (!tags.size) $("tag-filters").append(element("span", "muted", "Topics appear when you add tags."));
  $("clear-tags").hidden = !state.tags.size;
  const stats = [
    [all.length, "Decisions", "A record of your reasoning"],
    [all.filter((record) => record.status === "accepted").length, "Accepted", state.valid ? "Marked as accepted" : "Collection needs review"],
    [state.graph.edges.length, "Connections", `${tags.size} topics to explore`],
  ];
  $("stats").replaceChildren(...stats.map(([count, label, note]) => {
    const node = element("div", "stat");
    const text = element("span", "stat-label", label);
    text.append(element("small", "", note));
    node.append(element("span", "stat-number", String(count).padStart(2, "0")), text);
    return node;
  }));
  $("status-legend").replaceChildren(...statuses.map(badge));
}
function renderCollection() {
  const records = filtered();
  const total = decisions().length;
  $("collection-title").textContent = state.status ? `${state.status[0].toUpperCase()}${state.status.slice(1)} decisions` : "All decisions";
  $("result-count").textContent = `${records.length} of ${total} records · Newest first`;
  $("cards-view").setAttribute("aria-pressed", String(state.view === "cards"));
  $("graph-view").setAttribute("aria-pressed", String(state.view === "graph"));
  $("active-filters").replaceChildren(...[...state.tags].map((tag) => button("", `${tag} ×`, () => {
    state.tags.delete(tag);
    renderFilters();
    renderCollection();
  })));
  $("records").hidden = state.view !== "cards" || !records.length;
  $("graph-panel").hidden = state.view !== "graph" || !records.length;
  $("empty").hidden = !!records.length;
  if (!records.length) {
    $("empty").replaceChildren(element("h3", "", total ? "A little too specific?" : "The next decision starts here."), element("p", "", total ? "No decisions match these filters. Try another topic or a broader search." : "Your collection is ready for its first record. Create a Markdown decision with the CLI, then watch it appear here."));
    if (total) $("empty").append(button("", "Clear filters", resetFilters));
    else $("empty").append(element("code", "", "vrdx new --help"));
    return;
  }
  if (state.view === "graph") { renderGraph(records); return; }
  $("records").replaceChildren(...records.map((record) => {
    const node = button("card", "", () => openRecord(record.id));
    node.setAttribute("aria-label", `${record.title}, ${record.status}. Read decision.`);
    const top = element("div", "card-top");
    top.append(badge(record.status), element("span", "card-date", dateLabel(record.date)));
    const tags = element("div", "card-tags");
    tags.append(...record.tags.map((tag) => element("span", "tag", tag)));
    const links = state.graph.edges.filter((edge) => edge.from === record.id || edge.to === record.id).length;
    const bottom = element("div", "card-bottom");
    bottom.append(element("span", "", `${links} ${links === 1 ? "connection" : "connections"}`), element("span", "", "↗"));
    node.append(top, element("h3", "", record.title), element("p", "card-excerpt", excerpt(record.body) || "Open this record to explore its metadata and connections."), tags, bottom);
    return node;
  }));
}
function renderGraph(records) {
  // A deterministic chronological grid keeps layout predictable, even for cycles.
  const columns = Math.max(1, Math.min(3, Math.floor(($("graph-scroll").clientWidth || 750) / 265)));
  const width = Math.max(columns * 270 + 40, 310);
  const height = Math.max(Math.ceil(records.length / columns) * 165 + 60, 350);
  const graph = $("graph");
  graph.replaceChildren();
  graph.style.width = `${width}px`;
  graph.style.height = `${height}px`;
  const positions = new Map(records.map((record, index) => [record.id, { x: 35 + index % columns * 270, y: 30 + Math.floor(index / columns) * 165 }]));
  const ns = "http://www.w3.org/2000/svg";
  const svg = document.createElementNS(ns, "svg");
  svg.setAttribute("width", String(width));
  svg.setAttribute("height", String(height));
  svg.setAttribute("aria-hidden", "true");
  const defs = document.createElementNS(ns, "defs");
  const colors = { supersedes: "#55806b", depends_on: "#8393b0", related_to: "#b8a577" };
  for (const [kind, color] of Object.entries(colors)) {
    const marker = document.createElementNS(ns, "marker");
    for (const [key, value] of Object.entries({ id: `arrow-${kind}`, viewBox: "0 0 10 10", refX: "9", refY: "5", markerWidth: "6", markerHeight: "6", orient: "auto-start-reverse" })) marker.setAttribute(key, value);
    const arrow = document.createElementNS(ns, "path");
    arrow.setAttribute("d", "M 0 0 L 10 5 L 0 10 z");
    arrow.setAttribute("fill", color);
    marker.append(arrow);
    defs.append(marker);
  }
  svg.append(defs);
  for (const edge of state.graph.edges) {
    const from = positions.get(edge.from), to = positions.get(edge.to);
    if (!from || !to || !colors[edge.relation]) continue;
    const path = document.createElementNS(ns, "path");
    const downward = to.y > from.y;
    const sameRow = from.y === to.y;
    const sx = from.x + 105, sy = from.y + (downward || sameRow ? 105 : 0);
    const tx = to.x + 105, ty = to.y + (downward ? 0 : 105);
    const bend = sameRow ? sy + 30 : (sy + ty) / 2;
    path.setAttribute("d", `M ${sx} ${sy} C ${sx} ${bend}, ${tx} ${bend}, ${tx} ${ty}`);
    path.setAttribute("fill", "none");
    path.setAttribute("stroke", colors[edge.relation]);
    path.setAttribute("stroke-width", "1.7");
    if (edge.relation !== "related_to") path.setAttribute("marker-end", `url(#arrow-${edge.relation})`);
    if (edge.relation !== "supersedes") path.setAttribute("stroke-dasharray", edge.relation === "depends_on" ? "6 4" : "2 4");
    svg.append(path);
  }
  graph.append(svg);
  for (const record of records) {
    const position = positions.get(record.id);
    const node = button("graph-node", "", () => openRecord(record.id));
    node.style.left = `${position.x}px`;
    node.style.top = `${position.y}px`;
    node.setAttribute("aria-label", `${record.title}, ${record.status}. Open relationships and reasoning.`);
    node.append(badge(record.status), element("strong", "", record.title));
    graph.append(node);
  }
}
function appendInline(node, text) {
  const pattern = /(`[^`]+`|\*\*[^*]+\*\*|\[([^\]]+)\]\(([^\s)]+)\))/g;
  let offset = 0;
  for (const match of text.matchAll(pattern)) {
    node.append(document.createTextNode(text.slice(offset, match.index)));
    if (match[0].startsWith("`")) node.append(element("code", "", match[0].slice(1, -1)));
    else if (match[0].startsWith("**")) node.append(element("strong", "", match[0].slice(2, -2)));
    else {
      let url;
      try { url = new URL(match[3]); } catch { /* Local and malformed links remain text. */ }
      if (url && ["https:", "http:"].includes(url.protocol)) {
        const link = element("a", "", match[2]);
        link.href = url.href;
        link.target = "_blank";
        link.rel = "noopener noreferrer";
        node.append(link);
      } else node.append(document.createTextNode(match[0]));
    }
    offset = match.index + match[0].length;
  }
  node.append(document.createTextNode(text.slice(offset)));
}
function markdown(body) {
  const container = element("div", "markdown");
  let paragraph = [], code = null, list = null;
  const flush = () => {
    if (paragraph.length) {
      const p = element("p");
      appendInline(p, paragraph.join(" "));
      container.append(p);
      paragraph = [];
    }
    list = null;
  };
  for (const line of body.split(/\r?\n/)) {
    if (/^\s*```/.test(line)) {
      flush();
      if (code === null) code = []; else {
        const pre = element("pre");
        pre.append(element("code", "", code.join("\n")));
        container.append(pre);
        code = null;
      }
      continue;
    }
    if (code !== null) { code.push(line); continue; }
    if (!line.trim()) { flush(); continue; }
    const heading = line.match(/^(#{1,6})\s+(.+)$/);
    const item = line.match(/^\s*(?:([-*+])|\d+\.)\s+(.+)$/);
    if (heading) {
      flush();
      const h = element(`h${Math.min(heading[1].length + 1, 6)}`);
      appendInline(h, heading[2]);
      container.append(h);
    } else if (item) {
      if (paragraph.length) flush();
      const type = item[1] ? "ul" : "ol";
      if (!list || list.tagName.toLowerCase() !== type) { list = element(type); container.append(list); }
      const li = element("li");
      appendInline(li, item[2]);
      list.append(li);
    } else if (/^>\s?/.test(line)) {
      flush();
      const quote = element("blockquote");
      appendInline(quote, line.replace(/^>\s?/, ""));
      container.append(quote);
    } else { list = null; paragraph.push(line); }
  }
  flush();
  if (code !== null) { const pre = element("pre"); pre.append(element("code", "", code.join("\n"))); container.append(pre); }
  return container;
}
function relationships(id) {
  const result = [];
  for (const edge of state.graph.edges) {
    if (edge.from === id) result.push({ kind: edge.relation.replaceAll("_", " "), id: edge.to });
    else if (edge.to === id) result.push({ kind: ({ supersedes: "superseded by", depends_on: "required by", related_to: "related to" })[edge.relation], id: edge.from });
  }
  return result;
}
function openRecord(id, updateHash = true) {
  const record = state.graph.decisions[id];
  if (!record) return;
  state.selected = id;
  if (updateHash && location.hash !== `#${id}`) history.pushState(null, "", `#${id}`);
  const content = $("detail-content");
  content.replaceChildren();
  const meta = element("div", "detail-meta");
  meta.append(badge(record.status), element("span", "", dateLabel(record.date)));
  const title = element("h2", "", record.title);
  title.id = "detail-title";
  const tags = element("div", "detail-tags");
  tags.append(...record.tags.map((tag) => element("span", "tag", tag)));
  const identity = element("div", "identity");
  const identityRow = element("div");
  const copy = button("", "Copy ID", async () => {
    try { await navigator.clipboard.writeText(record.id); copy.textContent = "Copied"; }
    catch { copy.textContent = "Select ID to copy"; }
  });
  identityRow.append(element("code", "", record.id), copy);
  identity.append(identityRow, element("p", "", record.file));
  content.append(meta, title, tags, identity);
  if (!state.valid) content.append(element("p", "detail-warning", "This collection has validation findings. Review them before relying on lifecycle or replacement relationships."));
  content.append(markdown(record.body));
  const relations = element("section", "detail-section");
  relations.append(element("h3", "", "Connected decisions"));
  const links = relationships(id);
  if (!links.length) relations.append(element("p", "", "No explicit relationships yet. Add decision IDs to the relationship fields in this record’s metadata."));
  for (const link of links) {
    const target = state.graph.decisions[link.id];
    const node = button("relation", "", () => openRecord(link.id));
    node.disabled = !target;
    node.append(element("span", "relation-kind", link.kind), element("span", "relation-title", target ? target.title : `Missing: ${link.id}`));
    relations.append(node);
  }
  content.append(relations);
  const suggestions = element("section", "detail-section");
  suggestions.id = "suggestions";
  suggestions.append(element("h3", "", "Worth a look"), element("p", "", "Looking for other relevant decisions…"));
  content.append(suggestions);
  void loadSuggestions(id);
  if (!$("detail").open) $("detail").showModal();
  $("detail").scrollTop = 0;
}
async function loadSuggestions(id) {
  const request = ++suggestionRequest;
  try {
    const response = await fetch(`/api/suggest?id=${encodeURIComponent(id)}`, { cache: "no-store" });
    const envelope = await response.json();
    if (request !== suggestionRequest || state.selected !== id) return;
    const section = $("suggestions");
    if (!response.ok || !envelope.ok) { section.remove(); return; }
    const suggestions = envelope.data.suggestions || [];
    section.replaceChildren(element("h3", "", "Worth a look"), element("p", "", "Suggestions from shared topics and wording. These are hints to review, not recorded relationships."));
    if (!suggestions.length) section.append(element("p", "", "No additional matches in this collection."));
    for (const suggestion of suggestions.slice(0, 5)) {
      const targetId = suggestion.id || suggestion.decision?.id;
      const target = state.graph.decisions[targetId];
      if (!target) continue;
      const node = button("relation suggestion", "", () => openRecord(targetId));
      const text = element("span", "relation-title", target.title);
      const reason = suggestion.reasons;
      if (Array.isArray(reason) && reason.every((item) => typeof item === "string")) text.append(element("small", "suggestion-hint", reason.join(" · ")));
      node.append(badge(target.status), text);
      section.append(node);
    }
  } catch {
    if (request === suggestionRequest && state.selected === id) $("suggestions")?.remove();
  }
}
function renderNotice() {
  const findings = state.graph.findings || [];
  const notice = $("notice");
  notice.hidden = state.valid;
  notice.replaceChildren();
  if (state.valid) return;
  notice.append(element("strong", "", "This collection needs a little attention."), document.createTextNode("Records are shown for inspection. Resolve validation findings before relying on accepted decisions or replacement chains."));
  const list = element("ul");
  for (const finding of findings) list.append(element("li", "", `${finding.file || "Collection"}: ${finding.message} (${finding.code})`));
  notice.append(list);
}
async function refresh() {
  if (loading) return;
  loading = true;
  $("refresh").disabled = true;
  try {
    const response = await fetch("/api/graph", { cache: "no-store" });
    const envelope = await response.json();
    if (!response.ok || !envelope.ok) throw new Error(envelope.error?.message || "The collection could not be read.");
    const next = JSON.stringify(envelope.data);
    if (next !== snapshot) {
      snapshot = next;
      state.graph = envelope.data.graph;
      state.valid = envelope.data.valid;
      state.loaded = true;
      renderFilters();
      renderCollection();
      const selected = state.selected || location.hash.slice(1);
      if (selected && state.graph.decisions[selected]) openRecord(selected, false);
      else if (state.selected) $("detail").close();
    }
    renderNotice();
    $("sync-status").textContent = `Synced ${new Intl.DateTimeFormat("en", { hour: "2-digit", minute: "2-digit" }).format(new Date())}`;
  } catch (error) {
    $("sync-status").textContent = "Connection unavailable";
    $("notice").hidden = false;
    $("notice").replaceChildren(element("strong", "", state.loaded ? "This view may be out of date." : "We couldn’t read your collection."), document.createTextNode(`${error.message} Check the running dashboard command, then refresh.`));
    if (!state.loaded) $("result-count").textContent = "Collection unavailable";
  } finally {
    loading = false;
    $("refresh").disabled = false;
  }
}
$("search").addEventListener("input", (event) => { state.query = event.target.value; renderCollection(); });
$("clear-tags").addEventListener("click", () => { state.tags.clear(); renderFilters(); renderCollection(); });
$("cards-view").addEventListener("click", () => { state.view = "cards"; renderCollection(); });
$("graph-view").addEventListener("click", () => { state.view = "graph"; renderCollection(); });
$("refresh").addEventListener("click", refresh);
$("close-detail").addEventListener("click", () => $("detail").close());
$("detail").addEventListener("close", () => { state.selected = null; suggestionRequest++; history.replaceState(null, "", location.pathname + location.search); });
$("detail").addEventListener("click", (event) => { if (event.target === $("detail") && event.clientX < $("detail").getBoundingClientRect().left) $("detail").close(); });
window.addEventListener("popstate", () => { const id = location.hash.slice(1); if (state.graph.decisions[id]) openRecord(id, false); else $("detail").close(); });
document.addEventListener("keydown", (event) => { if (event.key === "/" && !event.ctrlKey && !event.metaKey && !$("detail").open && !["INPUT", "TEXTAREA"].includes(document.activeElement.tagName)) { event.preventDefault(); $("search").focus(); } });
window.addEventListener("resize", () => { if (state.view === "graph") renderGraph(filtered()); });
setInterval(() => { if (!document.hidden) void refresh(); }, 10000);
document.addEventListener("visibilitychange", () => { if (!document.hidden) void refresh(); });
void refresh();
