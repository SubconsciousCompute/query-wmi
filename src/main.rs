use query_wmi::computer_hardware::{
    get_Win32_CDROMDrive, get_Win32_ComputerSystem, get_Win32_OperatingSystem,
    get_Win32_PCMCIAController, get_Win32_PnPEntity, get_Win32_Processor,
    get_Win32_SystemEnclosure, get_Win32_TapeDrive, get_Win32_USBHub,
};

use query_wmi::computer_software::get_Win32_Product;
use query_wmi::date_and_times::{get_Win32_LocalTime, get_Win32_TimeZone};
use query_wmi::COMLibrary;

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

    dbg!(get_Win32_Product(com_con)?);

    dbg!(get_Win32_TimeZone(com_con)?);
    dbg!(get_Win32_LocalTime(com_con)?);

    Ok(())
}
