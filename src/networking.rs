//! <https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-tasks--networking>

#![allow(non_snake_case)]

use crate::wmi;
use crate::COMLibrary;
use crate::Query;
use crate::{Variant, WMIConnection};
use paste::paste;

use std::collections::HashMap;

wmi! {
    /// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-networkadapterconfiguration>
    Win32_NetworkAdapterConfiguration, r"Root\CIMV2"
}
wmi! {
    /// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-networkadapter>
    Win32_NetworkAdapter, r"Root\CIMV2"
}
/*
wmi! {
    /// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/wmipicmp/win32-pingstatus>
    Win32_PingStatus, r"Root\CIMV2"
}
*/
