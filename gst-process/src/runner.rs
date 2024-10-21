/*
 * Copyright (C) 2024 nota inc. All rights reserved.
 * This source code is the property of nota inc. and is protected by copyright law. THE RECEIPT OR POSSESSION OF  THIS SOURCE CODE AND/OR RELATED INFORMATION DOES NOT CONVEY OR IMPLY TRANSFER OF OWNERSHIP, COPYRIGHT OR ANY RIGHTS INCLUDING BUT NOT LIMITED TO REPRODUCE, DISCLOSE OR DISTRIBUTE ITS CONTENTS, OR TO MANUFACTURE, USE, OR SELL ANYTHING THAT IT  MAY DESCRIBE, IN WHOLE OR IN PART.
 * Unauthorized disclosure, reproduction, modification, distribution, or commercial use of all or any part of this source code without prior written consent from Nota Inc. is strictly prohibited.
 * This source code is provided "as is" and without any warranty, express or implied.
 * Nota Inc. shall not be liable for any damages arising from the use of this source code.
 * For inquiries regarding the use of this source code, please contact Nota Inc. at:
 * Email: contact@nota.ai
 */

use std::{env, path::PathBuf, process::Stdio};

// use gstreamer::prelude::{ElementExt, GstBinExtManual, GstObjectExt};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};

use crate::{project_root_path, utils::resources_path};

// macro_rules! gst_make {
//     ($call:expr, $invoker:expr) => {
//         match $call {
//             Ok(element) => element,
//             Err(error) => {
//                 return Err(format!("[{}] {}", $invoker, error.to_string()));
//             }
//         }
//     };
// }

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
    // String::from("rtsp://210.99.70.120:1935/live/cctv001.stream")
}

fn playlist_location() -> String {
    format!(
        "playlist-location={}\\output.m3u8",
        resources_path().to_string_lossy()
    )
    .replace("\\", "\\\\")
    // format!("{}\\output.m3u8", resources_path().to_string_lossy()).replace("\\", "\\\\")
}

fn segment_location() -> String {
    format!(
        "location={}\\segment-%05d.ts",
        resources_path().to_string_lossy()
    )
    .replace("\\", "\\\\")
    // format!("{}\\segment-%05d.ts", resources_path().to_string_lossy()).replace("\\", "\\\\")
}

pub async fn run() -> Result<(), String> {
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
    // env::set_var("DYLD_LIBRARY_PATH", gstreamer_lib_path);
    // env::set_var("GST_PLUGIN_PATH", gstreamer_plugin_path);
    // env::set_var("GST_PLUGIN_SCANNER", gstreamer_plugin_scanner_path);
    // env::set_var("PATH", gstreamer_binary_path);

    // if let Err(error) = gstreamer::init() {
    //     return Err(format!("[gstreamer::init] {}", error.to_string()));
    // }

    // let pipeline = gstreamer::Pipeline::new();
    // let rtsprc = gst_make!(
    //     gstreamer::ElementFactory::make("rtspsrc")
    //         .property("location", &location)
    //         .property("latency", &100i32)
    //         .build(),
    //     "rtsprc"
    // );
    // let rtsph264depay = gst_make!(
    //     gstreamer::ElementFactory::make("rtsph264depay").build(),
    //     "rtsph264depay"
    // );
    // let h264parse = gst_make!(
    //     gstreamer::ElementFactory::make("h264parse").build(),
    //     "h264parse"
    // );
    // let mpegtsmux = gst_make!(
    //     gstreamer::ElementFactory::make("mpegtsmux").build(),
    //     "mpegtsmux"
    // );
    // let hlssink = gst_make!(
    //     gstreamer::ElementFactory::make("hlssink")
    //         .property("max-files", &100u32)
    //         .property("target-duration0", &1u32)
    //         .property("playlist-length", &100u32)
    //         .property("playlist-location", &playlist_location)
    //         .property("location", &segment_location)
    //         .build(),
    //     "hlssink"
    // );

    // if let Err(error) =
    //     pipeline.add_many(&[&rtsprc, &rtsph264depay, &h264parse, &mpegtsmux, &hlssink])
    // {
    //     return Err(format!("[pipeline.add_many] {}", error.to_string()));
    // }

    // if let Err(error) =
    //     gstreamer::Element::link_many(&[&rtsph264depay, &h264parse, &mpegtsmux, &hlssink])
    // {
    //     return Err(format!(
    //         "[gstreamer::Element::link_many] {}",
    //         error.to_string()
    //     ));
    // }

    // match pipeline.set_state(gstreamer::State::Playing) {
    //     Ok(status) => {
    //         if status == gstreamer::StateChangeSuccess::Success {
    //             if let Some(bus) = pipeline.bus() {
    //                 for message in bus.iter_timed(gstreamer::ClockTime::NONE) {
    //                     use gstreamer::MessageView;

    //                     match message.view() {
    //                         MessageView::Eos(..) => {
    //                             println!("End of stream");
    //                             break;
    //                         }
    //                         MessageView::Error(err) => {
    //                             eprintln!(
    //                                 "Error received from element {:?}: {}",
    //                                 err.src().map(|s| s.path_string()),
    //                                 err.error()
    //                             );
    //                             break;
    //                         }
    //                         _ => (),
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     Err(error) => {
    //         return Err(format!(
    //             "[pipeline.set_state(Playing)] {}",
    //             error.to_string()
    //         ))
    //     }
    // };

    // if let Err(error) = pipeline.set_state(gstreamer::State::Null) {
    //     return Err(format!("[pipeline.set_state(Null)] {}", error.to_string()));
    // }

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

    Ok(())
}
