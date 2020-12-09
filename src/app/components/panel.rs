use tui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
    Frame,
};

use crate::{
    app::{
        actions::FileManagerActions,
        state::{AppState, PanelState},
    },
    core::{
        events::Event,
        ui::{component::Component, component_base::ComponentBase},
    },
};

use super::tab::{TabComponent, TabComponentProps};

#[derive(Clone, Default, Debug)]
pub struct PanelComponentProps {
    tabs: Vec<String>,
    current_tab: usize,
}

pub struct PanelComponent {
    base: ComponentBase<PanelComponentProps, ()>,
    tab: TabComponent,
}

impl PanelComponent {
    pub fn new(props: PanelComponentProps, tab_props: TabComponentProps) -> Self {
        PanelComponent {
            base: ComponentBase::new(Some(props), None),
            tab: TabComponent::with_props(tab_props),
        }
    }

    pub fn empty() -> Self {
        PanelComponent {
            base: ComponentBase::new(None, None),
            tab: TabComponent::empty(),
        }
    }
}

impl From<PanelState> for PanelComponent {
    fn from(panel_state: PanelState) -> Self {
        let tabs = panel_state
            .tabs
            .iter()
            .map(|tab| tab.name.clone())
            .collect();
        let panel_props = PanelComponentProps {
            tabs,
            current_tab: panel_state.current_tab,
        };

        PanelComponent::new(
            panel_props,
            panel_state.tabs[panel_state.current_tab].clone(),
        )
    }
}

impl Component<Event, AppState, FileManagerActions> for PanelComponent {
    fn render<TBackend: Backend>(&self, frame: &mut Frame<TBackend>, area: Option<Rect>) {
        let props = self.base.get_props().unwrap();
        if props.tabs.len() > 1 {
            let tabs_items: Vec<Spans> = props
                .tabs
                .iter()
                .enumerate()
                .map(|(idx, val)| {
                    if idx == props.current_tab {
                        Spans::from(vec![Span::styled(val, Style::default().fg(Color::Blue))])
                    } else {
                        Spans::from(vec![Span::styled(val, Style::default())])
                    }
                })
                .collect();

            let tabs = Tabs::new(tabs_items).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(props.tabs[props.current_tab].clone()),
            );

            let layout = Layout::default()
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
                .split(area.unwrap());

            frame.render_widget(tabs, layout[0]);
            self.tab.render(frame, Some(layout[1]));
        } else {
            self.tab.render(frame, area);
        }
    }
}

pub fn map_global_state_to_panel_props(gloal_state: PanelState) -> PanelComponentProps {
    let tabs = gloal_state
        .tabs
        .iter()
        .map(|tab| tab.name.clone())
        .collect();
    PanelComponentProps {
        tabs,
        current_tab: gloal_state.current_tab,
    }
}
