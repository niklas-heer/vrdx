//! Application state and keyboard interactions, independent of terminal ownership.

use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crossterm::event::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseButton, MouseEventKind,
};
use ratatui::{Frame, layout::Rect};

use crate::{
    document::{Document, Record},
    repository::Repository,
};

pub(crate) const STATUSES: [&str; 5] = [
    "📝 Draft",
    "✅ Accepted",
    "❌ Rejected",
    "⛔ Deprecated by …",
    "⬆️ Supersedes …",
];

/// Focusable regions of the application.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum Pane {
    #[default]
    Decisions,
    Files,
    Editor,
    Preview,
}

impl Pane {
    const fn next(self, reverse: bool) -> Self {
        match (self, reverse) {
            (Self::Decisions, false) | (Self::Editor, true) => Self::Files,
            (Self::Files, false) | (Self::Preview, true) => Self::Editor,
            (Self::Editor, false) | (Self::Decisions, true) => Self::Preview,
            (Self::Preview, false) | (Self::Files, true) => Self::Decisions,
        }
    }
}

/// Character positions avoid slicing UTF-8 at invalid byte boundaries.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct TextBuffer {
    pub chars: Vec<char>,
    pub cursor: usize,
}

impl TextBuffer {
    fn from_text(value: &str) -> Self {
        let chars: Vec<_> = value.chars().collect();
        let cursor = chars.len();
        Self { chars, cursor }
    }

    pub fn text(&self) -> String {
        self.chars.iter().collect()
    }

    fn insert(&mut self, text: &str, single_line: bool) {
        for ch in text.chars() {
            if ch == '\r' || (ch.is_control() && ch != '\n' && ch != '\t') {
                continue;
            }
            let value = if single_line && (ch == '\n' || ch == '\t') {
                ' '
            } else {
                ch
            };
            let position = self.cursor.min(self.chars.len());
            self.chars.insert(position, value);
            self.cursor = position.saturating_add(1);
        }
    }

    fn line_start(&self) -> usize {
        self.chars
            .iter()
            .take(self.cursor)
            .rposition(|ch| *ch == '\n')
            .map_or(0, |position| position.saturating_add(1))
    }

    fn line_end(&self) -> usize {
        self.chars
            .iter()
            .enumerate()
            .skip(self.cursor)
            .find(|(_, ch)| **ch == '\n')
            .map_or(self.chars.len(), |(position, _)| position)
    }

    fn vertical(&mut self, down: bool) {
        let start = self.line_start();
        let column = self.cursor.saturating_sub(start);
        if down {
            let end = self.line_end();
            if end < self.chars.len() {
                let next_start = end.saturating_add(1);
                let next_end = self
                    .chars
                    .iter()
                    .enumerate()
                    .skip(next_start)
                    .find(|(_, ch)| **ch == '\n')
                    .map_or(self.chars.len(), |(position, _)| position);
                self.cursor = next_start.saturating_add(column).min(next_end);
            }
        } else if start > 0 {
            let previous_end = start.saturating_sub(1);
            let previous_start = self
                .chars
                .iter()
                .take(previous_end)
                .rposition(|ch| *ch == '\n')
                .map_or(0, |position| position.saturating_add(1));
            self.cursor = previous_start.saturating_add(column).min(previous_end);
        }
    }

    fn key(&mut self, key: KeyEvent, single_line: bool) {
        match key.code {
            KeyCode::Char(ch)
                if !key
                    .modifiers
                    .intersects(KeyModifiers::CONTROL | KeyModifiers::ALT) =>
            {
                self.insert(&ch.to_string(), single_line);
            }
            KeyCode::Enter if !single_line => self.insert("\n", false),
            KeyCode::Left => self.cursor = self.cursor.saturating_sub(1),
            KeyCode::Right => self.cursor = self.cursor.saturating_add(1).min(self.chars.len()),
            KeyCode::Home => self.cursor = self.line_start(),
            KeyCode::End => self.cursor = self.line_end(),
            KeyCode::Up => self.vertical(false),
            KeyCode::Down => self.vertical(true),
            KeyCode::Backspace if self.cursor > 0 => {
                self.cursor = self.cursor.saturating_sub(1).min(self.chars.len());
                if self.cursor < self.chars.len() {
                    self.chars.remove(self.cursor);
                }
            }
            KeyCode::Delete if self.cursor < self.chars.len() => {
                self.chars.remove(self.cursor);
            }
            _ => {}
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Draft {
    pub path: PathBuf,
    pub id: u64,
    pub title: TextBuffer,
    pub status: String,
    pub decision: TextBuffer,
    pub context: TextBuffer,
    pub consequences: TextBuffer,
    pub baseline: Record,
    pub is_new: bool,
    pub field: usize,
}

impl Draft {
    fn new(path: PathBuf, record: Record, is_new: bool) -> Self {
        Self {
            path,
            id: record.id,
            title: TextBuffer::from_text(&record.title),
            status: record.status.clone(),
            decision: TextBuffer::from_text(&record.decision),
            context: TextBuffer::from_text(&record.context),
            consequences: TextBuffer::from_text(&record.consequences),
            baseline: record,
            is_new,
            field: 0,
        }
    }

    pub fn record(&self) -> Record {
        Record {
            id: self.id,
            title: self.title.text(),
            status: self.status.clone(),
            decision: self.decision.text(),
            context: self.context.text(),
            consequences: self.consequences.text(),
        }
    }

    pub fn dirty(&self) -> bool {
        self.record() != self.baseline
    }

    const fn buffer_mut(&mut self) -> Option<&mut TextBuffer> {
        match self.field {
            0 => Some(&mut self.title),
            2 => Some(&mut self.decision),
            3 => Some(&mut self.context),
            4 => Some(&mut self.consequences),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum Action {
    New,
    Refresh,
    Quit,
    File(usize),
    Decision(usize),
    Edit,
    Delete,
    Move(bool),
    Search,
    Templates,
    Links,
    History,
}

#[derive(Clone, Debug)]
pub(crate) enum Popup {
    Initialize(PathBuf),
    Status {
        selected: usize,
        target: Option<PathBuf>,
    },
    Guard {
        action: Action,
        selected: usize,
    },
    Help,
    Delete {
        id: u64,
    },
    Search {
        query: TextBuffer,
        selected: usize,
    },
    Templates {
        names: Vec<String>,
        selected: usize,
    },
    Links {
        targets: Vec<(String, u64, String)>,
        selected: usize,
        adding: bool,
    },
    History {
        entries: Vec<(String, String)>,
        selected: usize,
        offset: usize,
        has_more: bool,
    },
    Historical {
        title: String,
        text: String,
        scroll: u16,
        entries: Vec<(String, String)>,
        selected: usize,
        offset: usize,
        has_more: bool,
    },
}

/// All mutable UI state. Terminal setup and restoration belong to the caller.
pub struct App {
    pub(crate) root: PathBuf,
    pub(crate) documents: Vec<Document>,
    pub(crate) file_index: usize,
    pub(crate) decision_index: usize,
    pub(crate) pane: Pane,
    pub(crate) draft: Option<Draft>,
    pub(crate) popup: Option<Popup>,
    pub(crate) status: String,
    pub(crate) diagnostics: Vec<String>,
    pub(crate) editor_scroll: usize,
    pub(crate) preview_scroll: u16,
    pub(crate) save_rect: Rect,
    pub(crate) cancel_rect: Rect,
    pub(crate) status_rect: Rect,
    pub(crate) viewport: Rect,
    pub(crate) no_color: bool,
    /// Set when a quit request has been accepted.
    pub quit: bool,
}

impl App {
    /// Discover readable repository Markdown files.
    ///
    /// # Errors
    /// Returns an error if the repository root cannot be resolved or is not a directory.
    pub fn new(root: PathBuf) -> io::Result<Self> {
        let root = fs::canonicalize(root)?;
        if !root.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::NotADirectory,
                "repository root must be a directory",
            ));
        }
        let mut app = Self {
            root,
            documents: Vec::new(),
            file_index: 0,
            decision_index: 0,
            pane: Pane::Decisions,
            draft: None,
            popup: None,
            status: String::new(),
            diagnostics: Vec::new(),
            editor_scroll: 0,
            preview_scroll: 0,
            save_rect: Rect::default(),
            cancel_rect: Rect::default(),
            status_rect: Rect::default(),
            viewport: Rect::default(),
            no_color: std::env::var_os("NO_COLOR").is_some(),
            quit: false,
        };
        app.reload();
        Ok(app)
    }

    pub(crate) fn current_document(&self) -> Option<&Document> {
        self.documents.get(self.file_index)
    }

    pub(crate) fn current_record(&self) -> Option<&Record> {
        self.current_document()
            .and_then(|document| document.records.get(self.decision_index))
    }

    pub(crate) fn current_dirty(&self) -> bool {
        self.draft.as_ref().is_some_and(Draft::dirty)
    }

    fn reload(&mut self) {
        let old_path = self
            .current_document()
            .map(|document| document.path.clone());
        let old_id = self.current_record().map(|record| record.id);
        self.documents.clear();
        self.diagnostics.clear();
        for path in discover(&self.root, &mut self.diagnostics) {
            match Document::load(&path) {
                Ok(document) => self.documents.push(document),
                Err(error) => self
                    .diagnostics
                    .push(format!("{}: {error}", relative(&self.root, &path))),
            }
        }
        self.documents.sort_by(|left, right| {
            (!left.has_markers, &left.path).cmp(&(!right.has_markers, &right.path))
        });
        self.file_index = self
            .documents
            .iter()
            .position(|document| Some(&document.path) == old_path.as_ref())
            .unwrap_or(0);
        self.decision_index = self
            .current_document()
            .and_then(|document| {
                document
                    .records
                    .iter()
                    .position(|record| Some(record.id) == old_id)
            })
            .unwrap_or(0);
        self.draft = None;
        self.preview_scroll = 0;
        self.status = if let Some(error) = self.diagnostics.first() {
            format!("{} unreadable file(s): {error}", self.diagnostics.len())
        } else if self.documents.is_empty() {
            "No Markdown files. Press n to create DECISIONS.md.".into()
        } else {
            format!("Loaded {} Markdown file(s)", self.documents.len())
        };
    }

    fn request(&mut self, action: Action) {
        if self.current_dirty() {
            self.popup = Some(Popup::Guard {
                action,
                selected: 2,
            });
        } else {
            self.draft = None;
            self.execute(action);
        }
    }

    fn execute(&mut self, action: Action) {
        match action {
            Action::Quit => self.quit = true,
            Action::Refresh => {
                self.reload();
                self.pane = Pane::Decisions;
            }
            Action::New => self.new_decision(),
            Action::File(index) if index < self.documents.len() => {
                self.file_index = index;
                self.decision_index = 0;
                self.preview_scroll = 0;
                self.status =
                    "File selected. Enter on a decision to edit; n creates a decision.".into();
            }
            Action::Decision(index)
                if self
                    .current_document()
                    .is_some_and(|document| index < document.records.len()) =>
            {
                self.decision_index = index;
                self.preview_scroll = 0;
            }
            Action::Edit => self.edit_selected(),
            Action::Delete => {
                if let Some(record) = self.current_record() {
                    self.popup = Some(Popup::Delete { id: record.id });
                }
            }
            Action::Move(down) => self.reorder(down),
            Action::Search => {
                self.popup = Some(Popup::Search {
                    query: TextBuffer::default(),
                    selected: 0,
                });
            }
            Action::Templates => self.open_templates(),
            Action::Links => self.open_links(false),
            Action::History => self.open_history(),
            Action::File(_) | Action::Decision(_) => {}
        }
    }

    fn new_decision(&mut self) {
        let path = self.current_document().map_or_else(
            || self.root.join("DECISIONS.md"),
            |document| document.path.clone(),
        );
        if self
            .current_document()
            .is_none_or(|document| !document.has_markers)
        {
            self.popup = Some(Popup::Initialize(path));
        } else {
            self.popup = Some(Popup::Status {
                selected: 0,
                target: Some(path),
            });
        }
    }

    fn begin_new(&mut self, path: PathBuf, status: &str) {
        let next = self
            .documents
            .iter()
            .find(|document| document.path == path)
            .map_or_else(|| Document::new(path.clone()).next_id(), Document::next_id);
        match next {
            Ok(id) => {
                let mut record = Record::new(id);
                record.status = status.into();
                self.draft = Some(Draft::new(path, record, true));
                self.editor_scroll = 0;
                self.pane = Pane::Editor;
                self.status = "Creating decision — Ctrl+S saves, Esc discards.".into();
            }
            Err(error) => self.status = error.to_string(),
        }
    }

    fn edit_selected(&mut self) {
        let selected = self.current_document().and_then(|document| {
            document
                .records
                .get(self.decision_index)
                .map(|record| (document.path.clone(), record.clone()))
        });
        if let Some((path, record)) = selected {
            self.draft = Some(Draft::new(path, record, false));
            self.editor_scroll = 0;
            self.pane = Pane::Editor;
            self.status = "Editing — Ctrl+S saves, Esc discards. Alt+4 previews.".into();
        } else {
            self.status = "No decision selected. Press n to create one.".into();
        }
    }

    fn save(&mut self) -> bool {
        self.persist(false)
    }

    fn persist(&mut self, merge: bool) -> bool {
        let Some(draft) = self.draft.as_ref() else {
            return false;
        };
        let mut record = draft.record();
        record.title = record.title.trim().to_owned();
        let path = draft.path.clone();
        if record.title.trim().is_empty() || record.title.contains(['\n', '\r']) {
            self.status = "Title must contain text on a single line.".into();
            if let Some(draft) = self.draft.as_mut() {
                draft.field = 0;
            }
            self.pane = Pane::Editor;
            return false;
        }
        if let Err(error) = validate_target(&self.root, &path) {
            self.status = format!("Save failed: {error}");
            self.pane = Pane::Editor;
            return false;
        }
        #[expect(
            clippy::option_if_let_else,
            reason = "The fallback mutates the same document collection; closures would overlap its mutable borrow."
        )]
        let result = if let Some(document) = self
            .documents
            .iter_mut()
            .find(|document| document.path == path)
        {
            if merge {
                document.merge_record(&record)
            } else {
                document.save_record(&record)
            }
        } else {
            let mut document = Document::new(path.clone());
            document
                .save_record(&record)
                .map(|()| self.documents.push(document))
        };
        if let Err(error) = result {
            self.status = format!("Save failed: {error}");
            self.pane = Pane::Editor;
            return false;
        }
        self.file_index = self
            .documents
            .iter()
            .position(|document| document.path == path)
            .unwrap_or(0);
        self.decision_index = self
            .current_document()
            .and_then(|document| {
                document
                    .records
                    .iter()
                    .position(|entry| entry.id == record.id)
            })
            .unwrap_or(0);
        self.draft = None;
        self.pane = Pane::Decisions;
        self.preview_scroll = 0;
        self.status = format!(
            "Saved decision #{} to {}",
            record.id,
            relative(&self.root, &path)
        );
        true
    }

    fn cancel(&mut self) {
        self.draft = None;
        self.popup = None;
        self.pane = Pane::Decisions;
        self.status = "Editing cancelled. No draft changes written.".into();
    }

    /// Handle a terminal input event; failures are retained as visible status messages.
    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::Key(key) if key.kind != KeyEventKind::Release => self.handle_key(key),
            Event::Paste(text) if matches!(self.popup, Some(Popup::Search { .. })) => {
                if let Some(Popup::Search { query, selected }) = self.popup.as_mut() {
                    query.insert(&text, true);
                    *selected = 0;
                }
            }
            Event::Paste(text) if self.popup.is_none() && self.pane == Pane::Editor => {
                if let Some(draft) = self.draft.as_mut() {
                    let single_line = draft.field == 0;
                    if let Some(buffer) = draft.buffer_mut() {
                        buffer.insert(&text.replace("\r\n", "\n"), single_line);
                    }
                }
            }
            Event::Mouse(mouse)
                if mouse.kind == MouseEventKind::Down(MouseButton::Left)
                    && self.popup.is_none()
                    && self.draft.is_some() =>
            {
                let position = ratatui::layout::Position::new(mouse.column, mouse.row);
                if self.save_rect.contains(position) {
                    self.save();
                } else if self.cancel_rect.contains(position) {
                    self.cancel();
                } else if self.status_rect.contains(position) {
                    self.choose_status();
                }
            }
            _ => {}
        }
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if self.popup.is_some() {
            self.popup_key(key);
            return;
        }
        if key.modifiers.contains(KeyModifiers::CONTROL) {
            match key.code {
                KeyCode::Char('s') => {
                    self.save();
                    return;
                }
                KeyCode::Char('q') => {
                    self.request(Action::Quit);
                    return;
                }
                _ => {}
            }
        }
        if key.modifiers.contains(KeyModifiers::ALT) && key.code == KeyCode::Char('m') {
            self.persist(true);
            return;
        }
        if key.modifiers.contains(KeyModifiers::ALT)
            && let Some(pane) = number_pane(key.code)
        {
            self.pane = pane;
            return;
        }
        if self.draft.is_some() && self.pane == Pane::Editor {
            self.edit_key(key);
            return;
        }
        if let Some(pane) = number_pane(key.code) {
            self.pane = pane;
            return;
        }
        match key.code {
            KeyCode::Char('q') => self.request(Action::Quit),
            KeyCode::Char('n' | 'N') => self.request(Action::New),
            KeyCode::Char('r') => self.request(Action::Refresh),
            KeyCode::Char('d') => self.request(Action::Delete),
            KeyCode::Char('J') => self.request(Action::Move(true)),
            KeyCode::Char('K') => self.request(Action::Move(false)),
            KeyCode::Char('/') => self.request(Action::Search),
            KeyCode::Char('t') => self.request(Action::Templates),
            KeyCode::Char('l') => self.request(Action::Links),
            KeyCode::Char('h') => self.request(Action::History),
            KeyCode::Char('m') => {
                self.persist(true);
            }
            KeyCode::Char('?') => self.popup = Some(Popup::Help),
            KeyCode::Char('s') => {
                self.save();
            }
            KeyCode::Esc if self.draft.is_some() => self.cancel(),
            KeyCode::Tab => self.pane = self.pane.next(false),
            KeyCode::BackTab => self.pane = self.pane.next(true),
            KeyCode::Enter | KeyCode::Char(' ') => {
                if self.pane == Pane::Files {
                    self.pane = Pane::Decisions;
                } else {
                    self.request(Action::Edit);
                }
            }
            KeyCode::Down | KeyCode::Char('j') => self.navigate(true),
            KeyCode::Up | KeyCode::Char('k') => self.navigate(false),
            _ => {}
        }
    }

    fn navigate(&mut self, down: bool) {
        match self.pane {
            Pane::Files => {
                let next = move_index(self.file_index, self.documents.len(), down);
                if next != self.file_index {
                    self.request(Action::File(next));
                }
            }
            Pane::Decisions => {
                let length = self
                    .current_document()
                    .map_or(0, |document| document.records.len());
                let next = move_index(self.decision_index, length, down);
                if next != self.decision_index {
                    self.request(Action::Decision(next));
                }
            }
            Pane::Preview => {
                self.preview_scroll = if down {
                    self.preview_scroll.saturating_add(1)
                } else {
                    self.preview_scroll.saturating_sub(1)
                }
            }
            Pane::Editor => {}
        }
    }

    fn edit_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => self.cancel(),
            KeyCode::Tab | KeyCode::BackTab => {
                if let Some(draft) = self.draft.as_mut() {
                    draft.field = if key.code == KeyCode::BackTab {
                        if draft.field == 0 {
                            6
                        } else {
                            draft.field.saturating_sub(1)
                        }
                    } else if draft.field >= 6 {
                        0
                    } else {
                        draft.field.saturating_add(1)
                    };
                }
            }
            _ => {
                let field = self.draft.as_ref().map_or(0, |draft| draft.field);
                if field == 1
                    && matches!(
                        key.code,
                        KeyCode::Enter | KeyCode::Char(' ') | KeyCode::Down | KeyCode::Up
                    )
                {
                    self.choose_status();
                } else if field >= 5 && matches!(key.code, KeyCode::Enter | KeyCode::Char(' ')) {
                    if field == 5 {
                        self.save();
                    } else {
                        self.cancel();
                    }
                } else if let Some(draft) = self.draft.as_mut()
                    && let Some(buffer) = draft.buffer_mut()
                {
                    buffer.key(key, field == 0);
                }
            }
        }
    }

    fn choose_status(&mut self) {
        let selected = self
            .draft
            .as_ref()
            .and_then(|draft| STATUSES.iter().position(|status| *status == draft.status))
            .unwrap_or(0);
        self.popup = Some(Popup::Status {
            selected,
            target: None,
        });
        self.pane = Pane::Editor;
    }

    fn popup_key(&mut self, key: KeyEvent) {
        let Some(popup) = self.popup.take() else {
            return;
        };
        match popup {
            Popup::Help => {
                if !matches!(
                    key.code,
                    KeyCode::Esc | KeyCode::Enter | KeyCode::Char('?' | 'q')
                ) {
                    self.popup = Some(Popup::Help);
                }
            }
            Popup::Initialize(path) => match key.code {
                KeyCode::Char('y' | 'Y') | KeyCode::Enter => {
                    self.popup = Some(Popup::Status {
                        selected: 0,
                        target: Some(path),
                    });
                }
                KeyCode::Esc | KeyCode::Char('n' | 'N') => {
                    self.status = "Initialization cancelled; no files changed.".into();
                }
                _ => self.popup = Some(Popup::Initialize(path)),
            },
            Popup::Status { selected, target } => match key.code {
                KeyCode::Esc => {}
                KeyCode::Enter => {
                    if let Some(status) = STATUSES.get(selected) {
                        if let Some(path) = target {
                            self.begin_new(path, status);
                        } else if let Some(draft) = self.draft.as_mut() {
                            draft.status = (*status).into();
                        }
                    }
                }
                _ => {
                    let selected = popup_index(selected, STATUSES.len(), key.code);
                    self.popup = Some(Popup::Status { selected, target });
                }
            },
            Popup::Guard { action, selected } => {
                let choice = match key.code {
                    KeyCode::Char('s') => Some(0),
                    KeyCode::Char('d') => Some(1),
                    KeyCode::Esc | KeyCode::Char('c') => Some(2),
                    KeyCode::Enter => Some(selected),
                    _ => None,
                };
                match choice {
                    Some(0) => {
                        if self.save() {
                            self.execute(action);
                        }
                    }
                    Some(1) => {
                        self.draft = None;
                        self.execute(action);
                    }
                    Some(_) => {}
                    None => {
                        self.popup = Some(Popup::Guard {
                            action,
                            selected: popup_index(selected, 3, key.code),
                        });
                    }
                }
            }
            other => self.workflow_key(other, key),
        }
    }

    fn workflow_key(&mut self, popup: Popup, key: KeyEvent) {
        match popup {
            Popup::Delete { id } => match key.code {
                KeyCode::Char('y' | 'Y') => self.delete(id),
                KeyCode::Esc | KeyCode::Enter | KeyCode::Char('n' | 'N') => {
                    self.status = "Deletion cancelled.".into();
                }
                _ => self.popup = Some(Popup::Delete { id }),
            },
            Popup::Search {
                mut query,
                mut selected,
            } => {
                match key.code {
                    KeyCode::Esc => return,
                    KeyCode::Enter => {
                        if let Some((file, id, _)) =
                            self.search_results(&query.text()).get(selected).cloned()
                        {
                            self.select_identity(&file, id);
                        }
                        return;
                    }
                    KeyCode::Down => {
                        selected =
                            move_index(selected, self.search_results(&query.text()).len(), true);
                    }
                    KeyCode::Up => selected = selected.saturating_sub(1),
                    _ => {
                        query.key(key, true);
                        selected = 0;
                    }
                }
                self.popup = Some(Popup::Search { query, selected });
            }
            Popup::Templates { names, selected } => {
                if key.code == KeyCode::Enter {
                    if let Some(name) = names.get(selected) {
                        self.use_template(name);
                    }
                } else if key.code != KeyCode::Esc {
                    let selected = popup_index(selected, names.len(), key.code);
                    self.popup = Some(Popup::Templates { names, selected });
                }
            }
            Popup::Links {
                targets,
                selected,
                adding,
            } => match key.code {
                KeyCode::Esc => {}
                KeyCode::Char('a') => self.open_links(true),
                KeyCode::Enter => {
                    if let Some((file, id, _)) = targets.get(selected) {
                        if adding {
                            self.add_link(file, *id);
                        } else {
                            self.select_identity(file, *id);
                        }
                    }
                }
                _ => {
                    let selected = popup_index(selected, targets.len(), key.code);
                    self.popup = Some(Popup::Links {
                        targets,
                        selected,
                        adding,
                    });
                }
            },
            other => self.history_key(other, key),
        }
    }

    fn history_key(&mut self, popup: Popup, key: KeyEvent) {
        match popup {
            Popup::History {
                entries,
                selected,
                offset,
                has_more,
            } => {
                if key.code == KeyCode::Char('n') && has_more {
                    let next_offset = offset.saturating_add(entries.len());
                    self.popup = Some(Popup::History {
                        entries,
                        selected,
                        offset,
                        has_more,
                    });
                    self.history_at(next_offset);
                } else if key.code == KeyCode::Char('p') && offset > 0 {
                    self.popup = Some(Popup::History {
                        entries,
                        selected,
                        offset,
                        has_more,
                    });
                    self.history_at(offset.saturating_sub(200));
                } else if key.code == KeyCode::Enter {
                    if let Some((revision, summary)) = entries.get(selected) {
                        let result = self.repository().and_then(|repository| {
                            repository.historical(
                                &self.current_file(),
                                self.current_record().map_or(0, |record| record.id),
                                revision,
                            )
                        });
                        match result {
                            Ok(record) => {
                                self.popup = Some(Popup::Historical {
                                    title: format!(
                                        "{} {summary}",
                                        revision.chars().take(8).collect::<String>()
                                    ),
                                    text: format!(
                                        "#{} {}\n{}\n\nDecision\n{}\n\nContext\n{}\n\nConsequences\n{}",
                                        record.id,
                                        record.title,
                                        record.status,
                                        record.decision,
                                        record.context,
                                        record.consequences
                                    ),
                                    scroll: 0,
                                    entries,
                                    selected,
                                    offset,
                                    has_more,
                                });
                            }
                            Err(error) => {
                                self.status = error.to_string();
                                self.popup = Some(Popup::History {
                                    entries,
                                    selected,
                                    offset,
                                    has_more,
                                });
                            }
                        }
                    }
                } else if key.code != KeyCode::Esc {
                    let selected = popup_index(selected, entries.len(), key.code);
                    self.popup = Some(Popup::History {
                        entries,
                        selected,
                        offset,
                        has_more,
                    });
                }
            }
            other => self.historical_key(other, key),
        }
    }

    fn historical_key(&mut self, popup: Popup, key: KeyEvent) {
        if let Popup::Historical {
            title,
            text,
            mut scroll,
            entries,
            selected,
            offset,
            has_more,
        } = popup
        {
            if key.code == KeyCode::Esc {
                self.popup = Some(Popup::History {
                    entries,
                    selected,
                    offset,
                    has_more,
                });
            } else {
                let maximum = crate::ui::historical_max_scroll(&text, self.viewport);
                scroll = scroll.min(maximum);
                match key.code {
                    KeyCode::Down | KeyCode::Char('j') => {
                        scroll = scroll.saturating_add(1).min(maximum);
                    }
                    KeyCode::Up | KeyCode::Char('k') => scroll = scroll.saturating_sub(1),
                    KeyCode::Home => scroll = 0,
                    KeyCode::End => scroll = maximum,
                    _ => {}
                }
                self.popup = Some(Popup::Historical {
                    title,
                    text,
                    scroll,
                    entries,
                    selected,
                    offset,
                    has_more,
                });
            }
        }
    }

    fn repository(&self) -> Result<Repository, crate::repository::Error> {
        Repository::new(&self.root)
    }

    fn current_file(&self) -> String {
        self.current_document().map_or_else(
            || "DECISIONS.md".into(),
            |document| relative(&self.root, &document.path),
        )
    }

    fn delete(&mut self, id: u64) {
        let root = self.root.clone();
        if let Some(document) = self.documents.get_mut(self.file_index) {
            if let Err(error) = validate_target(&root, &document.path) {
                self.status = error.to_string();
                return;
            }
            match document.delete_record(id) {
                Ok(()) => {
                    self.decision_index = self
                        .decision_index
                        .min(document.records.len().saturating_sub(1));
                    self.status = format!("Deleted decision #{id}");
                }
                Err(error) => self.status = format!("Delete failed: {error}"),
            }
        }
    }

    fn reorder(&mut self, down: bool) {
        if let Some(document) = self.documents.get_mut(self.file_index) {
            if let Err(error) = validate_target(&self.root, &document.path) {
                self.status = error.to_string();
                return;
            }
            if let Some(record) = document.records.get(self.decision_index) {
                let id = record.id;
                let destination = move_index(self.decision_index, document.records.len(), down);
                match document.move_record(id, destination) {
                    Ok(()) => {
                        self.decision_index = destination;
                        self.status = format!("Moved decision #{id}");
                    }
                    Err(error) => self.status = format!("Move failed: {error}"),
                }
            }
        }
    }

    pub(crate) fn search_results(&self, query: &str) -> Vec<(String, u64, String)> {
        let query = query.to_lowercase();
        self.documents
            .iter()
            .flat_map(|document| {
                let file = relative(&self.root, &document.path);
                let query = &query;
                document.records.iter().filter_map(move |record| {
                    let haystack = format!(
                        "{file} {} {} {} {} {} {}",
                        record.id,
                        record.title,
                        record.status,
                        record.decision,
                        record.context,
                        record.consequences
                    )
                    .to_lowercase();
                    haystack
                        .contains(query)
                        .then(|| (file.clone(), record.id, record.title.clone()))
                })
            })
            .collect()
    }

    fn select_identity(&mut self, file: &str, id: u64) {
        if let Some((file_index, decision_index)) =
            self.documents
                .iter()
                .enumerate()
                .find_map(|(index, document)| {
                    (relative(&self.root, &document.path) == file)
                        .then(|| {
                            document
                                .records
                                .iter()
                                .position(|record| record.id == id)
                                .map(|position| (index, position))
                        })
                        .flatten()
                })
        {
            self.file_index = file_index;
            self.decision_index = decision_index;
            self.preview_scroll = 0;
            self.pane = Pane::Decisions;
            self.status = format!("Selected {file}#{id}");
        } else {
            self.status = format!("Decision not found: {file}#{id}. Reload with r.");
        }
    }

    fn open_templates(&mut self) {
        match self
            .repository()
            .and_then(|repository| repository.templates())
        {
            Ok(names) => {
                self.popup = Some(Popup::Templates { names, selected: 0 });
            }
            Err(error) => self.status = error.to_string(),
        }
    }

    fn use_template(&mut self, name: &str) {
        match self
            .repository()
            .and_then(|repository| repository.template(name))
        {
            Ok(mut record) => {
                let path = self.current_document().map_or_else(
                    || self.root.join("DECISIONS.md"),
                    |document| document.path.clone(),
                );
                let next = self.current_document().map_or(Ok(1), Document::next_id);
                match next {
                    Ok(id) => {
                        record.id = id;
                        let mut draft = Draft::new(path, record, true);
                        draft.baseline = Record::new(id);
                        self.draft = Some(draft);
                        self.pane = Pane::Editor;
                        self.editor_scroll = 0;
                        self.status = format!("Template {name} loaded. Ctrl+S saves.");
                    }
                    Err(error) => self.status = error.to_string(),
                }
            }
            Err(error) => self.status = error.to_string(),
        }
    }

    fn open_links(&mut self, adding: bool) {
        let Some(id) = self.current_record().map(|record| record.id) else {
            self.status = "Select a decision first.".into();
            return;
        };
        let targets = if adding {
            self.search_results("")
                .into_iter()
                .filter(|(file, target_id, _)| *file != self.current_file() || *target_id != id)
                .collect()
        } else {
            match self
                .repository()
                .and_then(|repository| repository.relationships(&self.current_file(), id))
            {
                Ok(links) => links
                    .into_iter()
                    .map(|link| (link.file, link.id, link.title))
                    .collect(),
                Err(error) => {
                    self.status = error.to_string();
                    return;
                }
            }
        };
        self.popup = Some(Popup::Links {
            targets,
            selected: 0,
            adding,
        });
    }

    fn add_link(&mut self, file: &str, id: u64) {
        let Some(source_id) = self.current_record().map(|record| record.id) else {
            return;
        };
        match self
            .repository()
            .and_then(|repository| repository.link(&self.current_file(), source_id, file, id))
        {
            Ok(link) => {
                self.edit_selected();
                if let Some(draft) = &mut self.draft {
                    if !draft.context.chars.is_empty() {
                        draft.context.insert("\n\n", false);
                    }
                    draft.context.insert(&link, false);
                    draft.field = 3;
                }
                self.status = "Relationship added to draft. Ctrl+S saves.".into();
            }
            Err(error) => self.status = error.to_string(),
        }
    }

    fn open_history(&mut self) {
        self.history_at(0);
    }

    fn history_at(&mut self, offset: usize) {
        match self
            .repository()
            .and_then(|repository| repository.history_page(&self.current_file(), offset, 200))
        {
            Ok(page) => {
                if offset > 0 && page.entries.is_empty() {
                    self.status = "No older commits remain. History may have changed.".into();
                    return;
                }
                self.popup = Some(Popup::History {
                    entries: page
                        .entries
                        .into_iter()
                        .map(|entry| (entry.revision, entry.summary))
                        .collect(),
                    selected: 0,
                    offset,
                    has_more: page.has_more,
                });
            }
            Err(error) => self.status = error.to_string(),
        }
    }

    /// Render one frame without performing terminal I/O or mutating document contents.
    pub fn draw(&mut self, frame: &mut Frame<'_>) {
        crate::ui::draw(self, frame);
    }
}

const fn number_pane(code: KeyCode) -> Option<Pane> {
    match code {
        KeyCode::Char('1') => Some(Pane::Decisions),
        KeyCode::Char('2') => Some(Pane::Files),
        KeyCode::Char('3') => Some(Pane::Editor),
        KeyCode::Char('4') => Some(Pane::Preview),
        _ => None,
    }
}

fn move_index(current: usize, length: usize, down: bool) -> usize {
    if down {
        current.saturating_add(1).min(length.saturating_sub(1))
    } else {
        current.saturating_sub(1)
    }
}

const fn popup_index(current: usize, length: usize, code: KeyCode) -> usize {
    match code {
        KeyCode::Down | KeyCode::Right | KeyCode::Tab | KeyCode::Char('j') => {
            if current.saturating_add(1) >= length {
                0
            } else {
                current.saturating_add(1)
            }
        }
        KeyCode::Up | KeyCode::Left | KeyCode::BackTab | KeyCode::Char('k') => {
            if current == 0 {
                length.saturating_sub(1)
            } else {
                current.saturating_sub(1)
            }
        }
        _ => current,
    }
}

pub(crate) fn relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .display()
        .to_string()
}

fn validate_target(root: &Path, target: &Path) -> io::Result<()> {
    let parent = target
        .parent()
        .ok_or_else(|| io::Error::other("Missing target directory"))?;
    if !fs::canonicalize(parent)?.starts_with(root)
        || fs::canonicalize(target).is_ok_and(|resolved| !resolved.starts_with(root))
    {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "Decision target resolves outside repository root",
        ));
    }
    Ok(())
}

fn discover(root: &Path, errors: &mut Vec<String>) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(directory) = pending.pop() {
        let entries = match fs::read_dir(&directory) {
            Ok(entries) => entries,
            Err(error) => {
                errors.push(format!("{}: {error}", relative(root, &directory)));
                continue;
            }
        };
        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(error) => {
                    errors.push(error.to_string());
                    continue;
                }
            };
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if name.starts_with('.')
                || matches!(
                    name.as_ref(),
                    "node_modules"
                        | "target"
                        | "vendor"
                        | "dist"
                        | "build"
                        | "__pycache__"
                        | "venv"
                )
            {
                continue;
            }
            let kind = match entry.file_type() {
                Ok(kind) => kind,
                Err(error) => {
                    errors.push(error.to_string());
                    continue;
                }
            };
            if kind.is_dir() {
                pending.push(entry.path());
            } else if kind.is_file()
                && entry.path().extension().is_some_and(|extension| {
                    extension.eq_ignore_ascii_case("md")
                        || extension.eq_ignore_ascii_case("markdown")
                })
            {
                files.push(entry.path());
            }
        }
    }
    files.sort();
    files
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

    fn press(app: &mut App, code: KeyCode) {
        app.handle_event(Event::Key(KeyEvent::new(code, KeyModifiers::NONE)));
    }

    fn control(app: &mut App, character: char) {
        app.handle_event(Event::Key(KeyEvent::new(
            KeyCode::Char(character),
            KeyModifiers::CONTROL,
        )));
    }

    fn begin_first(app: &mut App) {
        press(app, KeyCode::Char('n'));
        press(app, KeyCode::Char('y'));
        press(app, KeyCode::Enter);
    }

    fn seed(root: &Path, title: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let path = root.join("DECISIONS.md");
        let mut record = Record::new(0);
        record.title = title.into();
        Document::new(path.clone()).save_record(&record)?;
        Ok(path)
    }

    #[test]
    fn first_decision_is_saved_only_on_explicit_save() -> Result<(), Box<dyn std::error::Error>> {
        let directory = tempfile::tempdir()?;
        let path = directory.path().join("DECISIONS.md");
        let mut app = App::new(directory.path().to_path_buf())?;
        begin_first(&mut app);
        check!(!path.exists());
        app.handle_event(Event::Paste("Café 界 🙂".into()));
        check!(app.current_dirty());
        control(&mut app, 's');
        check!(app.draft.is_none());
        check_eq!(app.pane, Pane::Decisions);
        check!(app.status.starts_with("Saved decision"));
        let document = Document::load(&path)?;
        check_eq!(
            document.records.first().map(|record| record.title.as_str()),
            Some("Café 界 🙂")
        );
        press(&mut app, KeyCode::Char('q'));
        check!(app.quit);
        Ok(())
    }

    #[test]
    fn cancel_initialization_and_new_draft_leave_no_file() -> Result<(), Box<dyn std::error::Error>>
    {
        let directory = tempfile::tempdir()?;
        let mut app = App::new(directory.path().to_path_buf())?;
        press(&mut app, KeyCode::Char('n'));
        press(&mut app, KeyCode::Esc);
        check!(!directory.path().join("DECISIONS.md").exists());
        begin_first(&mut app);
        app.handle_event(Event::Paste("Draft".into()));
        press(&mut app, KeyCode::Esc);
        check!(!directory.path().join("DECISIONS.md").exists());
        check!(app.draft.is_none());
        Ok(())
    }

    #[test]
    fn continued_typing_after_save_cannot_mutate_closed_draft()
    -> Result<(), Box<dyn std::error::Error>> {
        let directory = tempfile::tempdir()?;
        let path = seed(directory.path(), "Original")?;
        let mut app = App::new(directory.path().to_path_buf())?;
        press(&mut app, KeyCode::Enter);
        app.handle_event(Event::Paste(" edited".into()));
        control(&mut app, 's');
        press(&mut app, KeyCode::Char('X'));
        control(&mut app, 's');
        check!(app.draft.is_none());
        check_eq!(
            Document::load(&path)?
                .records
                .first()
                .map(|record| record.title.as_str()),
            Some("Original edited")
        );
        press(&mut app, KeyCode::Enter);
        check!(app.draft.is_some());
        Ok(())
    }

    #[test]
    fn guard_stay_preserves_draft_and_discard_permits_quit()
    -> Result<(), Box<dyn std::error::Error>> {
        let directory = tempfile::tempdir()?;
        let path = seed(directory.path(), "Original")?;
        let mut app = App::new(directory.path().to_path_buf())?;
        press(&mut app, KeyCode::Enter);
        app.handle_event(Event::Paste(" changed".into()));
        control(&mut app, 'q');
        check!(matches!(app.popup, Some(Popup::Guard { .. })));
        press(&mut app, KeyCode::Esc);
        check!(app.current_dirty());
        check!(!app.quit);
        control(&mut app, 'q');
        press(&mut app, KeyCode::Char('d'));
        check!(app.quit);
        check_eq!(
            Document::load(path)?
                .records
                .first()
                .map(|record| record.title.as_str()),
            Some("Original")
        );
        Ok(())
    }

    #[test]
    fn guard_save_failure_keeps_draft_and_does_not_quit() -> Result<(), Box<dyn std::error::Error>>
    {
        let directory = tempfile::tempdir()?;
        let path = seed(directory.path(), "Original")?;
        let mut app = App::new(directory.path().to_path_buf())?;
        press(&mut app, KeyCode::Enter);
        app.handle_event(Event::Paste(" changed".into()));
        fs::write(&path, "External edit\n")?;
        control(&mut app, 'q');
        press(&mut app, KeyCode::Char('s'));
        check!(!app.quit);
        check!(app.current_dirty());
        check!(app.status.starts_with("Save failed:"));
        check_eq!(fs::read_to_string(path)?, "External edit\n");
        Ok(())
    }

    #[test]
    fn revert_restores_clean_state_and_status_popup_escape_retains_draft()
    -> Result<(), Box<dyn std::error::Error>> {
        let directory = tempfile::tempdir()?;
        seed(directory.path(), "Original")?;
        let mut app = App::new(directory.path().to_path_buf())?;
        press(&mut app, KeyCode::Enter);
        press(&mut app, KeyCode::Char('X'));
        check!(app.current_dirty());
        press(&mut app, KeyCode::Backspace);
        check!(!app.current_dirty());
        press(&mut app, KeyCode::Tab);
        press(&mut app, KeyCode::Enter);
        press(&mut app, KeyCode::Down);
        press(&mut app, KeyCode::Esc);
        check!(app.draft.is_some());
        check!(!app.current_dirty());
        press(&mut app, KeyCode::Enter);
        press(&mut app, KeyCode::Down);
        press(&mut app, KeyCode::Enter);
        check!(app.current_dirty());
        Ok(())
    }

    #[test]
    fn initialization_creation_conflict_preserves_new_draft()
    -> Result<(), Box<dyn std::error::Error>> {
        let directory = tempfile::tempdir()?;
        let mut app = App::new(directory.path().to_path_buf())?;
        begin_first(&mut app);
        app.handle_event(Event::Paste("Draft".into()));
        let path = directory.path().join("DECISIONS.md");
        fs::write(&path, "Do not clobber\n")?;
        control(&mut app, 's');
        check!(app.current_dirty());
        check!(app.status.starts_with("Save failed:"));
        check_eq!(fs::read_to_string(path)?, "Do not clobber\n");
        Ok(())
    }

    #[test]
    fn refresh_reads_disk_and_ignores_generated_and_hidden_directories()
    -> Result<(), Box<dyn std::error::Error>> {
        let directory = tempfile::tempdir()?;
        fs::create_dir(directory.path().join("target"))?;
        fs::create_dir(directory.path().join(".hidden"))?;
        fs::write(directory.path().join("target/ignored.md"), "ignored")?;
        fs::write(directory.path().join(".hidden/ignored.md"), "ignored")?;
        let mut app = App::new(directory.path().to_path_buf())?;
        check!(app.documents.is_empty());
        fs::write(directory.path().join("README.md"), "New file")?;
        fs::write(directory.path().join("OTHER.MARKDOWN"), "Also supported")?;
        press(&mut app, KeyCode::Char('r'));
        check_eq!(app.documents.len(), 2);
        check!(
            app.documents
                .iter()
                .any(|document| document.path.ends_with("README.md"))
        );
        check!(
            app.documents
                .iter()
                .any(|document| document.path.ends_with("OTHER.MARKDOWN"))
        );
        Ok(())
    }

    #[test]
    fn text_buffer_unicode_editing_and_multiline_navigation()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut text = TextBuffer::from_text("é🙂界\nabc");
        text.key(KeyEvent::new(KeyCode::Home, KeyModifiers::NONE), false);
        text.key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE), false);
        check_eq!(text.cursor, 0);
        text.key(KeyEvent::new(KeyCode::Delete, KeyModifiers::NONE), false);
        check_eq!(text.text(), "🙂界\nabc");
        text.key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE), false);
        text.key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE), false);
        check_eq!(text.text(), "界\nabc");
        text.key(KeyEvent::new(KeyCode::End, KeyModifiers::NONE), false);
        text.insert(" café", false);
        check_eq!(text.text(), "界 café\nabc");
        Ok(())
    }

    #[test]
    #[cfg(unix)]
    fn initialization_rejects_external_symlink_destination()
    -> Result<(), Box<dyn std::error::Error>> {
        let directory = tempfile::tempdir()?;
        let outside = tempfile::tempdir()?;
        let target = outside.path().join("outside.md");
        fs::write(&target, "Untouched outside file")?;
        let mut app = App::new(directory.path().to_path_buf())?;
        begin_first(&mut app);
        app.handle_event(Event::Paste("Draft".into()));
        std::os::unix::fs::symlink(&target, directory.path().join("DECISIONS.md"))?;
        control(&mut app, 's');
        check!(app.current_dirty());
        check!(app.status.contains("outside repository root"));
        check_eq!(fs::read_to_string(target)?, "Untouched outside file");
        Ok(())
    }

    #[test]
    fn bracketed_paste_handles_newlines_without_triggering_shortcuts()
    -> Result<(), Box<dyn std::error::Error>> {
        let directory = tempfile::tempdir()?;
        let mut app = App::new(directory.path().to_path_buf())?;
        begin_first(&mut app);
        app.handle_event(Event::Paste("nq\r\nTitle".into()));
        press(&mut app, KeyCode::Tab);
        press(&mut app, KeyCode::Tab);
        app.handle_event(Event::Paste("first\r\nsecond 界".into()));
        check!(!app.quit);
        check_eq!(
            app.draft.as_ref().map(|draft| draft.title.text()),
            Some("nq Title".into())
        );
        check_eq!(
            app.draft.as_ref().map(|draft| draft.decision.text()),
            Some("first\nsecond 界".into())
        );
        Ok(())
    }

    #[test]
    fn search_cancellation_restores_selection_and_escape_does_not_drop_a_guarded_draft()
    -> Result<(), Box<dyn std::error::Error>> {
        let directory = tempfile::tempdir()?;
        seed(directory.path(), "Original")?;
        let mut document = Document::new(directory.path().join("OTHER.md"));
        let mut record = Record::new(0);
        record.title = "Needle".into();
        document.save_record(&record)?;
        let mut app = App::new(directory.path().to_path_buf())?;
        let original = app.current_file();
        press(&mut app, KeyCode::Char('/'));
        app.handle_event(Event::Paste("Needle".into()));
        check_eq!(app.search_results("Needle").len(), 1);
        check_eq!(app.current_file(), original);
        press(&mut app, KeyCode::Esc);
        check_eq!(app.current_file(), original);
        press(&mut app, KeyCode::Enter);
        app.handle_event(Event::Paste(" changed".into()));
        app.pane = Pane::Decisions;
        for shortcut in ['/', 'd', 'J', 't', 'l', 'h'] {
            press(&mut app, KeyCode::Char(shortcut));
            check!(matches!(app.popup, Some(Popup::Guard { .. })));
            press(&mut app, KeyCode::Esc);
            check!(app.current_dirty());
            check_eq!(app.current_file(), original);
        }
        Ok(())
    }

    #[test]
    fn deleting_a_stale_record_preserves_external_edit() -> Result<(), Box<dyn std::error::Error>> {
        let directory = tempfile::tempdir()?;
        let path = seed(directory.path(), "Original")?;
        let mut app = App::new(directory.path().to_path_buf())?;
        press(&mut app, KeyCode::Char('d'));
        let mut document = Document::load(&path)?;
        let mut record = document
            .records
            .first()
            .cloned()
            .ok_or_else(|| io::Error::other("Missing fixture"))?;
        record.context = "Concurrent edit".into();
        document.save_record(&record)?;
        press(&mut app, KeyCode::Char('y'));
        check!(app.status.starts_with("Delete failed:"));
        check_eq!(
            Document::load(&path)?
                .records
                .first()
                .map(|record| record.context.clone()),
            Some("Concurrent edit".into())
        );
        Ok(())
    }

    #[test]
    fn history_scroll_is_bounded_and_an_exhausted_page_cannot_advance()
    -> Result<(), Box<dyn std::error::Error>> {
        let directory = tempfile::tempdir()?;
        let mut app = App::new(directory.path().to_path_buf())?;
        app.viewport = Rect::new(0, 0, 80, 24);
        let text = "History line\n".repeat(50);
        app.popup = Some(Popup::Historical {
            title: "History".into(),
            text: text.clone(),
            scroll: 0,
            entries: vec![("revision".into(), "subject".into())],
            selected: 0,
            offset: 0,
            has_more: false,
        });
        press(&mut app, KeyCode::End);
        let maximum = crate::ui::historical_max_scroll(&text, app.viewport);
        check!(maximum > 0);
        for _ in 0..10 {
            press(&mut app, KeyCode::Down);
        }
        check!(matches!(app.popup, Some(Popup::Historical { scroll, .. }) if scroll == maximum));
        press(&mut app, KeyCode::Home);
        check!(matches!(
            app.popup,
            Some(Popup::Historical { scroll: 0, .. })
        ));
        press(&mut app, KeyCode::Esc);
        press(&mut app, KeyCode::Char('n'));
        check!(
            matches!(&app.popup, Some(Popup::History { entries, offset: 0, .. }) if entries.len() == 1)
        );
        Ok(())
    }
}
