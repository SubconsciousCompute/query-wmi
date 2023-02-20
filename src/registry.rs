//! <https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-tasks--registry>

#![allow(non_snake_case)]

use crate::wmi;
use crate::COMLibrary;
use crate::Query;
use crate::{Variant, WMIConnection};
use paste::paste;

use std::collections::HashMap;

wmi! {
    /// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-registry>
    Win32_Registry, r"Root\CIMV2"
}
