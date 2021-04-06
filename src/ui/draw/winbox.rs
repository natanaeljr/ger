use crate::ui::rect::Rect;
use crate::ui::winbox::WinBox;
use crossterm::{cursor, queue, style};

/// Draw the Window Box widget in the Rect space.
///
/// Basically, it just draws window borders.
///
/// It is the entrypoint function of this module.
pub fn draw_winbox<W>(stdout: &mut W, (rect, uibox): (&Rect, &WinBox)) -> Option<()>
where
    W: std::io::Write,
{
    // Only draw if we have inner area, that is (rect size is >= 3)
    let inner = rect.inner()?;

    // Horizontal character string
    let horizontal = uibox
        .borders
        .horizontal
        .to_string()
        .repeat(inner.width() as usize);

    // Top border
    queue!(
        stdout,
        cursor::MoveTo(rect.x.0, rect.y.0),
        style::PrintStyledContent(uibox.style.apply(uibox.borders.upper_left)),
        style::PrintStyledContent(uibox.style.apply(&horizontal)),
        style::PrintStyledContent(uibox.style.apply(uibox.borders.upper_right)),
    )
    .unwrap();

    // Bottom border
    queue!(
        stdout,
        cursor::MoveTo(rect.x.0, rect.y.1),
        style::PrintStyledContent(uibox.style.apply(uibox.borders.lower_left)),
        style::PrintStyledContent(uibox.style.apply(&horizontal)),
        style::PrintStyledContent(uibox.style.apply(uibox.borders.lower_right)),
    )
    .unwrap();

    // Left/Right borders
    for y in inner.y.0..=inner.y.1 {
        queue!(
            stdout,
            cursor::MoveTo(rect.x.0, y),
            style::Print(uibox.style.apply(uibox.borders.vertical)),
            cursor::MoveRight(inner.cols()),
            style::Print(uibox.style.apply(uibox.borders.vertical))
        )
        .unwrap();
    }

    // Successful return
    Some(())
}
