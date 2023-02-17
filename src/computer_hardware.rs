#![allow(non_snake_case)]

use crate::wmi;
use crate::COMLibrary;
use crate::Query;
use crate::{Variant, WMIConnection};
use paste::paste;
use std::collections::HashMap;

// https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-tasks--computer-hardware

wmi!(Win32_OperatingSystem, r"Root\CIMV2");
wmi!(Win32_CDROMDrive, r"Root\CIMV2");
wmi!(Win32_ComputerSystem, r"Root\CIMV2");
wmi!(Win32_PCMCIAController, r"Root\CIMV2");
wmi!(Win32_PnPEntity, r"Root\CIMV2");
wmi!(Win32_PointingDevice, r"Root\CIMV2");
wmi!(Win32_Processor, r"Root\CIMV2");
wmi!(Win32_SystemEnclosure, r"Root\CIMV2");
wmi!(Win32_USBHub, r"Root\CIMV2");
wmi!(Win32_TapeDrive, r"Root\CIMV2");
