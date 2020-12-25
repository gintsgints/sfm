use crate::core::events::Event;
use crate::core::ui::component::Component;
use crate::core::{events::EventQueue, store::Store};
use std::{
    error::Error,
    io::{stdout, Write},
    process::Command,
};

use app::{
    actions::FileManagerActions, components::root::RootComponent, config::Config,
    reducers::root_reducer, state::AppState,
};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::disable_raw_mode,
    terminal::{enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{backend::CrosstermBackend, Terminal};

pub mod app;
pub mod core;

fn main() -> Result<(), Box<dyn Error>> {
    let cfg = Config::default();
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    let mut event_queue = EventQueue::start_with_config(cfg.core_cfg);

    let mut store = Store::<AppState, FileManagerActions>::new(root_reducer);

    terminal.clear()?;

    let mut root_component = RootComponent::new();
    root_component.on_init(&store);

    store.dispatch(FileManagerActions::App(app::actions::AppAction::FocusLeft));
    loop {
        terminal.draw(|f| root_component.render(f, None))?;

        let state = store.get_state();

        if let Ok(event) = event_queue.pool() {
            if let Event::Tick = event {
                root_component.on_tick();
            } else {
                root_component.handle_event(event, &mut store);
            }
        }

        if let Some(program_desc) = state.child_program {
            event_queue.lock_event_read();
            match Command::new(program_desc.program_name)
                .args(program_desc.args.as_slice())
                .spawn()
            {
                Ok(mut child) => {
                    child.wait().expect("");
                    store.dispatch(FileManagerActions::App(
                        app::actions::AppAction::ChildProgramClosed,
                    ));
                    terminal.clear()?;
                    terminal.draw(|f| root_component.render(f, None))?;
                    event_queue.unlock_event_read();
                }
                Err(_) => {}
            };
        }

        if state.app_exit {
            terminal.clear()?;
            disable_raw_mode()?;
            execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            )?;
            terminal.show_cursor()?;
            break;
        }
    }

    Ok(())
}
