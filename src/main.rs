use query_wmi::desktop_management::{
    get_Win32_Desktop, get_Win32_DesktopMonitor, get_Win32_StartupCommand,
};
use query_wmi::COMLibrary;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let com_con = COMLibrary::new()?;

    dbg!(get_Win32_Desktop(com_con)?);
    dbg!(get_Win32_DesktopMonitor(com_con)?);
    dbg!(get_Win32_StartupCommand(com_con)?);

    Ok(())
}
