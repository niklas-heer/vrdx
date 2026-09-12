//! Responsive Ratatui rendering. Rendering never writes repository files.

use pulldown_cmark::{Event as MarkdownEvent, Options, Parser, Tag, TagEnd};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
};
use unicode_width::UnicodeWidthChar;

use crate::app::{App, Pane, Popup, STATUSES, TextBuffer, relative};

fn color(value: Color, no_color: bool) -> Style {
    if no_color {
        Style::default()
    } else {
        Style::default().fg(value)
    }
}

fn panel(title: impl Into<Line<'static>>, active: bool, no_color: bool) -> Block<'static> {
    let style = if active {
        color(Color::Cyan, no_color).add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(style)
}

pub(crate) fn draw(app: &mut App, frame: &mut Frame<'_>) {
    let area = frame.area();
    app.viewport = area;
    app.save_rect = Rect::default();
    app.cancel_rect = Rect::default();
    app.status_rect = Rect::default();
    if area.width < 80 || area.height < 24 {
        frame.render_widget(
            Paragraph::new(
                "Terminal too small. Resize to at least 80 × 24.\nYour draft is preserved.",
            )
            .block(panel("vrdx — Resize", true, app.no_color))
            .wrap(Wrap { trim: false }),
            area,
        );
        return;
    }
    let [header, body, footer] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(1),
        Constraint::Length(3),
    ])
    .areas(area);
    let mode = app
        .draft
        .as_ref()
        .map_or("VIEW", |draft| if draft.is_new { "NEW" } else { "EDIT" });
    let dirty = if app.current_dirty() {
        "* Unsaved"
    } else if app.draft.is_some() {
        "Unchanged draft"
    } else {
        "Saved"
    };
    let heading = Line::from(vec![
        Span::styled(
            " vrdx ",
            color(Color::Cyan, app.no_color).add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!("{mode}  {dirty}  {}", app.root.display())),
    ]);
    frame.render_widget(Paragraph::new(heading), header);
    let [left, right] =
        Layout::horizontal([Constraint::Percentage(27), Constraint::Min(30)]).areas(body);
    let [decisions, files] =
        Layout::vertical([Constraint::Percentage(58), Constraint::Min(5)]).areas(left);
    draw_decisions(app, frame, decisions);
    draw_files(app, frame, files);
    if body.height >= 30 {
        let [editor, preview] =
            Layout::vertical([Constraint::Length(22), Constraint::Min(7)]).areas(right);
        draw_editor(app, frame, editor);
        draw_preview(app, frame, preview);
    } else if app.pane == Pane::Preview {
        draw_preview(app, frame, right);
    } else {
        draw_editor(app, frame, right);
    }
    let shortcuts = if app.draft.is_some() {
        "Ctrl+S Save · Alt+m Merge · Esc Cancel · Tab Fields · Ctrl+Q Quit"
    } else {
        "Enter Edit · n New · / Search · t Templates · l Links · h History · ? Help"
    };
    let lines = vec![
        Line::from(app.status.clone()),
        Line::from(Span::styled(shortcuts, color(Color::Cyan, app.no_color))),
    ];
    frame.render_widget(
        Paragraph::new(lines)
            .block(Block::default().borders(Borders::TOP))
            .wrap(Wrap { trim: false }),
        footer,
    );
    if let Some(popup) = app.popup.as_ref() {
        draw_popup(app, popup, frame, area);
    }
}

fn draw_decisions(app: &App, frame: &mut Frame<'_>, area: Rect) {
    let records = app
        .current_document()
        .map(|document| document.records.as_slice())
        .unwrap_or_default();
    let items: Vec<_> = if records.is_empty() {
        vec![ListItem::new("No decisions. Press n.")]
    } else {
        records
            .iter()
            .map(|record| {
                ListItem::new(format!(
                    "#{} {}\n  {}",
                    record.id, record.title, record.status
                ))
            })
            .collect()
    };
    let list = List::new(items)
        .block(panel(
            "[1] Decisions",
            app.pane == Pane::Decisions,
            app.no_color,
        ))
        .highlight_symbol("› ")
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));
    let mut state = ListState::default().with_selected(if records.is_empty() {
        None
    } else {
        Some(app.decision_index)
    });
    frame.render_stateful_widget(list, area, &mut state);
}

fn draw_files(app: &App, frame: &mut Frame<'_>, area: Rect) {
    let items: Vec<_> = if app.documents.is_empty() {
        vec![ListItem::new("No Markdown files")]
    } else {
        app.documents
            .iter()
            .map(|document| {
                let label = if document.has_markers {
                    relative(&app.root, &document.path)
                } else {
                    format!("· {}", relative(&app.root, &document.path))
                };
                let item = ListItem::new(label);
                if document.has_markers {
                    item
                } else {
                    item.style(Style::default().add_modifier(Modifier::DIM))
                }
            })
            .collect()
    };
    let list = List::new(items)
        .block(panel("[2] Files", app.pane == Pane::Files, app.no_color))
        .highlight_symbol("› ")
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));
    let mut state = ListState::default().with_selected(if app.documents.is_empty() {
        None
    } else {
        Some(app.file_index)
    });
    frame.render_stateful_widget(list, area, &mut state);
}

fn editor_title(draft: Option<&crate::app::Draft>) -> String {
    draft.map_or_else(
        || "[3] Editor — read only".to_owned(),
        |draft| {
            format!(
                "[3] {} #{}",
                if draft.is_new { "NEW" } else { "EDIT" },
                draft.id
            )
        },
    )
}

fn draw_editor(app: &mut App, frame: &mut Frame<'_>, area: Rect) {
    let draft = app.draft.clone();
    let record = draft
        .as_ref()
        .map(crate::app::Draft::record)
        .or_else(|| app.current_record().cloned());
    let title = editor_title(draft.as_ref());
    let block = panel(title, app.pane == Pane::Editor, app.no_color);
    let inner = block.inner(area);
    frame.render_widget(block, area);
    let Some(record) = record else {
        frame.render_widget(Paragraph::new("Capture a decision and the reason behind it.\n\nPress n to create your first decision.\n\nMarkdown stays in your repository.\nNothing is written until you save.").wrap(Wrap { trim: false }), inner);
        return;
    };
    let values = [
        record.title,
        record.status,
        record.decision,
        record.context,
        record.consequences,
    ];
    let labels = [
        "Title",
        "Status · [Change] · Enter",
        "Decision",
        "Context",
        "Consequences",
    ];
    let focus = draft.as_ref().map(|draft| draft.field);
    let focused_row = focus.map(|field| field.min(5));
    let visible = usize::from(inner.height.checked_div(3).unwrap_or(1)).max(1);
    let total: usize = if draft.is_some() { 6 } else { 5 };
    if let Some(row) = focused_row {
        if row < app.editor_scroll {
            app.editor_scroll = row;
        }
        if row >= app.editor_scroll.saturating_add(visible) {
            app.editor_scroll = row.saturating_add(1).saturating_sub(visible);
        }
    }
    app.editor_scroll = app.editor_scroll.min(total.saturating_sub(visible));
    for (row, (label, value)) in labels
        .iter()
        .zip(values.iter())
        .enumerate()
        .skip(app.editor_scroll)
        .take(visible)
    {
        let y = inner.y.saturating_add(
            u16::try_from(row.saturating_sub(app.editor_scroll))
                .unwrap_or(u16::MAX)
                .saturating_mul(3),
        );
        let field_area = Rect::new(
            inner.x,
            y,
            inner.width,
            3.min(inner.bottom().saturating_sub(y)),
        );
        let focused = focus == Some(row) && app.pane == Pane::Editor && app.popup.is_none();
        let label = if row == 1 && draft.is_none() {
            "Status"
        } else {
            label
        };
        if row == 1 && draft.is_some() {
            app.status_rect = field_area;
        }
        let block = panel((*label).to_owned(), focused, app.no_color);
        let content_area = block.inner(field_area);
        frame.render_widget(block, field_area);
        let buffer = draft.as_ref().and_then(|draft| match row {
            0 => Some(&draft.title),
            2 => Some(&draft.decision),
            3 => Some(&draft.context),
            4 => Some(&draft.consequences),
            _ => None,
        });
        if focused {
            if let Some(buffer) = buffer {
                draw_buffer(buffer, frame, content_area);
            } else {
                frame.render_widget(Paragraph::new(value.as_str()), content_area);
            }
        } else {
            frame.render_widget(
                Paragraph::new(value.as_str()).wrap(Wrap { trim: false }),
                content_area,
            );
        }
    }
    if draft.is_some() && 5 >= app.editor_scroll && 5 < app.editor_scroll.saturating_add(visible) {
        draw_buttons(app, frame, inner, focus);
    }
}

fn draw_buttons(app: &mut App, frame: &mut Frame<'_>, inner: Rect, focus: Option<usize>) {
    let y = inner.y.saturating_add(
        u16::try_from(5_usize.saturating_sub(app.editor_scroll))
            .unwrap_or(u16::MAX)
            .saturating_mul(3),
    );
    let buttons = Rect::new(
        inner.x,
        y,
        inner.width,
        3.min(inner.bottom().saturating_sub(y)),
    );
    let [save, cancel] =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(buttons);
    app.save_rect = save;
    app.cancel_rect = cancel;
    frame.render_widget(
        Paragraph::new("Save · Ctrl+S")
            .block(panel("", focus == Some(5), app.no_color))
            .style(if focus == Some(5) {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                color(Color::Green, app.no_color)
            }),
        save,
    );
    frame.render_widget(
        Paragraph::new("Cancel · Esc").block(panel("", focus == Some(6), app.no_color)),
        cancel,
    );
}

fn draw_buffer(buffer: &TextBuffer, frame: &mut Frame<'_>, area: Rect) {
    if area.is_empty() {
        return;
    }
    let (lines, row, column) = wrapped_buffer(buffer, usize::from(area.width));
    let scroll = row.saturating_sub(usize::from(area.height).saturating_sub(1));
    let visible: Vec<Line<'_>> = lines.into_iter().skip(scroll).map(Line::from).collect();
    frame.render_widget(Paragraph::new(visible), area);
    let x = area.x.saturating_add(
        u16::try_from(column)
            .unwrap_or(0)
            .min(area.width.saturating_sub(1)),
    );
    let y = area.y.saturating_add(
        u16::try_from(row.saturating_sub(scroll))
            .unwrap_or(0)
            .min(area.height.saturating_sub(1)),
    );
    frame.set_cursor_position((x, y));
}

fn wrapped_buffer(buffer: &TextBuffer, width: usize) -> (Vec<String>, usize, usize) {
    let width = width.max(1);
    let mut lines = Vec::new();
    let mut line = String::new();
    let mut column: usize = 0;
    let mut cursor = (0, 0);
    for (index, ch) in buffer.chars.iter().enumerate() {
        let cells = if *ch == '\t' {
            4
        } else {
            UnicodeWidthChar::width(*ch).unwrap_or(0)
        };
        if *ch != '\n' && column.saturating_add(cells) > width {
            lines.push(std::mem::take(&mut line));
            column = 0;
        }
        if index == buffer.cursor {
            cursor = (lines.len(), column);
        }
        if *ch == '\n' {
            lines.push(std::mem::take(&mut line));
            column = 0;
        } else {
            if *ch == '\t' {
                line.push_str("    ");
            } else {
                line.push(*ch);
            }
            column = column.saturating_add(cells);
        }
    }
    if buffer.cursor >= buffer.chars.len() {
        if column >= width {
            lines.push(std::mem::take(&mut line));
            column = 0;
        }
        cursor = (lines.len(), column);
    }
    lines.push(line);
    (lines, cursor.0, cursor.1)
}

fn draw_preview(app: &App, frame: &mut Frame<'_>, area: Rect) {
    let record = app
        .draft
        .as_ref()
        .map(crate::app::Draft::record)
        .or_else(|| app.current_record().cloned());
    let source = record.map_or_else(
        || "No decision selected.\n\nPress n to create a decision.".into(),
        |record| {
            format!(
                "# #{} {}\n\n{}\n\n## Decision\n\n{}\n\n## Context\n\n{}\n\n## Consequences\n\n{}",
                record.id,
                record.title,
                record.status,
                record.decision,
                record.context,
                record.consequences
            )
        },
    );
    frame.render_widget(
        Paragraph::new(markdown_text(&source, app.no_color))
            .block(panel(
                "[4] Preview · j/k scroll",
                app.pane == Pane::Preview,
                app.no_color,
            ))
            .wrap(Wrap { trim: false })
            .scroll((app.preview_scroll, 0)),
        area,
    );
}

fn flush_line(lines: &mut Vec<Line<'static>>, spans: &mut Vec<Span<'static>>) {
    if !spans.is_empty() {
        lines.push(Line::from(std::mem::take(spans)));
    }
}

fn append_markdown_text(
    value: &str,
    style: Style,
    lines: &mut Vec<Line<'static>>,
    spans: &mut Vec<Span<'static>>,
) {
    for (index, part) in value.split('\n').enumerate() {
        if index > 0 {
            lines.push(Line::from(std::mem::take(spans)));
        }
        if !part.is_empty() {
            spans.push(Span::styled(part.to_owned(), style));
        }
    }
}

fn markdown_text(source: &str, no_color: bool) -> Text<'static> {
    let mut lines = Vec::new();
    let mut spans = Vec::new();
    let mut styles = vec![Style::default()];
    let mut destinations = Vec::new();
    for event in Parser::new_ext(source, Options::ENABLE_STRIKETHROUGH) {
        let style = styles.last().copied().unwrap_or_default();
        match event {
            MarkdownEvent::Start(tag) => match tag {
                Tag::Heading { .. } => {
                    flush_line(&mut lines, &mut spans);
                    styles.push(color(Color::Cyan, no_color).add_modifier(Modifier::BOLD));
                }
                Tag::Strong => styles.push(style.add_modifier(Modifier::BOLD)),
                Tag::Emphasis => styles.push(style.add_modifier(Modifier::ITALIC)),
                Tag::Strikethrough => styles.push(style.add_modifier(Modifier::CROSSED_OUT)),
                Tag::CodeBlock(_) => {
                    flush_line(&mut lines, &mut spans);
                    styles.push(color(Color::Yellow, no_color));
                }
                Tag::BlockQuote(_) => {
                    flush_line(&mut lines, &mut spans);
                    spans.push(Span::raw("│ "));
                }
                Tag::Item => {
                    flush_line(&mut lines, &mut spans);
                    spans.push(Span::raw("• "));
                }
                Tag::Link { dest_url, .. } => {
                    styles.push(style.add_modifier(Modifier::UNDERLINED));
                    destinations.push(dest_url.into_string());
                }
                _ => {}
            },
            MarkdownEvent::End(tag) => match tag {
                TagEnd::Heading(_) | TagEnd::CodeBlock => {
                    styles.pop();
                    flush_line(&mut lines, &mut spans);
                }
                TagEnd::Strong | TagEnd::Emphasis | TagEnd::Strikethrough => {
                    styles.pop();
                }
                TagEnd::Link => {
                    styles.pop();
                    if let Some(url) = destinations.pop() {
                        spans.push(Span::styled(
                            format!(" ({url})"),
                            color(Color::Cyan, no_color),
                        ));
                    }
                }
                TagEnd::Paragraph => {
                    flush_line(&mut lines, &mut spans);
                    lines.push(Line::default());
                }
                TagEnd::Item | TagEnd::BlockQuote(_) => flush_line(&mut lines, &mut spans),
                _ => {}
            },
            MarkdownEvent::Text(value) => {
                append_markdown_text(&value, style, &mut lines, &mut spans);
            }
            MarkdownEvent::Code(value) => append_markdown_text(
                &value,
                color(Color::Yellow, no_color).add_modifier(Modifier::BOLD),
                &mut lines,
                &mut spans,
            ),
            MarkdownEvent::SoftBreak => spans.push(Span::raw(" ")),
            MarkdownEvent::HardBreak => lines.push(Line::from(std::mem::take(&mut spans))),
            MarkdownEvent::Rule => {
                flush_line(&mut lines, &mut spans);
                lines.push(Line::raw("────────"));
            }
            MarkdownEvent::TaskListMarker(checked) => {
                spans.push(Span::raw(if checked { "[x] " } else { "[ ] " }));
            }
            _ => {}
        }
    }
    flush_line(&mut lines, &mut spans);
    Text::from(lines)
}

fn draw_choices(
    frame: &mut Frame<'_>,
    area: Rect,
    title: &str,
    items: Vec<String>,
    selected: usize,
    hint: &str,
    no_color: bool,
) {
    let [list, footer] = Layout::vertical([Constraint::Min(1), Constraint::Length(2)]).areas(area);
    let empty = items.is_empty();
    let items: Vec<_> = if empty {
        vec![ListItem::new("No entries")]
    } else {
        items.into_iter().map(ListItem::new).collect()
    };
    frame.render_stateful_widget(
        List::new(items)
            .block(panel(title.to_owned(), true, no_color))
            .highlight_symbol("› ")
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED)),
        list,
        &mut ListState::default().with_selected((!empty).then_some(selected)),
    );
    frame.render_widget(Paragraph::new(hint).wrap(Wrap { trim: false }), footer);
}

fn draw_popup(app: &App, popup: &Popup, frame: &mut Frame<'_>, area: Rect) {
    let width = 68.min(area.width.saturating_sub(4));
    let height = match popup {
        Popup::Help
        | Popup::Search { .. }
        | Popup::Templates { .. }
        | Popup::Links { .. }
        | Popup::History { .. }
        | Popup::Historical { .. } => 20,
        Popup::Status { .. } => 11,
        _ => 9,
    }
    .min(area.height.saturating_sub(2));
    let x = area
        .x
        .saturating_add(area.width.saturating_sub(width).checked_div(2).unwrap_or(0));
    let y = area.y.saturating_add(
        area.height
            .saturating_sub(height)
            .checked_div(2)
            .unwrap_or(0),
    );
    let rectangle = Rect::new(x, y, width, height);
    frame.render_widget(Clear, rectangle);
    match popup {
        Popup::Initialize(path) => {
            let text = format!(
                "Initialize {}?\n\nA decision block will be created on the first successful save.\nCancelling leaves the file unchanged.\n\n[y / Enter] Continue    [n / Esc] Cancel",
                relative(&app.root, path)
            );
            frame.render_widget(
                Paragraph::new(text)
                    .block(panel("Initialize", true, app.no_color))
                    .wrap(Wrap { trim: false }),
                rectangle,
            );
        }
        Popup::Status { selected, .. } => {
            let items: Vec<_> = STATUSES
                .iter()
                .map(|status| ListItem::new(*status))
                .collect();
            let [list, hint] =
                Layout::vertical([Constraint::Min(1), Constraint::Length(2)]).areas(rectangle);
            frame.render_stateful_widget(
                List::new(items)
                    .block(panel("Status", true, app.no_color))
                    .highlight_symbol("› ")
                    .highlight_style(Style::default().add_modifier(Modifier::REVERSED)),
                list,
                &mut ListState::default().with_selected(Some(*selected)),
            );
            frame.render_widget(
                Paragraph::new(" ↑/↓ or j/k · Enter Confirm · Esc Cancel"),
                hint,
            );
        }
        Popup::Guard { selected, .. } => {
            let lines = [
                "Save — persist draft, then continue [s]",
                "Discard — abandon draft, then continue [d]",
                "Stay — keep editing [Esc]",
            ];
            let items: Vec<_> = lines.into_iter().map(ListItem::new).collect();
            frame.render_stateful_widget(
                List::new(items)
                    .block(panel("Unsaved changes", true, app.no_color))
                    .highlight_symbol("› ")
                    .highlight_style(Style::default().add_modifier(Modifier::REVERSED)),
                rectangle,
                &mut ListState::default().with_selected(Some(*selected)),
            );
        }
        Popup::Help => {
            let mut help = String::from(
                "Browse: 1–4 panes · Tab switches · j/k or arrows move\nEnter edits · n/N new · t template · r reload · q quit\n/ searches all repository decisions as you type\nd delete (y confirms; Enter cancels) · J/K reorder\nl relationships: Enter follows; a adds to draft\nh history: n older / p newer · Enter inspect · Esc back\n\nEdit: Tab / Shift+Tab switch fields and buttons\nCtrl+S strict save · Alt+m explicitly merge and save\nm merges from another pane while retaining the draft\nEsc discards · Alt+1–4 panes · Ctrl+Q guarded quit\nEnter or click Change opens Status; Enter confirms\nPaste and Unicode text supported\n\nEnter / Escape closes help.",
            );
            if !app.diagnostics.is_empty() {
                help.push_str("\n\nUnreadable files:\n");
                help.push_str(&app.diagnostics.join("\n"));
            }
            frame.render_widget(
                Paragraph::new(Text::from(help))
                    .block(panel("Help", true, app.no_color))
                    .wrap(Wrap { trim: false }),
                rectangle,
            );
        }
        other => draw_workflow_popup(app, other, frame, rectangle),
    }
}

fn historical_lines(text: &str, width: u16) -> Vec<String> {
    wrapped_buffer(
        &TextBuffer {
            chars: text.chars().collect(),
            cursor: 0,
        },
        usize::from(width),
    )
    .0
}

pub(crate) fn historical_max_scroll(text: &str, viewport: Rect) -> u16 {
    let width = 68.min(viewport.width.saturating_sub(4)).saturating_sub(2);
    let visible = 20.min(viewport.height.saturating_sub(2)).saturating_sub(2);
    u16::try_from(
        historical_lines(text, width)
            .len()
            .saturating_sub(usize::from(visible)),
    )
    .unwrap_or(u16::MAX)
}

fn draw_workflow_popup(app: &App, popup: &Popup, frame: &mut Frame<'_>, rectangle: Rect) {
    match popup {
        Popup::Delete { id } => {
            frame.render_widget(Paragraph::new(format!("Delete decision #{id}?\n\nThis removes the decision from its Markdown file.\n\n[y] Delete    [Enter / n / Esc] Cancel (default)"))
                .block(panel("Confirm deletion", true, app.no_color)).wrap(Wrap { trim: false }), rectangle);
        }
        Popup::Search { query, selected } => {
            let results = app.search_results(&query.text());
            let items = results
                .iter()
                .map(|(file, id, title)| format!("{file}#{id}  {title}"))
                .collect();
            draw_choices(
                frame,
                rectangle,
                &format!("Search: {}", query.text()),
                items,
                *selected,
                "Type to search · ↑/↓ select · Enter open · Esc cancel",
                app.no_color,
            );
        }
        Popup::Templates { names, selected } => draw_choices(
            frame,
            rectangle,
            "Templates",
            names.clone(),
            *selected,
            "↑/↓ select · Enter create draft · Esc cancel",
            app.no_color,
        ),
        Popup::Links {
            targets,
            selected,
            adding,
        } => draw_choices(
            frame,
            rectangle,
            if *adding {
                "Add relationship"
            } else {
                "Relationships"
            },
            targets
                .iter()
                .map(|(file, id, title)| format!("{file}#{id}  {title}"))
                .collect(),
            *selected,
            if *adding {
                "Enter add to draft · Esc cancel"
            } else {
                "Enter follow · a add relationship · Esc close"
            },
            app.no_color,
        ),
        other => draw_history_popup(app, other, frame, rectangle),
    }
}

fn draw_history_popup(app: &App, popup: &Popup, frame: &mut Frame<'_>, rectangle: Rect) {
    match popup {
        Popup::History {
            entries,
            selected,
            offset,
            has_more,
        } => draw_choices(
            frame,
            rectangle,
            &format!(
                "Git history · commits {}–{}",
                offset.saturating_add(1),
                offset.saturating_add(entries.len())
            ),
            entries
                .iter()
                .map(|(revision, summary)| {
                    format!("{} {summary}", revision.chars().take(8).collect::<String>())
                })
                .collect(),
            *selected,
            if *has_more {
                "↑/↓ select · Enter inspect · n older · p newer · Esc close"
            } else {
                "↑/↓ select · Enter inspect · p newer · Esc close"
            },
            app.no_color,
        ),
        Popup::Historical {
            title,
            text,
            scroll,
            ..
        } => {
            let width = rectangle.width.saturating_sub(2);
            let lines = historical_lines(text, width);
            let visible = usize::from(rectangle.height.saturating_sub(2));
            let maximum = lines.len().saturating_sub(visible);
            let scroll = usize::from(*scroll).min(maximum);
            let position = format!(
                " Lines {}–{}/{} · ↑/↓ Home/End · Esc back ",
                scroll.saturating_add(1),
                scroll.saturating_add(visible).min(lines.len()),
                lines.len()
            );
            let block =
                panel(format!("History: {title}"), true, app.no_color).title_bottom(position);
            let lines: Vec<_> = lines
                .into_iter()
                .skip(scroll)
                .take(visible)
                .map(Line::from)
                .collect();
            frame.render_widget(Paragraph::new(lines).block(block), rectangle);
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    // Test failures are returned as errors, keeping the same no-panic rules as production.
    macro_rules! check {
        ($condition:expr) => {
            if !$condition {
                return Err(std::io::Error::other(concat!(
                    "check failed: ",
                    stringify!($condition)
                ))
                .into());
            }
        };
    }
    macro_rules! check_eq {
        ($actual:expr, $expected:expr $(,)?) => {
            match (&$actual, &$expected) {
                (actual, expected) if actual == expected => {}
                (actual, expected) => {
                    return Err(std::io::Error::other(format!(
                        "Expected {expected:?}; got {actual:?}"
                    ))
                    .into())
                }
            }
        };
    }

    use super::*;
    use crossterm::event::{
        Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
    };
    use ratatui::{Terminal, backend::TestBackend};

    fn screen(terminal: &Terminal<TestBackend>) -> String {
        terminal
            .backend()
            .buffer()
            .content
            .iter()
            .map(ratatui::buffer::Cell::symbol)
            .collect()
    }

    fn key(app: &mut App, code: KeyCode) {
        app.handle_event(Event::Key(KeyEvent::new(code, KeyModifiers::NONE)));
    }

    #[test]
    fn compact_layout_all_fields_buttons_preview_and_resize_are_reachable()
    -> Result<(), Box<dyn std::error::Error>> {
        let directory = tempfile::tempdir()?;
        let mut app = App::new(directory.path().to_path_buf())?;
        let Ok(mut terminal) = Terminal::new(TestBackend::new(80, 24));
        key(&mut app, KeyCode::Char('n'));
        let Ok(_) = terminal.draw(|frame| app.draw(frame));
        check!(screen(&terminal).contains("Initialize"));
        key(&mut app, KeyCode::Char('y'));
        let Ok(_) = terminal.draw(|frame| app.draw(frame));
        check!(screen(&terminal).contains("Status"));
        key(&mut app, KeyCode::Enter);
        app.handle_event(Event::Paste("Responsive café 界".into()));
        for _ in 0..5 {
            key(&mut app, KeyCode::Tab);
        }
        let Ok(_) = terminal.draw(|frame| app.draw(frame));
        check!(screen(&terminal).contains("Save · Ctrl+S"));
        check!(!app.save_rect.is_empty());
        check!(
            terminal
                .backend()
                .buffer()
                .area
                .contains((app.save_rect.x, app.save_rect.y).into())
        );
        app.handle_event(Event::Key(KeyEvent::new(
            KeyCode::Char('4'),
            KeyModifiers::ALT,
        )));
        let Ok(_) = terminal.draw(|frame| app.draw(frame));
        check!(screen(&terminal).contains("Preview"));
        check!(screen(&terminal).contains("Responsive café 界"));
        terminal.backend_mut().resize(60, 15);
        let Ok(()) = terminal.autoresize();
        let Ok(_) = terminal.draw(|frame| app.draw(frame));
        check!(screen(&terminal).contains("Terminal too small"));
        terminal.backend_mut().resize(120, 40);
        let Ok(()) = terminal.autoresize();
        let Ok(_) = terminal.draw(|frame| app.draw(frame));
        check!(screen(&terminal).contains("Preview"));
        check!(app.current_dirty());
        Ok(())
    }

    #[test]
    fn save_and_cancel_mouse_buttons_execute_without_rendering_errors()
    -> Result<(), Box<dyn std::error::Error>> {
        let directory = tempfile::tempdir()?;
        let mut app = App::new(directory.path().to_path_buf())?;
        let Ok(mut terminal) = Terminal::new(TestBackend::new(120, 40));
        key(&mut app, KeyCode::Char('n'));
        key(&mut app, KeyCode::Char('y'));
        key(&mut app, KeyCode::Enter);
        app.handle_event(Event::Paste("Mouse save".into()));
        let Ok(_) = terminal.draw(|frame| app.draw(frame));
        let click = |rectangle: Rect| {
            Event::Mouse(MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                column: rectangle.x.saturating_add(1),
                row: rectangle.y.saturating_add(1),
                modifiers: KeyModifiers::NONE,
            })
        };
        app.handle_event(click(app.save_rect));
        check!(app.draft.is_none());
        check!(directory.path().join("DECISIONS.md").exists());
        key(&mut app, KeyCode::Enter);
        app.handle_event(Event::Paste(" not saved".into()));
        let Ok(_) = terminal.draw(|frame| app.draw(frame));
        app.handle_event(click(app.cancel_rect));
        check!(app.draft.is_none());
        check_eq!(
            app.current_record().map(|record| record.title.as_str()),
            Some("Mouse save")
        );
        Ok(())
    }

    #[test]
    fn markdown_preview_styles_emphasis_and_preserves_links()
    -> Result<(), Box<dyn std::error::Error>> {
        let rendered = markdown_text(
            "# Heading\n\n**Strong** and `code` [guide](https://example.org)\n\n- item",
            true,
        );
        let plain: String = rendered
            .lines
            .iter()
            .flat_map(|line| line.spans.iter().map(|span| span.content.as_ref()))
            .collect();
        check!(plain.contains("Strong and code guide (https://example.org)"));
        check!(plain.contains("• item"));
        check!(!plain.contains("**"));
        check!(
            rendered
                .lines
                .iter()
                .flat_map(|line| &line.spans)
                .any(|span| span.content == "Strong"
                    && span.style.add_modifier.contains(Modifier::BOLD))
        );
        Ok(())
    }

    #[test]
    fn unicode_width_keeps_cursor_on_valid_visible_cell() -> Result<(), Box<dyn std::error::Error>>
    {
        let text = TextBuffer {
            chars: "a界🙂".chars().collect(),
            cursor: 3,
        };
        let (lines, row, column) = wrapped_buffer(&text, 4);
        check_eq!(lines, vec!["a界", "🙂"]);
        check_eq!((row, column), (1, 2));
        Ok(())
    }
}
