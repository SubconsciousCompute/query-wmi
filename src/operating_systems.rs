//! <https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-tasks--operating-systems>

#![allow(non_snake_case)]

use crate::wmi;
use crate::COMLibrary;
use crate::Query;
use crate::{Variant, WMIConnection};
use paste::paste;

use std::collections::HashMap;

wmi! {
    /// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-operatingsystem>
    Win32_OperatingSystem, r"Root\CIMV2"
}

wmi! {
    /// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-quickfixengineering>
    Win32_QuickFixEngineering, r"Root\CIMV2"
}
wmi! {
    /// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/aa394520(v=vs.85)>
    ///
    /// *Note:* Support ended with Windows XP
    Win32_WindowsProductActivation, r"Root\CIMV2"
}
