use query_wmi::registry::get_Win32_Registry;
use query_wmi::COMLibrary;
use query_wmi::scheduled_tasks::get_Win32_ScheduledJob;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let com_con = COMLibrary::new()?;

    dbg!(get_Win32_ScheduledJob(com_con)?);

    Ok(())
}
