use query_wmi::operating_systems::{
    get_Win32_QuickFixEngineering, get_Win32_WindowsProductActivation,
};
use query_wmi::COMLibrary;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let com_con = COMLibrary::new()?;

    dbg!(get_Win32_QuickFixEngineering(com_con)?);
    dbg!(get_Win32_WindowsProductActivation(com_con)?);

    Ok(())
}
