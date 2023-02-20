//! <https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-tasks--files-and-folders>

#![allow(non_snake_case)]

use crate::wmi;
use crate::COMLibrary;
use crate::Query;
use crate::{Variant, WMIConnection};
use paste::paste;
use std::collections::HashMap;

wmi! {
    /// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/cim-datafile>
    CIM_DataFile, r"Root\CIMV2"
}
wmi! {
    /// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-share>
    Win32_Share, r"Root\CIMV2"
}
wmi! {
    /// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-directory>
    Win32_Directory, r"Root\CIMV2"
}
