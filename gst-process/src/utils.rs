/*
 * Copyright (C) 2024 nota inc. All rights reserved.
 * This source code is the property of nota inc. and is protected by copyright law. THE RECEIPT OR POSSESSION OF  THIS SOURCE CODE AND/OR RELATED INFORMATION DOES NOT CONVEY OR IMPLY TRANSFER OF OWNERSHIP, COPYRIGHT OR ANY RIGHTS INCLUDING BUT NOT LIMITED TO REPRODUCE, DISCLOSE OR DISTRIBUTE ITS CONTENTS, OR TO MANUFACTURE, USE, OR SELL ANYTHING THAT IT  MAY DESCRIBE, IN WHOLE OR IN PART.
 * Unauthorized disclosure, reproduction, modification, distribution, or commercial use of all or any part of this source code without prior written consent from Nota Inc. is strictly prohibited.
 * This source code is provided "as is" and without any warranty, express or implied.
 * Nota Inc. shall not be liable for any damages arising from the use of this source code.
 * For inquiries regarding the use of this source code, please contact Nota Inc. at:
 * Email: contact@nota.ai
 */

use std::{fs::create_dir_all, path::PathBuf};

#[macro_export]
macro_rules! project_root_path {
    () => {
        std::env::var("CARGO_MANIFEST_DIR").unwrap_or(String::new())
    };
}

#[macro_export]
macro_rules! gstreamer_root_path {
    ($path:expr) => {
        if cfg!(debug_assertions) {
            if cfg!(target_os = "windows") {
                std::path::PathBuf::from(format!(
                    "{}{}",
                    std::env::var("GSTREAMER_1_0_ROOT_MSVC_X86_64").unwrap_or(String::new()),
                    String::from($path)
                ))
            } else if cfg!(target_os = "macos") {
                std::path::PathBuf::from(format!(
                    "/Library/Frameworks/Gstreamer.framework/Versions/Current/{}",
                    String::from($path)
                ))
            } else {
                std::path::PathBuf::default()
            }
        } else {
            std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap_or(String::new()))
        }
    };

    ($windows:expr, $macos:expr) => {
        if cfg!(debug_assertions) {
            if cfg!(target_os = "windows") {
                std::path::PathBuf::from(format!(
                    "{}{}",
                    std::env::var("GSTREAMER_1_0_ROOT_MSVC_X86_64").unwrap_or(String::new()),
                    String::from($windows)
                ))
            } else if cfg!(target_os = "macos") {
                std::path::PathBuf::from(format!(
                    "/Library/Frameworks/Gstreamer.framework/Versions/Current/{}",
                    String::from($macos)
                ))
            } else {
                std::path::PathBuf::default()
            }
        } else {
            if cfg!(target_os = "windows") {
                std::path::PathBuf::from(format!(
                    "{}{}",
                    std::env::var("CARGO_MANIFEST_DIR").unwrap_or(String::new()),
                    String::from($windows)
                ))
            } else if cfg!(target_os = "macos") {
                std::path::PathBuf::from(format!(
                    "{}{}",
                    std::env::var("CARGO_MANIFEST_DIR").unwrap_or(String::new()),
                    String::from($macos)
                ))
            } else {
                std::path::PathBuf::from(
                    std::env::var("CARGO_MANIFEST_DIR").unwrap_or(String::new()),
                )
            }
        }
    };
}

pub fn resources_path() -> PathBuf {
    let resources_path = PathBuf::from_iter([project_root_path!(), String::from("resources")]);
    if !resources_path.exists() {
        if let Err(error) = create_dir_all(resources_path.clone()) {
            log::error!("{}", error.to_string());
        }
    }
    resources_path
}
