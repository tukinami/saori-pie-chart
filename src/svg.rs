use resvg::{
    tiny_skia::{Pixmap, Transform},
    usvg::{Options, Tree, TreeParsing, TreeTextToPath},
    FitTo,
};

use crate::error::PieChartError;

pub(crate) fn render_svg<T>(svg_src: T) -> Result<Pixmap, PieChartError>
where
    T: AsRef<str>,
{
    let mut options = Options::default();
    let fit_to = FitTo::Original;

    let mut fontdb = resvg::usvg::fontdb::Database::new();
    fontdb.load_system_fonts();
    options.languages.push("ja".to_string());
    options.font_family = String::from("ＭＳ ゴシック");

    let mut tree = Tree::from_str(svg_src.as_ref(), &options)?;
    tree.convert_text(&fontdb);

    let size = fit_to
        .fit_to(tree.view_box.rect.to_screen_size())
        .ok_or(PieChartError::SizeTooSmall)?;
    let mut pixmap = Pixmap::new(size.width(), size.height()).ok_or(PieChartError::SizeTooSmall)?;

    resvg::render(&tree, fit_to, Transform::default(), pixmap.as_mut());

    Ok(pixmap)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod render_svg {
        use std::path::PathBuf;

        use crate::png::write_png;

        use super::*;

        #[test]
        fn success_when_valid_svg_str_with_text_element_001() {
            let case = r#"<svg viewBox="0 0 200 30" xmlns="http://www.w3.org/2000/svg">
  <text y="20" font-family="Arial, Helvetica, sans-serif">Sans serif</text>
  <text x="100" y="20" font-family="monospace">Monospace</text>
</svg>
"#;
            let pixmap = render_svg(case).unwrap();
            let output_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("test_target/render_svg/with_text_element_001.png");
            write_png(output_path, &pixmap);
        }

        #[test]
        fn success_when_valid_svg_str_with_text_element_002() {
            let case = r#"<svg viewBox="0 0 300 30" xmlns="http://www.w3.org/2000/svg">
  <text y="20" font-family="sans-serif">ゴシック</text>
  <text x="100" y="20" font-family="monospace">モノスペース</text>
  <text x="200" y="20" font-family="游ゴシック, sans-serif">游ゴシック</text>
</svg>
"#;
            let pixmap = render_svg(case).unwrap();
            let output_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("test_target/render_svg/with_text_element_002.png");
            write_png(output_path, &pixmap);
        }

        #[test]
        fn success_when_valid_svg_str_with_text_element_003() {
            let case = r#"<svg viewBox="0 0 300 30" xmlns="http://www.w3.org/2000/svg">
  <text y="20" font-family="Anything Wrong Font">ABC</text>
  <text x="100" y="20" font-family="Anything Wrong Font">あいう</text>
</svg>
"#;
            let pixmap = render_svg(case).unwrap();
            let output_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("test_target/render_svg/with_text_element_003.png");
            write_png(output_path, &pixmap);
        }
    }
}
