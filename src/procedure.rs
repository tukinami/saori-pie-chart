use std::path::PathBuf;

use crate::error::PieChartError;
use crate::piechart::pie_chart;
use crate::png::write_png;
use crate::request::*;
use crate::response::*;
use crate::svg::render_svg;

/// load時に呼ばれる関数
pub fn load(_path: &str) {}

/// unload時に呼ばれる関数
pub fn unload(_path: &str) {}

/// request GET Version時に呼ばれる関数
pub fn get_version(_path: &str, _request: &SaoriRequest, response: &mut SaoriResponse) {
    response.set_result(String::from(env!("CARGO_PKG_VERSION")));
}

/// request EXECUTE時に呼ばれる関数
/// メインの処理はここに記述する
pub fn execute(path: &str, request: &SaoriRequest, response: &mut SaoriResponse) {
    let args = request.argument();
    let mut path = PathBuf::from(path);
    if !path.is_dir() {
        path.pop();
    }
    let mut args_iter = args.iter();
    let output_path = if let Some(path_raw) = args_iter.next() {
        path.join(path_raw)
    } else {
        response.set_result(PieChartError::NotEnoughArguments.to_string());
        return;
    };

    let pie_chart = match pie_chart(&mut args_iter) {
        Ok(v) => v,
        Err(e) => {
            response.set_result(e.to_string());
            return;
        }
    };

    let pixmap = match render_svg(pie_chart.to_string()) {
        Ok(v) => v,
        Err(e) => {
            response.set_result(e.to_string());
            return;
        }
    };

    let result = write_png(output_path, &pixmap);

    response.set_result(result.to_string());
}
