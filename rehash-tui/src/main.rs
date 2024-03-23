use crossterm::{
    event,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, style::Stylize as _, widgets::Paragraph, Terminal};

fn reset_terminal() -> color_eyre::Result<()> {
    std::io::stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}

fn setup_panic_hooks() -> color_eyre::Result<()> {
    let (panic_hook, eyre_hook) = color_eyre::config::HookBuilder::default().into_hooks();

    let panic_hook = panic_hook.into_panic_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        reset_terminal().unwrap();
        panic_hook(panic_info);
    }));

    let eyre_hook = eyre_hook.into_eyre_hook();
    color_eyre::eyre::set_hook(Box::new(
        move |error: &(dyn std::error::Error + 'static)| {
            reset_terminal().unwrap();
            eyre_hook(error)
        },
    ))?;

    Ok(())
}

fn main() -> color_eyre::Result<()> {
    setup_panic_hooks()?;

    std::io::stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;

    loop {
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new("Hello World! ('q' to quit)")
                    .white()
                    .on_blue(),
                area,
            )
        })?;

        let event = event::read()?;
        match event {
            event::Event::Key(_) => break,
            event::Event::Resize(_, _) => todo!(),
            _ => (),
        };
    }

    reset_terminal()?;

    Ok(())
}
