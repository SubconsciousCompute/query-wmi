use query_wmi::desktop_management::{
    get_Win32_Desktop, get_Win32_DesktopMonitor, get_Win32_StartupCommand,
};
use query_wmi::disks_and_file_systems::{
    get_Win32_DiskDrive, get_Win32_DiskPartition, get_Win32_DiskQuota, get_Win32_LogicalDisk,
    get_Win32_MappedLogicalDisk, get_Win32_Volume, get_Win32_VolumeChangeEvent,
};
use query_wmi::event_logs::{get_Win32_NTEventlogFile, get_Win32_NTLogEvent};
use query_wmi::COMLibrary;
use query_wmi::files_and_folders::{get_CIM_DataFile, get_Win32_Directory, get_Win32_Share};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let com_con = COMLibrary::new()?;

    //dbg!(get_CIM_DataFile(com_con)?);
    dbg!(get_Win32_Share(com_con)?);
    dbg!(get_Win32_Directory(com_con)?);

    Ok(())
}
