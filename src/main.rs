use query_wmi::operating_systems::{
    get_Win32_QuickFixEngineering, get_Win32_WindowsProductActivation,
};
use query_wmi::performance_monitoring::{
    get_Win32_Perf, get_Win32_PerfFormattedData, get_Win32_PerfRawData,
};
use query_wmi::COMLibrary;
use query_wmi::processes::{get_Win32_Process, get_Win32_ProcessStartup};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let com_con = COMLibrary::new()?;

    dbg!(get_Win32_Process(com_con)?);
    dbg!(get_Win32_ProcessStartup(com_con)?);

    Ok(())
}
