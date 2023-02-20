//! <https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-tasks--disks-and-file-systems>

#![allow(non_snake_case)]

use crate::wmi;
use crate::COMLibrary;
use crate::Query;
use crate::{Variant, WMIConnection};
use paste::paste;
use std::collections::HashMap;

wmi! {
    /// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/wmipdskq/win32-diskquota>
    Win32_DiskQuota, r"Root\CIMV2"
}
wmi! {
    /// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-volumechangeevent>
    Win32_VolumeChangeEvent, r"Root\CIMV2"
}

wmi! {
    /// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-logicaldisk>
    Win32_LogicalDisk, r"Root\CIMV2"
}
wmi! {
    /// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-mappedlogicaldisk>
    Win32_MappedLogicalDisk, r"Root\CIMV2"
}
wmi! {
    /// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/aa394515(v=vs.85)>
    Win32_Volume, r"Root\CIMV2"
}

wmi! {
    /// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-diskdrive>
    Win32_DiskDrive, r"Root\CIMV2"
}

wmi! {
    /// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-diskpartition>
    Win32_DiskPartition, r"Root\CIMV2"
}
