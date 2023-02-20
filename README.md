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

## Return type

`type Query = Vec<HashMap<String, Variant>>`.

`String` is the name of the returned struct field with `Variant` being an enum type.

## Currently included queries:

The subsections were defined according
to [WMI Tasks for Scripts and Applications](https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-tasks-for-scripts-and-applications),
you can find more classes [here](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/operating-system-classes).

[Accounts and Domains](https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-tasks--accounts-and-domains)

- [Win32_ComputerSystem](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-computersystem)
- [Win32_Group](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-group)

[Computer Hardware](https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-tasks--computer-hardware)

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

[Desktop Management](https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-tasks--desktop-management)

- [Win32_Desktop](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-desktop)
- [Win32_DesktopMonitor](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-desktopmonitor)
- [Win32_StartupCommand](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-startupcommand)

[Disks and File Systems](https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-tasks--disks-and-file-systems)

- [Win32_DiskQuota](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/wmipdskq/win32-diskquota)
- [Win32_VolumeChangeEvent](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-volumechangeevent)
- [Win32_LogicalDisk](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-logicaldisk)
- [Win32_MappedLogicalDisk](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-mappedlogicaldisk)
- [Win32_Volume](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/aa394515(v=vs.85))
- [Win32_DiskDrive](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-diskdrive)
- [Win32_DiskPartition](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-diskpartition)

[Event Logs](https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-tasks--event-logs)

- [Win32_NTLogEvent](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/eventlogprov/win32-ntlogevent)
- [Win32_NTEventlogFile](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/aa394225(v=vs.85))

[Files and Folders](https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-tasks--files-and-folders)

- [CIM_DataFile](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/cim-datafile)
- [Win32_Share](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-share)
- [Win32_Directory](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-directory)

[Networking](https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-tasks--networking)

- [Win32_NetworkAdapterConfiguration](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-networkadapterconfiguration)
- [Win32_NetworkAdapter](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-networkadapter)

[Operating Systems](https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-tasks--operating-systems)

- [Win32_OperatingSystem](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-operatingsystem)
- [Win32_QuickFixEngineering](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-quickfixengineering)
- [Win32_WindowsProductActivation](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/aa394520(v=vs.85))

[Performance Monitoring](https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-tasks--performance-monitoring)

- [Win32_Perf](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-perf)
- [Win32_PerfFormattedData](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-perfformatteddata)
- [Win32_PerfRawData](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-perfrawdata)

[Processes](https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-tasks--processes)

- [Win32_Process](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-process)
- [Win32_ProcessStartup](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-processstartup)

[Printers and Printing](https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-tasks--printers-and-printing)

- [Win32_Printer](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-printer)

[Registry](https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-tasks--registry)

- [Win32_Registry](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-registry)

[Scheduled Tasks](https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-tasks--scheduled-tasks)

- [Win32_ScheduledJob](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-scheduledjob)

[Services](https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-tasks--services)

- [Win32_Service](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-service)
- [Win32_DependentService](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-dependentservice)

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