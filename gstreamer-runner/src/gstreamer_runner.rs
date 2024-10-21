/*
 * Copyright (C) 2024 nota inc. All rights reserved.
 * This source code is the property of nota inc. and is protected by copyright law. THE RECEIPT OR POSSESSION OF  THIS SOURCE CODE AND/OR RELATED INFORMATION DOES NOT CONVEY OR IMPLY TRANSFER OF OWNERSHIP, COPYRIGHT OR ANY RIGHTS INCLUDING BUT NOT LIMITED TO REPRODUCE, DISCLOSE OR DISTRIBUTE ITS CONTENTS, OR TO MANUFACTURE, USE, OR SELL ANYTHING THAT IT  MAY DESCRIBE, IN WHOLE OR IN PART.
 * Unauthorized disclosure, reproduction, modification, distribution, or commercial use of all or any part of this source code without prior written consent from Nota Inc. is strictly prohibited.
 * This source code is provided "as is" and without any warranty, express or implied.
 * Nota Inc. shall not be liable for any damages arising from the use of this source code.
 * For inquiries regarding the use of this source code, please contact Nota Inc. at:
 * Email: contact@nota.ai
 */

use std::{path::PathBuf, process::Stdio};

use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};

use crate::{project_root_path, utils::resources_path};

fn gstreamer_binary_path() -> PathBuf {
    PathBuf::from_iter([project_root_path!(), "assets\\gstreamer"])
}

fn gstreamer_launch_path() -> PathBuf {
    PathBuf::from(format!(
        "{}\\gst-launch-1.0.exe",
        gstreamer_binary_path().to_string_lossy()
    ))
}

fn gstreamer_lib_path() -> PathBuf {
    PathBuf::from(format!(
        "{}\\lib",
        gstreamer_binary_path().to_string_lossy()
    ))
}

fn gstreamer_plugin_path() -> PathBuf {
    PathBuf::from(format!(
        "{}\\libexec",
        gstreamer_binary_path().to_string_lossy()
    ))
}

fn gstreamer_plugin_scanner_path() -> PathBuf {
    PathBuf::from(format!(
        "{}\\gst-plugin-scanner.exe",
        gstreamer_plugin_path().to_string_lossy()
    ))
}

fn location() -> String {
    String::from("location=rtsp://210.99.70.120:1935/live/cctv001.stream")
}

fn playlist_location() -> String {
    format!(
        "playlist-location={}\\output.m3u8",
        resources_path().to_string_lossy()
    )
    .replace("\\", "\\\\")
}

fn segment_location() -> String {
    format!(
        "location={}\\segment-%05d.ts",
        resources_path().to_string_lossy()
    )
    .replace("\\", "\\\\")
}

pub async fn gstreamer_run() {
    log::info!("project_root_path: {}", project_root_path!());

    let gstreamer_binary_path = gstreamer_binary_path();
    let gstreamer_launch_path = gstreamer_launch_path();
    let gstreamer_lib_path = gstreamer_lib_path();
    let gstreamer_plugin_path = gstreamer_plugin_path();
    let gstreamer_plugin_scanner_path = gstreamer_plugin_scanner_path();
    let location = location();
    let playlist_location = playlist_location();
    let segment_location = segment_location();

    log::info!(
        "gstreamer_binary_path: {}\n
        gstreamer_launch_path: {}\n
        gstreamer_lib_path: {}\n
        gstreamer_plugin_path: {}\n
        gstreamer_plugin_scanner_path: {}\n
        location: {location}\n
        playlist_location: {playlist_location}\n
        segment_location: {segment_location}\n",
        gstreamer_binary_path.to_string_lossy(),
        gstreamer_launch_path.to_string_lossy(),
        gstreamer_lib_path.to_string_lossy(),
        gstreamer_plugin_path.to_string_lossy(),
        gstreamer_plugin_scanner_path.to_string_lossy()
    );

    match Command::new(gstreamer_launch_path)
        .args(&[
            "-v",
            "rtspsrc",
            &location,
            "latency=100",
            "!",
            "rtph264depay",
            "!",
            "h264parse",
            "!",
            "mpegtsmux",
            // "!",
            // "video/mpegts",
            // "!",
            // "queue",
            "!",
            "hlssink",
            "max-files=100",
            "target-duration=1",
            "playlist-length=100",
            &playlist_location,
            &segment_location,
        ])
        .env("DYLD_LIBRARY_PATH", gstreamer_lib_path)
        // .env("PATH", gstreamer_binary_path)
        .env("GST_PLUGIN_PATH", gstreamer_plugin_path)
        .env("GST_PLUGIN_SCANNER", gstreamer_plugin_scanner_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(mut process) => {
            if let Some(stdout) = process.stdout.take() {
                tokio::spawn(async move {
                    let reader = BufReader::new(stdout);
                    let mut lines = reader.lines();

                    while let Ok(Some(line)) = lines.next_line().await {
                        log::info!("{}", line);
                    }
                });
            }

            if let Some(stderr) = process.stderr.take() {
                tokio::spawn(async move {
                    let reader = BufReader::new(stderr);
                    let mut lines = reader.lines();

                    while let Ok(Some(line)) = lines.next_line().await {
                        log::error!("{}", line);
                    }
                });
            }

            match process.wait().await {
                Ok(status) => {
                    log::info!("{:#?}", status);
                }
                Err(error) => {
                    log::error!("{}", error.to_string());
                }
            }
        }
        Err(error) => {
            log::error!("{}", error.to_string());
        }
    };
}
