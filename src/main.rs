mod ptrs;
use log::{debug, error, info, trace, warn};
//use ptrs;
use std;

fn main() {
    let project_name: std::path::PathBuf = ptrs::find_project_root_path(env!("CARGO_PKG_NAME"))
        .ok()
        .unwrap();
    let test_tmp_file_path: std::path::PathBuf =
        project_name.clone().join("tmp_test_build_logger.log");
    ptrs::build_logger(test_tmp_file_path.clone()).ok().unwrap();
    trace!(
        "{}的測試日志<追蹤>",
        project_name.file_name().unwrap().to_str().unwrap()
    );
    debug!(
        "{}的測試日志<除錯>",
        project_name.file_name().unwrap().to_str().unwrap()
    );
    info!(
        "{}的測試日志<資訊>",
        project_name.file_name().unwrap().to_str().unwrap()
    );
    warn!(
        "{}的測試日志<警告>",
        project_name.file_name().unwrap().to_str().unwrap()
    );
    error!(
        "{}的測試日志<錯誤>",
        project_name.file_name().unwrap().to_str().unwrap()
    );
}
