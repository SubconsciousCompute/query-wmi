//! <https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-tasks--event-logs>

#![allow(non_snake_case)]

use crate::wmi;
use crate::COMLibrary;
use crate::Query;
use crate::{Variant, WMIConnection};
use paste::paste;
use std::collections::HashMap;

wmi! {
    /// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/eventlogprov/win32-ntlogevent>
    Win32_NTLogEvent, r"Root\CIMV2"
}
wmi! {
    /// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/aa394225(v=vs.85)>
    Win32_NTEventlogFile, r"Root\CIMV2"
}
