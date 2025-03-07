# Sony Headphones Bluetooth Communication Protocol
### (This file was made by Claude AI using [Claude Code](https://github.com/anthropics/claude-code))

## Overview

The Sony Connect App uses Bluetooth Low Energy (BLE) to communicate with Sony headphones. The communication follows a specific protocol with commands and responses. These files provide a structured way to understand and implement this protocol.

## Key Components

### 1. BluetoothAddress.java
Represents a Bluetooth device address (MAC address) in the format XX:XX:XX:XX:XX:XX.

### 2. BLEInfo.java
Contains information about a Bluetooth Low Energy (BLE) device including the Bluetooth device address, BLE address, and model name.

### 3. SCAInfo.java
Stores information about the Sony Connect Application (SCA) version.

### 4. RegisteredDevice.java
Represents a registered Bluetooth device with all its associated information, including BLE information, SCA information, device type, and capability data.

### 5. HpCapabilityData.java
Contains capability information for Sony headphones, including supported features such as speech detection, quick access, custom playback, etc.

### 6. SonyBleConnectionManager.java
Handles the Bluetooth Low Energy (BLE) connections with Sony headphones, including device discovery, connection establishment, and data transmission.

### 7. SonyCommandProtocol.java
Defines the protocol for communicating with Sony headphones, providing methods for creating and parsing BLE commands.

### 8. SonyHeadphoneUtility.java
Provides utility methods for working with Sony headphones, such as parsing data and creating commands.

## Bluetooth Communication Protocol

The communication between the app and Sony headphones follows this general flow:

1. **Device Discovery**: The app scans for available BLE devices and filters for Sony headphones.
2. **Connection**: The app connects to a selected Sony headphone device.
3. **Service Discovery**: The app discovers the BLE services and characteristics provided by the headphones.
4. **Enable Notifications**: The app enables notifications for the characteristic that receives data from the headphones.
5. **Command Exchange**: The app sends commands to the headphones and receives responses.

### Command Format

Commands sent to the headphones follow this format:
```
[Command Type (1 byte)][Length (2 bytes)][Sequence Number (1 byte)][Payload]
```

### Response Format

Responses from the headphones follow this format:
```
[Response Type (1 byte)][Length (2 bytes)][Sequence Number (1 byte)][Payload]
```

## Implementing on Desktop

To implement this protocol on a desktop platform, you would need:

1. A Bluetooth adapter that supports Bluetooth Low Energy (BLE).
2. A BLE library for your platform (e.g., BlueZ for Linux, Windows.Devices.Bluetooth for Windows, CoreBluetooth for macOS).
3. Implementation of the Sony command protocol described in these files.

### Key Steps for Desktop Implementation:

1. **Device Discovery**: Scan for Sony headphones using their known service UUIDs or by filtering device names.
2. **Connection**: Connect to the selected headphone using the platform's BLE API.
3. **Service Discovery**: Discover the Sony headphone service and its characteristics.
4. **Communication**: Implement the command protocol to send commands and receive responses.
5. **Feature Implementation**: Implement specific features like getting headphone status, controlling noise cancellation, etc.

## Important UUIDs

These UUIDs are essential for communicating with Sony headphones:

- **Service UUID**: `96CC203E-5068-46ad-B32D-E316F5E069BA`
- **Command Characteristic UUID**: `96CC0076-5068-46ad-B32D-E316F5E069BA` (Write)
- **Notification Characteristic UUID**: `96CC0077-5068-46ad-B32D-E316F5E069BA` (Notify)
- **Client Characteristic Configuration Descriptor UUID**: `00002902-0000-1000-8000-00805f9b34fb`

## Notes

- The actual UUID values and command codes should be verified with a BLE traffic analyzer when implementing with real Sony headphones.
- Some features may vary between different Sony headphone models.
- The headphones might require specific authentication or initialization sequences that aren't documented here.