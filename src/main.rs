use query_wmi::registry::get_Win32_Registry;
use query_wmi::COMLibrary;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let com_con = COMLibrary::new()?;

    dbg!(get_Win32_Registry(com_con)?);

    Ok(())
}
