use std::{fs::File, io::BufWriter, path::Path};

use png::Encoder;
use resvg::tiny_skia::Pixmap;

use crate::error::PieChartError;

pub(crate) fn write_png<P>(output_path: P, pixmap: &Pixmap) -> PieChartError
where
    P: AsRef<Path>,
{
    let file = match File::create(output_path) {
        Ok(v) => v,
        Err(e) => return e.into(),
    };
    let w = &mut BufWriter::new(file);

    let mut encoder = Encoder::new(w, pixmap.width(), pixmap.height());
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = match encoder.write_header() {
        Ok(v) => v,
        Err(e) => return e.into(),
    };

    if let Err(e) = writer.write_image_data(pixmap.data()) {
        return e.into();
    }

    PieChartError::Success
}
