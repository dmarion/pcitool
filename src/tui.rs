use std::collections::HashMap;
use std::time::Duration;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::widgets::ListState;
use ratatui::{
    prelude::*,
    text::{Line, Span},
    widgets::{Block, Clear, List, ListItem, Paragraph},
};

use crate::pci_device::{self, DeviceSummary};
use crate::tree::{PciDevice, TreeNode};

struct App {
    devices: Vec<DeviceSummary>,
    detail: Option<PciDevice>,
    show_popup: bool,
    status: Option<String>,
    main_list_state: ListState,
    device_list_state: ListState,
    device_hscroll: usize,
}

impl App {
    fn new(devices: Vec<DeviceSummary>, selected: usize, popup: bool) -> Self {
        let mut device_list_state = ListState::default();
        device_list_state.select(Some(selected));
        Self {
            devices,
            detail: None,
            show_popup: popup,
            status: None,
            main_list_state: ListState::default(),
            device_list_state,
            device_hscroll: 0,
        }
    }

    fn load_selected(&mut self) {
        if self.devices.is_empty() {
            self.status = Some("no PCI devices found".to_string());
            self.detail = None;
            return;
        }
        let idx = self
            .device_list_state
            .selected()
            .unwrap_or(0)
            .min(self.devices.len() - 1);
        let summary = &self.devices[idx];
        match pci_device::get_device_tree(summary) {
            Ok(device_data) => {
                self.detail = Some(device_data);
                self.status = None;
                self.main_list_state.select(Some(0));
            }
            Err(e) => {
                self.status = Some(format!("[{}] {e:#}", summary.address));
                self.detail = None;
            }
        }
    }

    fn move_selection(&mut self, delta: isize) {
        if self.devices.is_empty() {
            return;
        }
        let len = self.devices.len() as isize;
        let current = self.device_list_state.selected().unwrap_or(0) as isize;
        let next = (current + delta).clamp(0, len - 1) as usize;
        if Some(next) != self.device_list_state.selected() {
            self.device_list_state.select(Some(next));
            self.load_selected();
        }
    }

    fn set_device_index(&mut self, index: usize) {
        if self.devices.is_empty() {
            return;
        }
        let idx = index.min(self.devices.len() - 1);
        if Some(idx) != self.device_list_state.selected() {
            self.device_list_state.select(Some(idx));
            self.load_selected();
        }
    }

    fn move_main(&mut self, delta: isize) {
        if let Some(detail) = self.detail.as_ref() {
            let count = visible_node_count(&detail.nodes);
            if count == 0 {
                return;
            }
            let len = count as isize;
            let current = self.main_list_state.selected().unwrap_or(0) as isize;
            let next = (current + delta).clamp(0, len - 1) as usize;
            self.main_list_state.select(Some(next));
        }
    }

    fn set_main_index(&mut self, index: usize) {
        if let Some(detail) = self.detail.as_ref() {
            let count = visible_node_count(&detail.nodes);
            if count == 0 {
                return;
            }
            let idx = index.min(count - 1);
            self.main_list_state.select(Some(idx));
        }
    }

    fn toggle_expansion(&mut self) {
        if let Some(detail) = self.detail.as_mut() {
            if let Some(selected) = self.main_list_state.selected() {
                toggle_node_at_visual_index(&mut detail.nodes, selected);
            }
        }
    }

    fn set_expansion(&mut self, expand: bool) {
        if let Some(detail) = self.detail.as_mut() {
            if let Some(selected) = self.main_list_state.selected() {
                set_node_expansion_at_visual_index(&mut detail.nodes, selected, expand);
            }
        }
    }

    fn page_selection(&mut self, delta: isize, page: usize) {
        if page == 0 {
            return;
        }
        self.move_selection(delta * page as isize);
    }

    fn page_main(&mut self, delta: isize, page: usize) {
        if page == 0 {
            return;
        }
        self.move_main(delta * page as isize);
    }

    fn home_selection(&mut self) {
        self.set_device_index(0);
    }

    fn end_selection(&mut self) {
        if !self.devices.is_empty() {
            self.set_device_index(self.devices.len() - 1);
        }
    }

    fn home_main(&mut self) {
        self.set_main_index(0);
    }

    fn end_main(&mut self) {
        if let Some(detail) = self.detail.as_ref() {
            let count = visible_node_count(&detail.nodes);
            if count == 0 {
                return;
            }
            self.set_main_index(count - 1);
        }
    }

    fn scroll_device_horiz(&mut self, delta: isize, max: usize) {
        if delta < 0 {
            let step = (-delta) as usize;
            self.device_hscroll = self.device_hscroll.saturating_sub(step);
        } else if delta > 0 {
            let step = delta as usize;
            self.device_hscroll = (self.device_hscroll + step).min(max);
        }
    }
}

fn update_node_at_visual_index<F>(
    nodes: &mut [TreeNode],
    target_index: usize,
    updater: &mut F,
) -> Option<usize>
where
    F: FnMut(&mut TreeNode),
{
    let mut current_index = 0;
    for node in nodes {
        if current_index == target_index {
            updater(node);
            return None;
        }
        current_index += 1;
        if node.expanded {
            if let Some(remaining) = update_node_at_visual_index(
                &mut node.children,
                target_index - current_index,
                updater,
            ) {
                current_index += remaining;
            } else {
                return None;
            }
        }
    }
    Some(current_index)
}

fn toggle_node_at_visual_index(nodes: &mut [TreeNode], target_index: usize) -> Option<usize> {
    update_node_at_visual_index(nodes, target_index, &mut |node| {
        node.expanded = !node.expanded;
    })
}

fn set_node_expansion_at_visual_index(
    nodes: &mut [TreeNode],
    target_index: usize,
    expand: bool,
) -> Option<usize> {
    update_node_at_visual_index(nodes, target_index, &mut |node| {
        node.expanded = expand;
    })
}

fn visible_node_count(nodes: &[TreeNode]) -> usize {
    let mut count = 0;
    for node in nodes {
        count += 1;
        if node.expanded {
            count += visible_node_count(&node.children);
        }
    }
    count
}

fn flatten_nodes<'a>(
    nodes: &'a [TreeNode],
    depth: usize,
    parent_column: usize,
    current_column: usize,
    lines: &mut Vec<ListItem<'static>>,
) {
    for node in nodes {
        let mut spans = Vec::new();
        if depth > 0 {
            spans.push(Span::raw(" ".repeat(depth * 2)));
        }
        if !node.children.is_empty() {
            if node.expanded {
                spans.push(Span::raw("[-] "));
            } else {
                spans.push(Span::raw("[+] "));
            }
        } else {
            spans.push(Span::raw("    ")); // Fixed prefix width
        }
        let target_column = if node.align_with_parent {
            parent_column
        } else {
            current_column
        };

        let rendered = node.render(target_column, depth);
        spans.extend(rendered.spans.into_iter().map(|s| {
            let style = match s.color {
                crate::tree::TreeColor::Default => Style::default(),
                crate::tree::TreeColor::Red => Style::default().fg(Color::LightRed),
                crate::tree::TreeColor::Green => Style::default().fg(Color::LightGreen),
                crate::tree::TreeColor::Yellow => Style::default().fg(Color::LightYellow),
            };
            Span::styled(s.text, style)
        }));

        lines.push(ListItem::new(Line::from(spans)));

        if node.expanded {
            flatten_nodes(
                &node.children,
                depth + 1,
                target_column,
                node.children_value_indent,
                lines,
            );
        }
    }
}

pub fn run(summaries: Vec<DeviceSummary>, selected: usize, show_list: bool) -> Result<()> {
    if summaries.is_empty() {
        return Ok(());
    }

    let mut app = App::new(summaries, selected, show_list);
    app.load_selected();

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode().ok();
    execute!(terminal.backend_mut(), LeaveAlternateScreen).ok();

    res
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if crossterm::event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('d') | KeyCode::Char('s') => {
                        app.show_popup = !app.show_popup;
                    }
                    KeyCode::Char(']') => {
                        app.move_selection(1);
                    }
                    KeyCode::Char('[') => {
                        app.move_selection(-1);
                    }
                    KeyCode::Up => {
                        if app.show_popup {
                            app.move_selection(-1);
                        } else {
                            app.move_main(-1);
                        }
                    }
                    KeyCode::Down => {
                        if app.show_popup {
                            app.move_selection(1);
                        } else {
                            app.move_main(1);
                        }
                    }
                    KeyCode::PageUp => {
                        let page = current_page_size(terminal, app.show_popup)?;
                        if app.show_popup {
                            app.page_selection(-1, page);
                        } else {
                            app.page_main(-1, page);
                        }
                    }
                    KeyCode::PageDown => {
                        let page = current_page_size(terminal, app.show_popup)?;
                        if app.show_popup {
                            app.page_selection(1, page);
                        } else {
                            app.page_main(1, page);
                        }
                    }
                    KeyCode::Home => {
                        if app.show_popup {
                            app.home_selection();
                        } else {
                            app.home_main();
                        }
                    }
                    KeyCode::End => {
                        if app.show_popup {
                            app.end_selection();
                        } else {
                            app.end_main();
                        }
                    }
                    KeyCode::Left => {
                        if app.show_popup {
                            let max_scroll = popup_max_hscroll(terminal, &app.devices)?;
                            app.scroll_device_horiz(-4, max_scroll);
                        } else {
                            app.set_expansion(false);
                        }
                    }
                    KeyCode::Right => {
                        if app.show_popup {
                            let max_scroll = popup_max_hscroll(terminal, &app.devices)?;
                            app.scroll_device_horiz(4, max_scroll);
                        } else {
                            app.set_expansion(true);
                        }
                    }
                    KeyCode::Enter => {
                        if app.show_popup {
                            app.show_popup = false;
                        } else {
                            app.toggle_expansion();
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn ui(frame: &mut Frame, app: &mut App) {
    let area = frame.area();

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)].as_ref())
        .split(area);

    let main_area = vertical[0];
    let status_area = vertical[1];

    let items = build_main_list(app);
    let list = List::new(items).highlight_style(Style::default().bg(Color::Blue).fg(Color::White));
    frame.render_stateful_widget(list, main_area, &mut app.main_list_state);

    if app.show_popup {
        let area = popup_rect(area);
        frame.render_widget(Clear, area);
        frame.render_widget(
            Block::default().style(Style::default().bg(Color::Rgb(32, 32, 32))),
            area,
        );
        let hscroll = app.device_hscroll;
        let prefixes = build_device_tree_prefix_spans(&app.devices);
        let items: Vec<ListItem> = app
            .devices
            .iter()
            .zip(prefixes.into_iter())
            .map(|(d, mut prefix_spans)| {
                prefix_spans.push(Span::styled(
                    d.address.clone(),
                    Style::default().fg(Color::LightYellow),
                ));
                prefix_spans.push(Span::raw(format!(
                    " {:04x}:{:04x}{}",
                    d.vendor_id, d.device_id, d.name_suffix
                )));
                let spans = trim_spans(prefix_spans, hscroll);
                ListItem::new(Line::from(spans))
            })
            .collect();
        let list = List::new(items)
            .block(Block::default().title(Span::styled(
                "Root Complex",
                Style::default().fg(Color::LightYellow),
            )))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));
        frame.render_stateful_widget(list, area, &mut app.device_list_state);
    }

    if let Some(status) = &app.status {
        let para = Paragraph::new(status.clone()).style(Style::default().fg(Color::Yellow));
        frame.render_widget(para, status_area);
    } else {
        let shortcuts = Paragraph::new(concat!(
            "q: quit  ",
            "d: select device  ",
            "[/]: next/prev device  ",
            "PgUp/PgDn/Home/End: page/top/bottom  ",
            "←/→: collapse/expand  ",
            "↑/↓: navigate"
        ))
        .style(Style::default().fg(Color::Yellow).bg(Color::Blue));
        frame.render_widget(shortcuts, status_area);
    }
}

fn build_main_list(app: &App) -> Vec<ListItem<'static>> {
    let mut items = Vec::new();

    if let Some(detail) = &app.detail {
        flatten_nodes(
            &detail.nodes,
            0,
            detail.children_value_indent,
            detail.children_value_indent,
            &mut items,
        );
    } else if let Some(status) = &app.status {
        items.push(ListItem::new(status.clone()));
    }

    items
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    let vertical = popup_layout[1];
    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(vertical);
    horizontal[1]
}

fn popup_rect(area: Rect) -> Rect {
    let mut rect = centered_rect(80, 80, area);
    rect.y = rect.y.saturating_add(3);
    let width = area.width.saturating_sub(2);
    if width > 0 {
        rect.x = area.x + 1;
        rect.width = width;
    }
    rect
}

fn build_device_tree_prefix_spans(devices: &[DeviceSummary]) -> Vec<Vec<Span<'static>>> {
    const ROOT_INDENT: &str = "  ";
    let labels: Vec<Option<String>> = devices.iter().map(format_link_info).collect();
    let max_label_len = labels
        .iter()
        .filter_map(|label| label.as_ref().map(|s| s.len()))
        .max()
        .unwrap_or(0);
    let segment_width = if max_label_len > 0 {
        max_label_len + 2
    } else {
        2
    };
    let indent_width = segment_width + 2;
    let mut last_child: HashMap<String, String> = HashMap::new();
    for device in devices {
        let chain = &device.tree_chain;
        if chain.is_empty() {
            continue;
        }
        let parent_key = chain[..chain.len() - 1].join("/");
        if let Some(name) = chain.last() {
            last_child.insert(parent_key, name.clone());
        }
    }

    let mut prefixes = Vec::with_capacity(devices.len());
    for (device, label) in devices.iter().zip(labels.into_iter()) {
        let chain = &device.tree_chain;
        if chain.is_empty() {
            prefixes.push(Vec::new());
            continue;
        }
        let depth = chain.len() - 1;
        let mut prefix = vec![Span::raw(ROOT_INDENT)];
        for idx in 0..depth {
            let ancestor_parent_key = chain[..idx].join("/");
            let ancestor_name = &chain[idx];
            let is_last_ancestor = last_child
                .get(&ancestor_parent_key)
                .map(|last| last == ancestor_name)
                .unwrap_or(true);
            if is_last_ancestor {
                prefix.push(Span::raw(" ".repeat(indent_width)));
            } else {
                prefix.push(Span::styled("│", Style::default().fg(Color::Blue)));
                if indent_width > 1 {
                    prefix.push(Span::raw(" ".repeat(indent_width - 1)));
                }
            }
        }

        let parent_key = chain[..depth].join("/");
        let current_name = chain.last().expect("chain is not empty");
        let is_last = last_child
            .get(&parent_key)
            .map(|last| last == current_name)
            .unwrap_or(true);
        let connector = if is_last { "└" } else { "├" };
        prefix.push(Span::styled(connector, Style::default().fg(Color::Blue)));

        if let Some(link) = label {
            let link_len = link.len();
            prefix.push(Span::styled("─", Style::default().fg(Color::Blue)));
            prefix.push(Span::styled(link, Style::default().fg(Color::Blue)));
            let used = link_len + 1;
            let pad = segment_width.saturating_sub(used);
            if pad > 0 {
                prefix.push(Span::styled(
                    "─".repeat(pad),
                    Style::default().fg(Color::Blue),
                ));
            }
            prefix.push(Span::raw(" "));
        } else {
            prefix.push(Span::styled(
                "─".repeat(segment_width),
                Style::default().fg(Color::Blue),
            ));
            prefix.push(Span::raw(" "));
        }
        prefixes.push(prefix);
    }
    prefixes
}

fn popup_max_hscroll(
    terminal: &Terminal<CrosstermBackend<std::io::Stdout>>,
    devices: &[DeviceSummary],
) -> Result<usize> {
    let area = popup_rect(terminal.size()?.into());
    let prefixes = build_device_tree_prefix_spans(devices);
    let max_len = devices
        .iter()
        .zip(prefixes.into_iter())
        .map(|(d, mut spans)| {
            spans.push(Span::raw(d.address.clone()));
            spans.push(Span::raw(format!(
                " {:04x}:{:04x}{}",
                d.vendor_id, d.device_id, d.name_suffix
            )));
            spans_len(&spans)
        })
        .max()
        .unwrap_or(0);
    let width = area.width as usize;
    Ok(max_len.saturating_sub(width))
}

fn spans_len(spans: &[Span<'static>]) -> usize {
    spans.iter().map(|span| span.content.chars().count()).sum()
}

fn trim_spans(spans: Vec<Span<'static>>, mut offset: usize) -> Vec<Span<'static>> {
    if offset == 0 {
        return spans;
    }
    let mut trimmed = Vec::new();
    for span in spans {
        if offset == 0 {
            trimmed.push(span);
            continue;
        }
        let content = span.content;
        let len = content.chars().count();
        if offset >= len {
            offset -= len;
            continue;
        }
        let start_idx = content
            .char_indices()
            .nth(offset)
            .map(|(idx, _)| idx)
            .unwrap_or(content.len());
        let sliced = content[start_idx..].to_string();
        trimmed.push(Span::styled(sliced, span.style));
        offset = 0;
    }
    trimmed
}

fn format_link_info(device: &DeviceSummary) -> Option<String> {
    let speed = device.current_link_speed?;
    let width = device.current_link_width?;
    let generation = link_gen(speed)?;
    Some(format!("Gen{}x{}", generation, width))
}

fn link_gen(code: u8) -> Option<u8> {
    match code {
        1 => Some(1),
        2 => Some(2),
        3 => Some(3),
        4 => Some(4),
        5 => Some(5),
        6 => Some(6),
        7 => Some(7),
        _ => None,
    }
}

fn current_page_size(
    terminal: &Terminal<CrosstermBackend<std::io::Stdout>>,
    show_popup: bool,
) -> Result<usize> {
    let area = terminal.size()?;
    let height = if show_popup {
        let popup = popup_rect(area.into());
        popup.height.saturating_sub(1)
    } else {
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)].as_ref())
            .split(area.into());
        vertical[0].height
    };
    Ok(height.max(1) as usize)
}
