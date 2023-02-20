use query_wmi::COMLibrary;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _com_con = COMLibrary::new()?;

    //dbg!(get_Win32_NetworkAdapterConfiguration(com_con)?);
    //dbg!(get_Win32_NetworkAdapter(com_con)?);
    //dbg!(get_Win32_PingStatus(com_con)?);

    Ok(())
}
