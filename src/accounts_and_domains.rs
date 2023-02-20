//! <https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-tasks--accounts-and-domains>

#![allow(non_snake_case)]

use crate::wmi;
use crate::COMLibrary;
use crate::Query;
use crate::{Variant, WMIConnection};
use paste::paste;
use std::collections::HashMap;

wmi! {
    /// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-computersystem>
    Win32_ComputerSystem, r"Root\CIMV2"
}
wmi! {
    /// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-group>
    Win32_Group, r"Root\CIMV2"
}
