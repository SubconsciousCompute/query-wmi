//! # query-wmi
//!
//! <https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-start-page>
//!
//! > Windows Management Instrumentation (WMI) is the infrastructure for management data and
//! > operations on Windows-based operating systems. You can write WMI scripts or applications to
//! > automate administrative tasks on remote computers, but WMI also supplies management data to
//! > other parts of the operating system and products—for example, System Center Operations Manager
//! > (formerly Microsoft Operations Manager (MOM)), or Windows Remote Management (`WinRM`).
//!
//! Usage:
//!
//! ```ignore
//! use query_wmi::computer_hardware::{
//!     get_Win32_CDROMDrive, get_Win32_ComputerSystem, get_Win32_OperatingSystem,
//!     get_Win32_PCMCIAController, get_Win32_PnPEntity, get_Win32_Processor,
//!     get_Win32_SystemEnclosure, get_Win32_TapeDrive, get_Win32_USBHub,
//! };
//!
//! use wmi::COMLibrary;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let com_con = COMLibrary::new()?;
//!
//!     dbg!(get_Win32_OperatingSystem(com_con)?);
//!     dbg!(get_Win32_CDROMDrive(com_con)?);
//!     dbg!(get_Win32_ComputerSystem(com_con)?);
//!     dbg!(get_Win32_PCMCIAController(com_con)?);
//!     dbg!(get_Win32_PnPEntity(com_con)?);
//!     dbg!(get_Win32_Processor(com_con)?);
//!     dbg!(get_Win32_SystemEnclosure(com_con)?);
//!     dbg!(get_Win32_USBHub(com_con)?);
//!     dbg!(get_Win32_TapeDrive(com_con)?);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Building your own queries
//!
//! You can use the provided [`wmi`](crate::wmi) macro to make your own queries:
//!
//! ```ignore
//! #![allow(non_snake_case)]
//!
//! use query_wmi::wmi;
//! use query_wmi::Query;
//! use paste::paste;
//! use std::collections::HashMap;
//! use query_wmi::COMLibrary;
//! use query_wmi::{Variant, WMIConnection};
//!
//! // this creates the function `get_CLASS_NAME()`
//! wmi!{
//! /// documentation
//! CLASS_NAME, r"path_to_namespace"
//! }
//!
//! // calling it
//! let com_con = COMLibrary::new()?;
//! dbg!(get_CLASS_NAME(com_con)?);
//! ```

pub mod accounts_and_domains;
pub mod computer_hardware;
pub mod computer_software;
pub mod date_and_times;
pub mod desktop_management;
pub mod disks_and_file_systems;
pub mod event_logs;
pub mod files_and_folders;
pub mod networking;
pub mod operating_systems;
pub mod performance_monitoring;

use std::collections::HashMap;

pub use wmi::COMLibrary;
pub use wmi::{Variant, WMIConnection};

// https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-tasks-for-scripts-and-applications

pub type Query = Vec<HashMap<String, Variant>>;

/// Our main macro to build queries.
///
/// The first argument takes documentation followed by WMI `class` name and it is followed by
/// `Namespace` or where it is located.
/// ## Building your own queries
///
/// You can use the provided [`wmi`](crate::wmi) macro to make your own queries:
///
/// ```ignore
/// #![allow(non_snake_case)]
///
/// use query_wmi::wmi;
/// use query_wmi::Query;
/// use paste::paste;
/// use std::collections::HashMap;
/// use query_wmi::COMLibrary;
/// use query_wmi::{Variant, WMIConnection};
///
/// // this creates the function `get_CLASS_NAME()`
/// wmi!{
/// /// documentation
/// CLASS_NAME, r"path_to_namespace"
/// }
///
/// // calling it
/// let com_con = COMLibrary::new()?;
/// dbg!(get_CLASS_NAME(com_con)?);
/// ```
#[macro_export]
macro_rules! wmi {
    (
        $(#[$attr:meta])*
        $query: ident, $path: expr
    ) => {
        paste!{
            $(#[$attr])*
            pub fn [<get_ $query>](com_con: COMLibrary) -> Result<Query, Box<dyn std::error::Error>> {
                    let wmi_con = WMIConnection::with_namespace_path($path, com_con)?;

                    let results: Vec<HashMap<String, Variant>> =
                        wmi_con.raw_query(format!("SELECT * FROM {}", stringify!($query)))?;

                    Ok(results)
            }
        }
    };
}
