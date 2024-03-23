use crossterm::{
    event::{self, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, widgets::Paragraph, Frame, Terminal};

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

#[derive(Clone)]
struct HelloModel {
    count: i32,
}

#[derive(Clone)]
struct SecondModel {}
impl Model for SecondModel {
    fn apply_event(&self, _event: crossterm::event::Event) -> Box<dyn Model> {
        Box::new(self.clone())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(Paragraph::new("Success! (q to exit)"), frame.size())
    }
}

enum HelloMessage {
    Increment,
    Decrement,
}

trait Model {
    fn apply_event(&self, event: crossterm::event::Event) -> Box<dyn Model>;
    fn draw(&self, frame: &mut Frame);
}

impl Model for HelloModel {
    fn apply_event(&self, event: crossterm::event::Event) -> Box<dyn Model> {
        // handle_event() -> Message
        let message = match event {
            event::Event::Key(KeyEvent { code, .. }) => match code {
                KeyCode::Char('j') => Some(HelloMessage::Decrement),
                KeyCode::Char('k') => Some(HelloMessage::Increment),
                _ => None,
            },
            _ => None,
        };

        // apply_message() -> Model
        let new_model = match message {
            Some(HelloMessage::Increment) => Box::new(Self {
                count: self.count + 1,
            }),
            Some(HelloMessage::Decrement) => Box::new(Self {
                count: self.count - 1,
            }),
            None => Box::new(self.clone()),
        };

        if new_model.count >= 10 {
            Box::new(SecondModel {})
        } else {
            new_model
        }
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(
            Paragraph::new(format!(
                "Counter (get it to ten by pressing k): {:0>2}/10",
                self.count
            )),
            frame.size(),
        )
    }
}

fn main() -> color_eyre::Result<()> {
    setup_panic_hooks()?;

    std::io::stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;

    let mut current_model: Option<Box<dyn Model>> = Some(Box::new(HelloModel { count: 0 }));

    loop {
        let model = current_model.take().unwrap();

        terminal.draw(|frame| model.draw(frame))?;

        let event = event::read()?;

        if let event::Event::Key(crossterm::event::KeyEvent {
            code: KeyCode::Char('q'),
            ..
        }) = event
        {
            break;
        }

        current_model = Some(model.apply_event(event));
    }

    reset_terminal()?;

    Ok(())
}
