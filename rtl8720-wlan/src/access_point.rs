use crate::*;

pub struct WifiAccessPoint;

impl WifiExtensions<{ WifiInterface::AccessPoint as i32 }, { WifiMode::AccessPoint as u32 }>
    for WifiAccessPoint
{
    fn mode_init() -> Result<()> {
        Ok(())
    }
}

impl WifiAccessPoint {}
