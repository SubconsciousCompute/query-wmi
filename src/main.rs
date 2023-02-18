use query_wmi::accounts_and_domains::get_Win32_Group;
use query_wmi::computer_hardware::get_Win32_ComputerSystem;

use query_wmi::COMLibrary;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let com_con = COMLibrary::new()?;

    dbg!(get_Win32_ComputerSystem(com_con)?);
    dbg!(get_Win32_Group(com_con)?);

    Ok(())
}
