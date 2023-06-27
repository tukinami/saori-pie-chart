use std::{slice::Iter, str::FromStr};

use svg_pie_chart::{create_pie_chart, Document};

use crate::error::{ArgumentKind, PieChartError};

pub(crate) fn pie_chart(args_iter: &mut Iter<String>) -> Result<Document, PieChartError> {
    let width = parse_args::<u32, _>(args_iter, 1, ArgumentKind::Integer)?;
    let height = parse_args::<u32, _>(args_iter, 2, ArgumentKind::Integer)?;
    let circle_radius = parse_args::<u32, _>(args_iter, 3, ArgumentKind::Integer)?;
    let label_color_r = parse_args::<u8, _>(args_iter, 4, ArgumentKind::Integer)?;
    let label_color_g = parse_args::<u8, _>(args_iter, 5, ArgumentKind::Integer)?;
    let label_color_b = parse_args::<u8, _>(args_iter, 6, ArgumentKind::Integer)?;
    let label_font = args_iter.next().ok_or(PieChartError::NotEnoughArguments)?;
    let label_size = parse_args::<u32, _>(args_iter, 8, ArgumentKind::Integer)?;
    let label_position_radius = parse_args::<u32, _>(args_iter, 9, ArgumentKind::Integer)?;

    let mut statuses = Vec::new();
    let mut number = 10;
    while let (Some(label), Some(range_raw), Some(color)) =
        (args_iter.next(), args_iter.next(), args_iter.next())
    {
        let range = range_raw.parse::<f64>().map_err(|_| {
            PieChartError::InvalidArgumentKind(number + 1, ArgumentKind::FloatNumber)
        })?;
        statuses.push((label, range, color));
        number += 3;
    }

    create_pie_chart(
        width,
        height,
        circle_radius,
        (label_color_r, label_color_g, label_color_b),
        label_font,
        label_size,
        label_position_radius,
        &statuses,
    )
    .map_err(|_| PieChartError::InvalidData)
}

fn parse_args<T, R>(
    iter: &mut Iter<String>,
    number: usize,
    kind: ArgumentKind,
) -> Result<T, PieChartError>
where
    T: FromStr<Err = R>,
{
    iter.next()
        .ok_or(PieChartError::NotEnoughArguments)?
        .parse::<T>()
        .map_err(|_| PieChartError::InvalidArgumentKind(number, kind))
}
