# Hardware-Specific Packages Needed for Axioo Pongo 725 v1 (Arch Linux)

This document lists essential packages, especially third-party drivers and utilities, required for optimal functionality and stability of Arch Linux on the Axioo Pongo 725 v1 hardware.

---

## Currently Identified Packages

* **`tuxedo_io`**: Kernel module/utility for managing TUXEDO Control Center (e.g., fan control, power profiles). Often necessary for laptops with custom OEM features.
* **`tuxedo_keyboard`**: Driver/utility specifically for managing keyboard functionalities, such as backlight control, for TUXEDO-branded (or similar OEM) keyboards.

---

## Instructions for User

Please add any other critical packages that you identify as necessary for your specific hardware here. This might include:

* GPU drivers (e.g., `nvidia`, `nvidia-utils`, `lib32-nvidia-utils` if not already handled)
* Firmware packages (e.g., `linux-firmware`)
* Audio drivers/firmware
* Touchpad drivers
* Bluetooth/Wi-Fi drivers
* Any other utilities or modules specific to the Axioo Pongo 725 v1.

The goal is to create a comprehensive list that the Rust application can eventually help manage during system installation and ongoing maintenance.
