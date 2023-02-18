//! <https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-tasks--computer-hardware>

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
    /// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-cdromdrive>
    Win32_CDROMDrive, r"Root\CIMV2"
}
wmi! {
    /// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-computersystem>
    Win32_ComputerSystem, r"Root\CIMV2"
}
wmi! {
    /// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-pcmciacontroller>
    Win32_PCMCIAController, r"Root\CIMV2"
}
wmi! {
    /// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-pnpentity>
    Win32_PnPEntity, r"Root\CIMV2"
}
wmi! {
    /// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-pointingdevice>
    Win32_PointingDevice, r"Root\CIMV2"
}
wmi! {
    /// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-processor>
    Win32_Processor, r"Root\CIMV2"
}
wmi! {
    /// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-systemenclosure>
    Win32_SystemEnclosure, r"Root\CIMV2"
}
wmi! {
    /// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/cimwin32a/win32-usbhub>
    Win32_USBHub, r"Root\CIMV2"
}
wmi! {
    /// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-tapedrive>
    Win32_TapeDrive, r"Root\CIMV2"
}
