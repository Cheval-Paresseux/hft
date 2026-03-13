use super::tui::TelemetryTUI;
use ratatui::{
    widgets::{Widget, Block, List, ListItem, Paragraph},
    text::Line,
    style::Stylize,
    symbols::border,
    layout::{Layout, Constraint, Direction},
};

// ── Const ─────────────────────────────────────────────────────────────────────

pub const MENU_ITEMS: &[&str] = &["Overview", "Logs", "Metrics"];

// ── Widget ────────────────────────────────────────────────────────────────────

impl Widget for &TelemetryTUI {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(20), Constraint::Min(0)])
            .split(area);

        // ── Left menu ────────────────────────────────────────────────────────
        let menu_block = Block::bordered()
            .title(Line::from("Menu"))
            .border_set(border::THICK);

        let items: Vec<ListItem> = MENU_ITEMS
            .iter()
            .enumerate()
            .map(|(i, name)| {
                if i == self.selected {
                    ListItem::new(*name).bold().reversed()
                } else {
                    ListItem::new(*name)
                }
            })
            .collect();

        List::new(items).block(menu_block).render(layout[0], buf);

        // ── Right content ─────────────────────────────────────────────────── 
        let content_block = Block::bordered()
            .title(Line::from(MENU_ITEMS[self.selected]))
            .border_set(border::THICK);

        match self.selected {
            1 => {
                // Logs view: show all lines from all monitored files
                let all_lines: Vec<Line> = self
                    .applications
                    .values()
                    .flat_map(|lines| lines.iter().map(|l| Line::from(l.clone())))
                    .collect();

                Paragraph::new(all_lines)
                    .block(content_block)
                    .render(layout[1], buf);
            }
            _ => {
                // Overview / Metrics placeholder
                let summary: Vec<Line> = self
                    .applications
                    .iter()
                    .map(|(path, logs)| {
                        Line::from(format!("{path}: {} log entries", logs.len()))
                    })
                    .collect();

                Paragraph::new(summary)
                    .block(content_block)
                    .render(layout[1], buf);
            }
        }
    }
}