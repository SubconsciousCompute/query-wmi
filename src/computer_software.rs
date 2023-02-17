#![allow(non_snake_case)]

use crate::wmi;
use crate::COMLibrary;
use crate::Query;
use crate::{Variant, WMIConnection};
use paste::paste;
use std::collections::HashMap;

// https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-tasks--computer-software

wmi!(Win32_Product, r"Root\CIMV2");
