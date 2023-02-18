//! <https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-tasks--computer-software>

#![allow(non_snake_case)]

use crate::wmi;
use crate::COMLibrary;
use crate::Query;
use crate::{Variant, WMIConnection};
use paste::paste;
use std::collections::HashMap;

wmi! {
    /// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/aa394378(v=vs.85)>
    Win32_Product, r"Root\CIMV2"
}
