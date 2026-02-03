#!/bin/bash

# Skrip instalasi untuk m4arch
#
# Skrip ini akan:
# 1. Menyalin aturan udev untuk memberikan izin akses keyboard tanpa sudo.
# 2. Memuat ulang aturan udev agar langsung aktif.

set -e

echo "Menyalin aturan udev untuk m4arch..."

# Dapatkan path direktori skrip ini dijalankan
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

sudo cp "$SCRIPT_DIR/../dist/99-m4arch-keyboard.rules" /etc/udev/rules.d/

echo "Memuat ulang aturan udev..."
sudo udevadm control --reload-rules
sudo udevadm trigger

echo "Instalasi selesai. Izin keyboard seharusnya sudah aktif."
echo "Pastikan user Anda adalah anggota grup 'users'."