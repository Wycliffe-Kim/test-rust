/*
 * Copyright (C) 2024 nota inc. All rights reserved.
 * This source code is the property of nota inc. and is protected by copyright law. THE RECEIPT OR POSSESSION OF  THIS SOURCE CODE AND/OR RELATED INFORMATION DOES NOT CONVEY OR IMPLY TRANSFER OF OWNERSHIP, COPYRIGHT OR ANY RIGHTS INCLUDING BUT NOT LIMITED TO REPRODUCE, DISCLOSE OR DISTRIBUTE ITS CONTENTS, OR TO MANUFACTURE, USE, OR SELL ANYTHING THAT IT  MAY DESCRIBE, IN WHOLE OR IN PART.
 * Unauthorized disclosure, reproduction, modification, distribution, or commercial use of all or any part of this source code without prior written consent from Nota Inc. is strictly prohibited.
 * This source code is provided "as is" and without any warranty, express or implied.
 * Nota Inc. shall not be liable for any damages arising from the use of this source code.
 * For inquiries regarding the use of this source code, please contact Nota Inc. at:
 * Email: contact@nota.ai
 */

use flexi_logger::{Cleanup, Criterion, FileSpec, Logger, Naming};
use log::{set_max_level, LevelFilter};
use once_cell::sync::OnceCell;
use utils::resources_path;

mod gstreamer_runner;
mod utils;

fn log_format_with_timestamp(
    w: &mut dyn std::io::Write,
    now: &mut flexi_logger::DeferredNow,
    record: &log::Record,
) -> std::io::Result<()> {
    write!(
        w,
        "[{}.{:03}] [{}] - {}",
        now.format("%Y-%m-%d %H:%M:%S"),
        now.now().timestamp_subsec_millis(),
        record.level(),
        record.args()
    )
}

fn init_global_logger() {
    let resources_dir = resources_path();
    static LOGGER_INITIALIZED: OnceCell<()> = OnceCell::new();

    LOGGER_INITIALIZED.get_or_init(|| {
        Logger::try_with_str("info")
            .unwrap()
            .log_to_file(
                FileSpec::default()
                    .directory(resources_dir)
                    .basename("logs"),
            )
            .duplicate_to_stdout(flexi_logger::Duplicate::All)
            .rotate(
                Criterion::Size(100_000_000),
                Naming::Numbers,
                Cleanup::KeepLogFiles(3),
            )
            .format(log_format_with_timestamp)
            .start()
            .expect("Failed to initialize logger");

        set_max_level(LevelFilter::Info);
    });
}

#[tokio::main]
async fn main() {
    init_global_logger();
    gstreamer_runner::gstreamer_run().await;
}
