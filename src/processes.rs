//! <https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-tasks--processes>

#![allow(non_snake_case)]

use crate::wmi;
use crate::COMLibrary;
use crate::Query;
use crate::{Variant, WMIConnection};
use paste::paste;

use std::collections::HashMap;

wmi! {
    /// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-process>
    Win32_Process, r"Root\CIMV2"
}
wmi! {
    /// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-processstartup>
    Win32_ProcessStartup, r"Root\CIMV2"
}
