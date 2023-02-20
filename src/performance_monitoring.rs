//! <https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-tasks--performance-monitoring>

#![allow(non_snake_case)]

use crate::wmi;
use crate::COMLibrary;
use crate::Query;
use crate::{Variant, WMIConnection};
use paste::paste;

use std::collections::HashMap;

wmi! {
    /// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-perf>
    Win32_Perf, r"Root\CIMV2"
}
wmi! {
    /// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-perfformatteddata>
    Win32_PerfFormattedData, r"Root\CIMV2"
}
wmi! {
    /// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-perfrawdata>
    Win32_PerfRawData, r"Root\CIMV2"
}
