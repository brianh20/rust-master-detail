use tui::{
    layout::{Alignment, Constraint},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table,
    },
};

use crate::database;

pub fn render_home<'a>() -> Paragraph<'a> {
    let home = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Welcome")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("to")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "person-CLI",
            Style::default().fg(Color::LightBlue),
        )]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Press 'p' to access people, 'a' to add random new person and 'd' to delete the currently selected person.")]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Home")
            .border_type(BorderType::Plain),
    );
    home
}

pub fn render_people<'a>(person_list_state: &ListState) -> (Option<List<'a>>, Option<Table<'a>>) {
    let people = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("People")
        .border_type(BorderType::Plain);

    let person_list = database::read_db().expect("can fetch person list");
    let items: Vec<_> = person_list
        .iter()
        .map(|person| {
            ListItem::new(Spans::from(vec![Span::styled(
                person.name.clone(),
                Style::default(),
            )]))
        })
        .collect();

    if person_list.len() == 0 {
        return (None, None);
    }

    let selected_person = person_list
        .get(
            person_list_state
                .selected()
                .expect("there is always a selected person"),
        )
        .expect("exists")
        .clone();

    let list = List::new(items).block(people).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    let person_detail = Table::new(vec![Row::new(vec![
        Cell::from(Span::raw(selected_person.id.to_string())),
        Cell::from(Span::raw(selected_person.name)),
        Cell::from(Span::raw(selected_person.category)),
        Cell::from(Span::raw(selected_person.age.to_string())),
        Cell::from(Span::raw(selected_person.created_at.to_string())),
    ])])
    .header(Row::new(vec![
        Cell::from(Span::styled(
            "ID",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Name",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Category",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Age",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Created At",
            Style::default().add_modifier(Modifier::BOLD),
        )),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Detail")
            .border_type(BorderType::Plain),
    )
    .widths(&[
        Constraint::Percentage(5),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(5),
        Constraint::Percentage(20),
    ]);

    (Some(list), Some(person_detail))
}
