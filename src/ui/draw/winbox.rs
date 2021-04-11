use crate::ui::layout::HorizontalAlignment;
use crate::ui::rect::Rect;
use crate::ui::term::TermUSize;
use crate::ui::winbox::{BoxHint, WinBox};
use crossterm::{cursor, queue, style};

/// Draw the Window Box widget in the Rect space.
///
/// It is the entrypoint function of this module.
pub fn draw_winbox<W>(stdout: &mut W, (rect, winbox): (&Rect, &WinBox)) -> Option<()>
where
    W: std::io::Write,
{
    draw_borders(stdout, (rect, winbox))?;
    draw_hints(stdout, (rect, winbox))
}

/// Basically, it just draws window borders around the Rect.
fn draw_borders<W>(stdout: &mut W, (rect, winbox): (&Rect, &WinBox)) -> Option<()>
where
    W: std::io::Write,
{
    // Only draw if we have inner area, that is (rect size is >= 3)
    let inner = rect.inner()?;

    // Horizontal character string
    let horizontal = winbox
        .borders
        .horizontal
        .to_string()
        .repeat(inner.width() as usize);

    // Top border
    queue!(
        stdout,
        cursor::MoveTo(rect.x.0, rect.y.0),
        style::PrintStyledContent(winbox.style.apply(winbox.borders.upper_left)),
        style::PrintStyledContent(winbox.style.apply(&horizontal)),
        style::PrintStyledContent(winbox.style.apply(winbox.borders.upper_right)),
    )
    .unwrap();

    // Bottom border
    queue!(
        stdout,
        cursor::MoveTo(rect.x.0, rect.y.1),
        style::PrintStyledContent(winbox.style.apply(winbox.borders.lower_left)),
        style::PrintStyledContent(winbox.style.apply(&horizontal)),
        style::PrintStyledContent(winbox.style.apply(winbox.borders.lower_right)),
    )
    .unwrap();

    // Left/Right borders
    for y in inner.y.0..=inner.y.1 {
        queue!(
            stdout,
            cursor::MoveTo(rect.x.0, y),
            style::Print(winbox.style.apply(winbox.borders.vertical)),
            cursor::MoveRight(inner.cols()),
            style::Print(winbox.style.apply(winbox.borders.vertical))
        )
        .unwrap();
    }

    // Successful draw
    Some(())
}

fn draw_hints<W>(stdout: &mut W, (rect, winbox): (&Rect, &WinBox)) -> Option<()>
where
    W: std::io::Write,
{
    let draw_hint = &mut |hint: &BoxHint, (x, y), available_width| {
        let actual_content = super::format_strip_align(
            &hint.content,
            available_width,
            &HorizontalAlignment::default(),
        );
        queue!(
            stdout,
            cursor::MoveTo(x, y),
            style::PrintStyledContent(winbox.style.apply(winbox.borders.vertical_left)),
            style::PrintStyledContent(hint.style.apply(actual_content)),
            style::PrintStyledContent(winbox.style.apply(winbox.borders.vertical_right)),
        )
        .unwrap();
    };

    let inner_x = rect.inner_x()?;
    foreach_boxhint_compute_and_draw(&inner_x, &winbox.top_hints, rect.y.0, draw_hint);
    foreach_boxhint_compute_and_draw(&inner_x, &winbox.bottom_hints, rect.y.1, draw_hint);

    Some(())
}

/// Traverse the boxhints vector and compute some information for the drawing function.
///
/// For each boxhint the available width is calculated and passed on to the drawing function.
/// When there is no more room in the screen, break the drawing loop.
///
/// draw_callback: FnMut(boxhint, (x,y), available_width)
fn foreach_boxhint_compute_and_draw<F>(
    rect: &Rect, boxhints: &Vec<BoxHint>, y_pos: TermUSize, draw_callback: &mut F,
) where
    F: FnMut(&BoxHint, (TermUSize, TermUSize), usize),
{
    let mut used_left_width = 0;
    let mut used_right_width = 0;
    let mut used_center_width = 0;

    for hint in boxhints {
        let brackets_width = 2;
        let margins = hint.margin.left + hint.margin.right;
        let minimum_width = margins + brackets_width + 1;

        let content_width = match hint.alignment {
            HorizontalAlignment::Left | HorizontalAlignment::Right => {
                let available_width =
                    rect.width() as usize - used_left_width - used_right_width - used_center_width;
                if available_width < minimum_width {
                    break;
                }
                let available_hint_width = available_width - brackets_width - margins;
                std::cmp::min(hint.content.len(), available_hint_width)
            }
            HorizontalAlignment::Center => {
                let half_width = rect.width() as f32 / 2.0;
                let available_left_width = half_width - used_left_width as f32;
                let available_right_width = half_width - used_right_width as f32;
                let available_center_width = available_left_width + available_right_width;
                let available_center_width = std::cmp::max(0, available_center_width as usize);
                let available_width = available_center_width - used_center_width;
                if available_width < minimum_width {
                    break;
                }
                let available_hint_width = available_width - brackets_width - margins;
                std::cmp::min(hint.content.len(), available_hint_width)
            }
        };

        let x_pos = match hint.alignment {
            HorizontalAlignment::Left => {
                (rect.x.0 as usize + used_left_width + hint.margin.left) as TermUSize
            }
            HorizontalAlignment::Right => {
                (rect.x.1 as usize + 1
                    - used_right_width
                    - content_width
                    - brackets_width
                    - hint.margin.right) as TermUSize
            }
            HorizontalAlignment::Center => {
                let half_width = rect.width() as f32 / 2.0;
                let half_content_width = content_width as f32 / 2.0;
                let half_used_center_width = used_center_width as f32 / 2.0;
                let half_brackets_width = brackets_width as f32 / 2.0;
                let offset = half_width
                    - half_content_width
                    - half_used_center_width
                    - half_brackets_width
                    /* - half_margins */;
                let center_x = rect.x.0 as usize + offset.round() as usize /* + hint.margin.left */;
                let center_x_end = center_x + content_width /* + hint.margin.right */ + 1 /* non-inclusive */;
                // left and right shift, for when the left/right hints have passed over the center already
                let left_x = rect.x.0 as usize + used_left_width + hint.margin.left;
                let left_shift = std::cmp::max(0, left_x as i32 - center_x as i32) as usize;
                let right_x = rect.x.1 as usize - used_right_width - hint.margin.right;
                let right_shift = std::cmp::max(0, center_x_end as i32 - right_x as i32) as usize;
                (center_x + left_shift - right_shift) as TermUSize
            }
        };

        draw_callback(hint, (x_pos, y_pos), content_width);

        let used_width = content_width + brackets_width + margins;
        match hint.alignment {
            HorizontalAlignment::Left => used_left_width += used_width,
            HorizontalAlignment::Right => used_right_width += used_width,
            HorizontalAlignment::Center => used_center_width += used_width,
        }
    }
}
