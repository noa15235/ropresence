//! Detect whether the Roblox player/studio process is currently running.
//!
//! Uses the Windows ToolHelp snapshot API directly (no repeated process
//! spawning), which is cheap enough to call on every poll tick.

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RobloxProcesses {
    pub player: bool,
    pub studio: bool,
}

impl RobloxProcesses {
    pub fn any(&self) -> bool {
        self.player || self.studio
    }
}

#[cfg(windows)]
pub fn detect() -> RobloxProcesses {
    use std::mem::size_of;
    use windows::Win32::Foundation::CloseHandle;
    use windows::Win32::System::Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W,
        TH32CS_SNAPPROCESS,
    };

    let mut result = RobloxProcesses::default();

    unsafe {
        let snapshot = match CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) {
            Ok(handle) => handle,
            Err(_) => return result,
        };

        let mut entry = PROCESSENTRY32W {
            dwSize: size_of::<PROCESSENTRY32W>() as u32,
            ..Default::default()
        };

        if Process32FirstW(snapshot, &mut entry).is_ok() {
            loop {
                let name = wide_to_string(&entry.szExeFile);
                if name.eq_ignore_ascii_case("RobloxPlayerBeta.exe") {
                    result.player = true;
                } else if name.eq_ignore_ascii_case("RobloxStudioBeta.exe") {
                    result.studio = true;
                }
                if result.player && result.studio {
                    break;
                }
                if Process32NextW(snapshot, &mut entry).is_err() {
                    break;
                }
            }
        }

        let _ = CloseHandle(snapshot);
    }

    result
}

#[cfg(not(windows))]
pub fn detect() -> RobloxProcesses {
    // Non-Windows builds (CI, type-checking) get an empty result.
    RobloxProcesses::default()
}

#[cfg(windows)]
fn wide_to_string(buf: &[u16]) -> String {
    let end = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    String::from_utf16_lossy(&buf[..end])
}
