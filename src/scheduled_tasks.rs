//! <https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-tasks--scheduled-tasks>

#![allow(non_snake_case)]

use crate::wmi;
use crate::COMLibrary;
use crate::Query;
use crate::{Variant, WMIConnection};
use paste::paste;

use std::collections::HashMap;

wmi! {
    /// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-scheduledjob>
    Win32_ScheduledJob, r"Root\CIMV2"
}
