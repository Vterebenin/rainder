use std::time::Duration;
use crossterm::{Result, event};

pub fn user_input() -> Result<bool> {
    if event::poll(Duration::from_millis(50))? {
        match event::read()? {
            event::Event::Key(keyevent) => {
                if keyevent
                    == event::KeyEvent::new(event::KeyCode::Char('q'), event::KeyModifiers::NONE)
                    || keyevent
                        == event::KeyEvent::new(event::KeyCode::Esc, event::KeyModifiers::NONE)
                {
                    return Ok(false);
                }
            }
            // event::Event::Resize(w, h) => {
            //     clear(stdout)?;
            //     *rain = Rain::new(create_color, w, h, user_settings);
            // }
            _ => {}
        }
    }
    Ok(true)
}
