# query_wmi

A crate to query `WMI` classes in windows

https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-start-page
> Windows Management Instrumentation (WMI) is the infrastructure for management data and
> operations on Windows-based operating systems. You can write WMI scripts or applications to
> automate administrative tasks on remote computers, but WMI also supplies management data to
> other parts of the operating system and products—for example, System Center Operations Manager
> (formerly Microsoft Operations Manager (MOM)), or Windows Remote Management (`WinRM`).
> Usage:

```rust
use query_wmi::computer_hardware::{
    get_Win32_CDROMDrive, get_Win32_ComputerSystem, get_Win32_OperatingSystem,
    get_Win32_PCMCIAController, get_Win32_PnPEntity, get_Win32_Processor,
    get_Win32_SystemEnclosure, get_Win32_TapeDrive, get_Win32_USBHub,
};
use wmi::COMLibrary;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let com_con = COMLibrary::new()?;
    dbg!(get_Win32_OperatingSystem(com_con)?);
    dbg!(get_Win32_CDROMDrive(com_con)?);
    dbg!(get_Win32_ComputerSystem(com_con)?);
    dbg!(get_Win32_PCMCIAController(com_con)?);
    dbg!(get_Win32_PnPEntity(com_con)?);
    dbg!(get_Win32_Processor(com_con)?);
    dbg!(get_Win32_SystemEnclosure(com_con)?);
    dbg!(get_Win32_USBHub(com_con)?);
    dbg!(get_Win32_TapeDrive(com_con)?);
    Ok(())
}
```

Currently included queries:

[Accounts and Domains](https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-tasks--accounts-and-domains)

- [Win32_ComputerSystem](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-computersystem)
- [Win32_Group](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-group)

[Computer Hardware](https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-tasks--computer-hardware)

- [Win32_OperatingSystem](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-operatingsystem)
- [Win32_CDROMDrive](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-cdromdrive)
- [Win32_ComputerSystem](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-computersystem)
- [Win32_PCMCIAController](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-pcmciacontroller)
- [Win32_PnPEntity](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-pnpentity)
- [Win32_PointingDevice](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-pointingdevice)
- [Win32_Processor](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-processor)
- [Win32_SystemEnclosure](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-systemenclosure)
- [Win32_USBHub](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/cimwin32a/win32-usbhub)
- [Win32_TapeDrive](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-tapedrive)

[Computer Software](https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-tasks--computer-software)

- [Win32_Product](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/aa394378(v=vs.85))

[Dates and Times](https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-tasks--dates-and-times)

- [Win32_LocalTime](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/wmitimepprov/win32-localtime)
- [Win32_TimeZone](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-timezone)

## Building your own queries

You can use the provided `wmi` macro to make your own queries:

```rust
#![allow(non_snake_case)]

use query_wmi::wmi;
use query_wmi::Query;
use paste::paste;
use std::collections::HashMap;
use query_wmi::COMLibrary;
use query_wmi::{Variant, WMIConnection};

// this creates the function `get_CLASS_NAME()`
wmi! {
    /// Documentation
    CLASS_NAME, r"path_to_namespace"
}

// calling it
let com_con = COMLibrary::new() ?;
dbg!(get_CLASS_NAME(com_con)?);
```