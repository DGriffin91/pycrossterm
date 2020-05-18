//#![feature(proc_macro, specialization, const_fn)]
//rustup override set nightly

use crossterm::event::{read, DisableMouseCapture, EnableMouseCapture, Event, MouseEvent};
use crossterm::{cursor, style, terminal, ExecutableCommand};
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::{wrap_pymodule, PyErr, PyResult};
use std::io::stdout;

#[pymodule]
fn terminal(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "clear")]
    fn clear_py(kind: String) -> PyResult<()> {
        let kind = match &kind.to_string()[..] {
            "All" => terminal::ClearType::All,
            "FromCursorDown" => terminal::ClearType::FromCursorDown,
            "FromCursorUp" => terminal::ClearType::FromCursorUp,
            "CurrentLine" => terminal::ClearType::CurrentLine,
            "UntilNewLine" => terminal::ClearType::UntilNewLine,
            _ => terminal::ClearType::All,
        };
        errconv(stdout().execute(terminal::Clear(kind)))?;
        Ok(())
    }
    #[pyfn(m, "scroll_up")]
    fn scroll_up_py(n: u16) -> PyResult<()> {
        errconv(stdout().execute(terminal::ScrollUp(n)))?;
        Ok(())
    }
    #[pyfn(m, "scroll_down")]
    fn scroll_down_py(n: u16) -> PyResult<()> {
        errconv(stdout().execute(terminal::ScrollDown(n)))?;
        Ok(())
    }
    #[pyfn(m, "set_size")]
    fn set_size_py(columns: u16, rows: u16) -> PyResult<()> {
        errconv(stdout().execute(terminal::SetSize(columns, rows)))?;
        Ok(())
    }
    #[pyfn(m, "enter_alternate_screen")]
    fn enter_alternate_screen_py() -> PyResult<()> {
        errconv(stdout().execute(terminal::EnterAlternateScreen))?;
        Ok(())
    }
    #[pyfn(m, "leave_alternate_screen")]
    fn leave_alternate_screen_py() -> PyResult<()> {
        errconv(stdout().execute(terminal::LeaveAlternateScreen))?;
        Ok(())
    }
    Ok(())
}

fn errconv<T>(error: Result<T, crossterm::ErrorKind>) -> PyResult<T> {
    match error {
        Err(e) => Err(PyErr::new::<exceptions::Exception, _>(format!("{}", e))),
        Ok(e) => Ok(e),
    }
}

fn attribute_from_string(s: String) -> Option<style::Attribute> {
    // Couldn't figure out how to use EnumFromStr on style::Attribute
    Some(match &s.to_string()[..] {
        "Reset" => style::Attribute::Reset,
        "Bold" => style::Attribute::Bold,
        "Dim" => style::Attribute::Dim,
        "Italic" => style::Attribute::Italic,
        "Underlined" => style::Attribute::Underlined,
        "SlowBlink" => style::Attribute::SlowBlink,
        "RapidBlink" => style::Attribute::RapidBlink,
        "Reverse" => style::Attribute::Reverse,
        "Hidden" => style::Attribute::Hidden,
        "CrossedOut" => style::Attribute::CrossedOut,
        "Fraktur" => style::Attribute::Fraktur,
        "NoBold" => style::Attribute::NoBold,
        "NormalIntensity" => style::Attribute::NormalIntensity,
        "NoItalic" => style::Attribute::NoItalic,
        "NoUnderline" => style::Attribute::NoUnderline,
        "NoBlink" => style::Attribute::NoBlink,
        "NoReverse" => style::Attribute::NoReverse,
        "NoHidden" => style::Attribute::NoHidden,
        "NotCrossedOut" => style::Attribute::NotCrossedOut,
        "Framed" => style::Attribute::Framed,
        "Encircled" => style::Attribute::Encircled,
        "OverLined" => style::Attribute::OverLined,
        "NotFramedOrEncircled" => style::Attribute::NotFramedOrEncircled,
        "NotOverLined" => style::Attribute::NotOverLined,
        _ => return None,
    })
}

#[pymodule]
fn style(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "set_foreground_color")]
    fn set_foreground_color_py(r: u8, g: u8, b: u8) -> PyResult<()> {
        errconv(stdout().execute(style::SetForegroundColor(style::Color::Rgb { r, g, b })))?;
        Ok(())
    }
    #[pyfn(m, "set_background_color")]
    fn set_background_color_py(r: u8, g: u8, b: u8) -> PyResult<()> {
        errconv(stdout().execute(style::SetBackgroundColor(style::Color::Rgb { r, g, b })))?;
        Ok(())
    }
    #[pyfn(m, "reset_color")]
    fn reset_color_py() -> PyResult<()> {
        errconv(stdout().execute(style::ResetColor))?;
        Ok(())
    }
    #[pyfn(m, "set_attribute")]
    fn set_attribute_py(kind: String) -> PyResult<()> {
        // Couldn't figure out how to use EnumFromStr on style::Attribute
        match attribute_from_string(kind) {
            Some(attrib) => errconv(stdout().execute(style::SetAttribute(attrib)))?,
            None => return Ok(()),
        };
        Ok(())
    }
    #[pyfn(m, "new")]
    fn new_py(string: String) -> PyResult<StyledContent> {
        Ok(StyledContent {
            string,
            foreground_color: Color {
                r: 255,
                g: 255,
                b: 255,
                color_name: String::new(),
            },
            background_color: Color {
                r: 0,
                g: 0,
                b: 0,
                color_name: String::new(),
            },
            attributes: Vec::new(),
        })
    }

    #[pyfn(m, "color")]
    fn color_py(r: u8, g: u8, b: u8) -> PyResult<Color> {
        Ok(Color {
            r,
            g,
            b,
            color_name: String::new(),
        })
    }
    #[pyfn(m, "named_color")]
    fn named_color_py(s: String) -> PyResult<Color> {
        Ok(Color {
            r: 0,
            g: 0,
            b: 0,
            color_name: s,
        })
    }
    Ok(())
}

#[pyclass]
#[derive(Debug, Clone, Default)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    color_name: String,
}

#[pyclass]
//#[pyclass(extends=PyDict)]
#[derive(Debug, Clone, Default)]
struct StyledContent {
    #[pyo3(get, set)]
    string: String,
    #[pyo3(get, set)]
    foreground_color: Color,
    #[pyo3(get, set)]
    background_color: Color,
    attributes: Vec<String>,
}

#[pymethods]
impl StyledContent {
    #[new]
    fn new(string: String) -> Self {
        StyledContent {
            string,
            foreground_color: Color {
                r: 255,
                g: 255,
                b: 255,
                color_name: String::new(),
            },
            background_color: Color {
                r: 255,
                g: 255,
                b: 255,
                color_name: String::new(),
            },
            attributes: Vec::new(),
        }
    }

    pub fn color(mut slf: PyRefMut<Self>, color: Color) -> PyResult<Py<Self>> {
        slf.foreground_color = color;
        Ok(slf.into())
    }

    pub fn on(mut slf: PyRefMut<Self>, color: Color) -> PyResult<Py<Self>> {
        slf.background_color = color;
        Ok(slf.into())
    }

    pub fn content(slf: PyRef<Self>) -> PyResult<String> {
        Ok(slf.string.clone())
    }

    pub fn attribute(mut slf: PyRefMut<Self>, attrib: String) -> PyResult<Py<Self>> {
        slf.attributes.push(attrib);
        Ok(slf.into())
    }

    pub fn print(slf: PyRefMut<Self>) -> PyResult<()> {
        let mut styled = style::style(&slf.string[..]);
        styled = styled.with(convert_color(&slf.foreground_color));
        styled = styled.on(convert_color(&slf.background_color));
        for attr in slf.attributes.iter() {
            match attribute_from_string(attr.into()) {
                Some(a) => styled = styled.attribute(a),
                None => (),
            }
        }
        println!("{}", styled);
        Ok(())
    }
}

fn convert_color(color: &Color) -> style::Color {
    if color.color_name == "" {
        return style::Color::Rgb {
            r: color.r,
            g: color.g,
            b: color.b,
        };
    } else {
        match (&color.color_name[..]).parse() {
            Err(_e) => return style::Color::White,
            Ok(c) => return c,
        }
    }
}

#[pymodule]
fn event(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyclass]
    struct PyEvent {
        #[pyo3(get, set)]
        x: u16,
        #[pyo3(get, set)]
        y: u16,
        #[pyo3(get, set)]
        key: String,
        #[pyo3(get, set)]
        event: String,
        #[pyo3(get, set)]
        modifiers: String,
    }
    #[pyfn(m, "read")]
    fn read_py() -> PyResult<PyEvent> {
        match read() {
            Ok(e) => match e {
                Event::Key(event) => Ok(PyEvent {
                    x: 0,
                    y: 0,
                    key: format!("{:?}", event.code),
                    event: String::from("Key"),
                    modifiers: String::from(""),
                }),
                Event::Mouse(mouse_event) => Ok({
                    match mouse_event {
                        MouseEvent::Down(button, x, y, m) => PyEvent {
                            x,
                            y,
                            key: format!("{:?}", button),
                            event: String::from("MouseDown"),
                            modifiers: format!("{:?}", m),
                        },
                        MouseEvent::Up(button, x, y, m) => PyEvent {
                            x,
                            y,
                            key: format!("{:?}", button),
                            event: String::from("MouseUp"),
                            modifiers: format!("{:?}", m),
                        },
                        MouseEvent::Drag(button, x, y, m) => PyEvent {
                            x,
                            y,
                            key: format!("{:?}", button),
                            event: String::from("MouseDrag"),
                            modifiers: format!("{:?}", m),
                        },
                        MouseEvent::ScrollDown(x, y, m) => PyEvent {
                            x,
                            y,
                            key: String::from(""),
                            event: String::from("ScrollDown"),
                            modifiers: format!("{:?}", m),
                        },
                        MouseEvent::ScrollUp(x, y, m) => PyEvent {
                            x,
                            y,
                            key: String::from(""),
                            event: String::from("ScrollUp"),
                            modifiers: format!("{:?}", m),
                        },
                    }
                }),
                Event::Resize(x, y) => Ok(PyEvent {
                    x,
                    y,
                    key: String::from(""),
                    event: String::from("Key"),
                    modifiers: String::from(""),
                }),
            },
            Err(_e) => Err(PyErr::new::<exceptions::Exception, _>("Read failed")),
        }
    }

    #[pyfn(m, "enable_mouse_capture")]
    fn enable_mouse_capture_py() -> PyResult<()> {
        errconv(stdout().execute(EnableMouseCapture))?;
        Ok(())
    }

    #[pyfn(m, "disable_mouse_capture")]
    fn disable_mouse_capture_py() -> PyResult<()> {
        errconv(stdout().execute(DisableMouseCapture))?;
        Ok(())
    }

    Ok(())
}

#[pymodule]
fn attribute(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "Reset")]
    fn reset_py() -> PyResult<String> {
        Ok(String::from("Reset"))
    }
    #[pyfn(m, "Bold")]
    fn bold_py() -> PyResult<String> {
        Ok(String::from("Bold"))
    }
    #[pyfn(m, "Dim")]
    fn dim_py() -> PyResult<String> {
        Ok(String::from("Dim"))
    }
    #[pyfn(m, "Italic")]
    fn italic_py() -> PyResult<String> {
        Ok(String::from("Italic"))
    }
    #[pyfn(m, "Underlined")]
    fn underlined_py() -> PyResult<String> {
        Ok(String::from("Underlined"))
    }
    #[pyfn(m, "SlowBlink")]
    fn slowblink_py() -> PyResult<String> {
        Ok(String::from("SlowBlink"))
    }
    #[pyfn(m, "RapidBlink")]
    fn rapidblink_py() -> PyResult<String> {
        Ok(String::from("RapidBlink"))
    }
    #[pyfn(m, "Reverse")]
    fn reverse_py() -> PyResult<String> {
        Ok(String::from("Reverse"))
    }
    #[pyfn(m, "Hidden")]
    fn hidden_py() -> PyResult<String> {
        Ok(String::from("Hidden"))
    }
    #[pyfn(m, "CrossedOut")]
    fn crossedout_py() -> PyResult<String> {
        Ok(String::from("CrossedOut"))
    }
    #[pyfn(m, "Fraktur")]
    fn fraktur_py() -> PyResult<String> {
        Ok(String::from("Fraktur"))
    }
    #[pyfn(m, "NoBold")]
    fn nobold_py() -> PyResult<String> {
        Ok(String::from("NoBold"))
    }
    #[pyfn(m, "NormalIntensity")]
    fn normalintensity_py() -> PyResult<String> {
        Ok(String::from("NormalIntensity"))
    }
    #[pyfn(m, "NoItalic")]
    fn noitalic_py() -> PyResult<String> {
        Ok(String::from("NoItalic"))
    }
    #[pyfn(m, "NoUnderline")]
    fn nounderline_py() -> PyResult<String> {
        Ok(String::from("NoUnderline"))
    }
    #[pyfn(m, "NoBlink")]
    fn noblink_py() -> PyResult<String> {
        Ok(String::from("NoBlink"))
    }
    #[pyfn(m, "NoReverse")]
    fn noreverse_py() -> PyResult<String> {
        Ok(String::from("NoReverse"))
    }
    #[pyfn(m, "NoHidden")]
    fn nohidden_py() -> PyResult<String> {
        Ok(String::from("NoHidden"))
    }
    #[pyfn(m, "NotCrossedOut")]
    fn notcrossedout_py() -> PyResult<String> {
        Ok(String::from("NotCrossedOut"))
    }
    #[pyfn(m, "Framed")]
    fn framed_py() -> PyResult<String> {
        Ok(String::from("Framed"))
    }
    #[pyfn(m, "Encircled")]
    fn encircled_py() -> PyResult<String> {
        Ok(String::from("Encircled"))
    }
    #[pyfn(m, "OverLined")]
    fn overlined_py() -> PyResult<String> {
        Ok(String::from("OverLined"))
    }
    #[pyfn(m, "NotFramedOrEncir")]
    fn notframedorencir_py() -> PyResult<String> {
        Ok(String::from("NotFramedOrEncir"))
    }
    #[pyfn(m, "NotOverLined")]
    fn notoverlined_py() -> PyResult<String> {
        Ok(String::from("NotOverLined"))
    }
    Ok(())
}

#[pymodule]
fn cursor(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "hide")]
    fn hide_py() -> PyResult<()> {
        errconv(stdout().execute(cursor::Hide))?;
        Ok(())
    }

    #[pyfn(m, "show")]
    fn show_py() -> PyResult<()> {
        errconv(stdout().execute(cursor::Show))?;
        Ok(())
    }

    #[pyfn(m, "enable_blinking")]
    fn enable_blinking_py() -> PyResult<()> {
        errconv(stdout().execute(cursor::EnableBlinking))?;
        Ok(())
    }

    #[pyfn(m, "disable_blinking")]
    fn disable_blinking_py() -> PyResult<()> {
        errconv(stdout().execute(cursor::DisableBlinking))?;
        Ok(())
    }

    #[pyfn(m, "save_position")]
    fn save_position_py() -> PyResult<()> {
        errconv(stdout().execute(cursor::SavePosition))?;
        Ok(())
    }

    #[pyfn(m, "restore_position")]
    fn restore_position_py() -> PyResult<()> {
        errconv(stdout().execute(cursor::RestorePosition))?;
        Ok(())
    }

    #[pyfn(m, "move_up")]
    fn move_up_py(n: u16) -> PyResult<()> {
        errconv(stdout().execute(cursor::MoveUp(n)))?;
        Ok(())
    }

    #[pyfn(m, "move_down")]
    fn move_down_py(n: u16) -> PyResult<()> {
        errconv(stdout().execute(cursor::MoveDown(n)))?;
        Ok(())
    }

    #[pyfn(m, "move_left")]
    fn move_left_py(n: u16) -> PyResult<()> {
        errconv(stdout().execute(cursor::MoveLeft(n)))?;
        Ok(())
    }

    #[pyfn(m, "move_right")]
    fn move_right_py(n: u16) -> PyResult<()> {
        errconv(stdout().execute(cursor::MoveRight(n)))?;
        Ok(())
    }

    #[pyfn(m, "move_to")]
    fn move_to_py(x: u16, y: u16) -> PyResult<()> {
        errconv(stdout().execute(cursor::MoveTo(x, y)))?;
        Ok(())
    }

    #[pyfn(m, "move_to_column")]
    fn move_to_column_py(n: u16) -> PyResult<()> {
        errconv(stdout().execute(cursor::MoveToColumn(n)))?;
        Ok(())
    }

    #[pyfn(m, "move_to_next_line")]
    fn move_to_next_line_py(n: u16) -> PyResult<()> {
        errconv(stdout().execute(cursor::MoveToNextLine(n)))?;
        Ok(())
    }

    #[pyfn(m, "move_to_previous_line")]
    fn move_to_previous_line_py(n: u16) -> PyResult<()> {
        errconv(stdout().execute(cursor::MoveToPreviousLine(n)))?;
        Ok(())
    }
    Ok(())
}

#[pymodule]
fn pycrossterm(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(cursor))?;
    m.add_wrapped(wrap_pymodule!(event))?;
    m.add_wrapped(wrap_pymodule!(style))?;
    m.add_wrapped(wrap_pymodule!(terminal))?;
    m.add_wrapped(wrap_pymodule!(attribute))?;
    m.add_class::<StyledContent>()?;
    Ok(())
}
