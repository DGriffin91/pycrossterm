//#![feature(proc_macro, specialization, const_fn)]
//rustup override set nightly

use crossterm::event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event, MouseEvent};
use crossterm::{cursor, style, terminal, ExecutableCommand};
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::{wrap_pymodule, PyErr, PyResult};
use std::io::stdout;
use std::time::Duration;

#[pymodule]
fn terminal(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "clear_all")]
    fn clear_all_py() -> PyResult<()> {
        errconv(stdout().execute(terminal::Clear(terminal::ClearType::All)))?;
        Ok(())
    }
    #[pyfn(m, "clear_from_cursor_down")]
    fn clear_from_cursor_down_py() -> PyResult<()> {
        errconv(stdout().execute(terminal::Clear(terminal::ClearType::FromCursorDown)))?;
        Ok(())
    }
    #[pyfn(m, "clear_from_cursor_up")]
    fn clear_from_cursor_up_py() -> PyResult<()> {
        errconv(stdout().execute(terminal::Clear(terminal::ClearType::FromCursorUp)))?;
        Ok(())
    }
    #[pyfn(m, "clear_current_line")]
    fn clear_current_line_py() -> PyResult<()> {
        errconv(stdout().execute(terminal::Clear(terminal::ClearType::CurrentLine)))?;
        Ok(())
    }
    #[pyfn(m, "clear_unitl_new_line")]
    fn clear_unitl_new_line_py() -> PyResult<()> {
        errconv(stdout().execute(terminal::Clear(terminal::ClearType::UntilNewLine)))?;
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
    #[pyfn(m, "size")]
    fn size_py() -> PyResult<(u16, u16)> {
        Ok(errconv(terminal::size())?)
    }
    #[pyfn(m, "enable_raw_mode")]
    fn enable_raw_mode_py() -> PyResult<()> {
        errconv(terminal::enable_raw_mode())?;
        Ok(())
    }
    #[pyfn(m, "disable_raw_mode")]
    fn disable_raw_mode_py() -> PyResult<()> {
        errconv(terminal::disable_raw_mode())?;
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
    fn set_foreground_color_py(color: Color) -> PyResult<()> {
        errconv(stdout().execute(style::SetForegroundColor(convert_color(&color))))?;
        Ok(())
    }
    #[pyfn(m, "set_background_color")]
    fn set_background_color_py(color: Color) -> PyResult<()> {
        errconv(stdout().execute(style::SetBackgroundColor(convert_color(&color))))?;
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

    #[pyfn(m, "available_color_count")]
    fn available_color_count_py() -> PyResult<u16> {
        Ok(style::available_color_count())
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
                r: 0,
                g: 0,
                b: 0,
                color_name: String::from("Reset"),
            },
            background_color: Color {
                r: 0,
                g: 0,
                b: 0,
                color_name: String::from("Black"),
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

    pub fn set_string(mut slf: PyRefMut<Self>, string: String) -> PyResult<Py<Self>> {
        slf.string = string;
        Ok(slf.into())
    }

    pub fn duplicate(slf: PyRefMut<Self>) -> Self {
        slf.clone()
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

#[pyfunction]
fn rgb_py(r: u8, g: u8, b: u8) -> PyResult<Color> {
    Ok(Color {
        r,
        g,
        b,
        color_name: String::new(),
    })
}

#[pymodule]
fn color(py: Python, m: &PyModule) -> PyResult<()> {
    use pyo3::wrap_pyfunction;
    m.add("rgb", wrap_pyfunction!(rgb_py)(py))?;

    let all_colors = [
        "Reset",
        "Black",
        "DarkGrey",
        "Red",
        "DarkRed",
        "Green",
        "DarkGreen",
        "Yellow",
        "DarkYellow",
        "Blue",
        "DarkBlue",
        "Magenta",
        "DarkMagenta",
        "Cyan",
        "DarkCyan",
        "White",
        "Grey",
    ];

    for c in all_colors.iter() {
        let obj = PyCell::new(
            py,
            Color {
                r: 0,
                g: 0,
                b: 0,
                color_name: String::from(*c),
            },
        )
        .unwrap();
        #[allow(unused_must_use)]
        {
            m.add(c, obj);
        }
    }

    Ok(())
}

#[pymodule]
fn event(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("MouseDown", "MouseDown").unwrap();
    m.add("MouseUp", "MouseUp").unwrap();
    m.add("MouseDrag", "MouseDrag").unwrap();
    m.add("ScrollDown", "ScrollDown").unwrap();
    m.add("ScrollUp", "ScrollUp").unwrap();
    m.add("Resize", "Resize").unwrap();

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
                    event: String::from("Resize"),
                    modifiers: String::from(""),
                }),
            },
            Err(_e) => Err(PyErr::new::<exceptions::Exception, _>("Read failed")),
        }
    }

    #[pyfn(m, "poll")]
    fn poll_py(timeout: f64) -> PyResult<bool> {
        Ok(errconv(poll(Duration::from_micros((timeout*1000000.0) as u64)))?)
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
    m.add("Reset", "Reset").unwrap();
    m.add("Bold", "Bold").unwrap();
    m.add("Dim", "Dim").unwrap();
    m.add("Italic", "Italic").unwrap();
    m.add("Underlined", "Underlined").unwrap();
    m.add("SlowBlink", "SlowBlink").unwrap();
    m.add("RapidBlink", "RapidBlink").unwrap();
    m.add("Reverse", "Reverse").unwrap();
    m.add("Hidden", "Hidden").unwrap();
    m.add("CrossedOut", "CrossedOut").unwrap();
    m.add("Fraktur", "Fraktur").unwrap();
    m.add("NoBold", "NoBold").unwrap();
    m.add("NormalIntensity", "NormalIntensity").unwrap();
    m.add("NoItalic", "NoItalic").unwrap();
    m.add("NoUnderline", "NoUnderline").unwrap();
    m.add("NoBlink", "NoBlink").unwrap();
    m.add("NoReverse", "NoReverse").unwrap();
    m.add("NoHidden", "NoHidden").unwrap();
    m.add("NotCrossedOut", "NotCrossedOut").unwrap();
    m.add("Framed", "Framed").unwrap();
    m.add("Encircled", "Encircled").unwrap();
    m.add("OverLined", "OverLined").unwrap();
    m.add("NotFramedOrEncircled", "NotFramedOrEncircled")
        .unwrap();
    m.add("NotOverLined", "NotOverLined").unwrap();
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
    #[pyfn(m, "position")]
    fn position_py() -> PyResult<(u16, u16)> {
        Ok(errconv(cursor::position())?)
    }
    Ok(())
}

#[pymodule]
fn pycrossterm(py: Python, m: &PyModule) -> PyResult<()> {
    use pyo3::wrap_pyfunction;

    // Here for convenience, this is also in the color module
    m.add("rgb", wrap_pyfunction!(rgb_py)(py))?;

    m.add_wrapped(wrap_pymodule!(cursor))?;
    m.add_wrapped(wrap_pymodule!(event))?;
    m.add_wrapped(wrap_pymodule!(style))?;
    m.add_wrapped(wrap_pymodule!(terminal))?;
    m.add_wrapped(wrap_pymodule!(attribute))?;
    m.add_wrapped(wrap_pymodule!(color))?;

    m.add_class::<StyledContent>()?;
    Ok(())
}
