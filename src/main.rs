use query_wmi::services::{get_Win32_DependentService, get_Win32_Service};
use query_wmi::COMLibrary;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let com_con = COMLibrary::new()?;

    dbg!(get_Win32_Service(com_con)?);
    dbg!(get_Win32_DependentService(com_con)?);

    Ok(())
}
