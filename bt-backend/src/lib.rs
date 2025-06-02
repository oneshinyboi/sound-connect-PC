use bluer::{Adapter, Device, AdapterEvent, gatt::remote::Characteristic, Uuid, UuidExt};
use futures::StreamExt;

const SONY_SERVICE_UUID: Uuid = Uuid::from_u128(0x96CC203E_5068_46ad_B32D_E316F5E069BA);
const SONY_CMD_CHAR_UUID: Uuid = Uuid::from_u128(0x96CC0076_5068_46ad_B32D_E316F5E069BA);
const SONY_NOTIFY_CHAR_UUID: Uuid = Uuid::from_u128(0x96CC0077_5068_46ad_B32D_E316F5E069BA);
pub struct SonyHeadphoneConnection {
    device: Device,
    cmd_char: Option<Characteristic>,
    notify_char: Option<Characteristic>,
}

impl SonyHeadphoneConnection {
    pub async fn discover_and_connect() -> Result<Self, Box<dyn std::error::Error>> {
        let session = bluer::Session::new().await?;
        let adapter = session.default_adapter().await?;
        
        if !adapter.is_powered().await? {
            return Err("Bluetooth adapter is not powered on. Please enable Bluetooth.".into());
        }

        let device = find_sony_device(&adapter).await?;
        
        device.connect().await?;

        let service = device.service(SONY_SERVICE_UUID.as_u16().unwrap()).await?;
        let cmd_char = service.characteristic(SONY_CMD_CHAR_UUID.as_u16().unwrap()).await.ok();
        let notify_char = service.characteristic(SONY_NOTIFY_CHAR_UUID.as_u16().unwrap()).await.ok();

        Ok(Self { device, cmd_char, notify_char })
    }

    pub async fn send_command(&self, cmd_type: u8, payload: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(char) = &self.cmd_char {
            let packet = create_command_packet(cmd_type, payload);
            char.write(&packet).await?;
        }
        Ok(())
    }
}
async fn find_sony_device(adapter: &Adapter) -> Result<Device, Box<dyn std::error::Error>> {
    let mut events = adapter.discover_devices().await?;

    while let Some(evt) = events.next().await {
        match evt {
            AdapterEvent::DeviceAdded(addr) => {
                println!("New or changed device: {addr}");
                let device = adapter.device(addr)?;
                let name = device.name().await?;
                let rssi = device.rssi().await?;
                println!("  name: {name:?}, rssi: {rssi:?}");
                
                // TODO: actually find a way to detect a sony device 
                if name == Some("Sony".to_string()) {
                    return Ok(device)
                }
            }
            AdapterEvent::DeviceRemoved(addr) => {
                println!("Device out of range: {addr}");
            }
            AdapterEvent::PropertyChanged(prop) => {
                println!("Device had {prop:?} change");
            }
        }
    }

    Err("Discovery stopped for some reason".into())
}
fn create_command_packet(cmd_type: u8, payload: &[u8]) -> Vec<u8> {
    let mut packet = Vec::new();
    packet.push(cmd_type);
    packet.extend_from_slice(&(payload.len() as u16).to_le_bytes());
    packet.push(0x00); // sequence number
    packet.extend_from_slice(payload);
    packet
}
