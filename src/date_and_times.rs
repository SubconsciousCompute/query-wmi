//! <https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-tasks--dates-and-times>

#![allow(non_snake_case)]

use crate::wmi;
use crate::COMLibrary;
use crate::Query;
use crate::{Variant, WMIConnection};
use paste::paste;
use std::collections::HashMap;

wmi! {
    /// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/wmitimepprov/win32-localtime>
    Win32_LocalTime, r"Root\CIMV2"
}
wmi! {
    /// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-timezone>
    Win32_TimeZone, r"Root\CIMV2"
}
