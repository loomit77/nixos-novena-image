# NOVENA-JESSIE – Erfolgreicher historischer SD-Boot

Datum: 18.10.2026
Abschluss: NOVENA-JESSIE-58

## Bestätigte Ergebnisse

- Historisches Debian Jessie 8.2 bootet von SD.
- U-Boot SPL und U-Boot 2015.04 funktionieren.
- Kernel: 3.17.0-rc5-00238-gfb115dd.
- Root-Dateisystem: ext4 auf mmcblk0p3.
- Serielle Konsole: ttymxc1, 115200 Baud.
- Internes LVDS-Display funktioniert.
- LightDM zeigt die grafische Anmeldung.
- Anmeldung und Abmeldung als mike erfolgreich.
- Cherry-USB-Tastatur und Logitech-G502-Maus funktionieren.

## Xorg-Fehlerursache

Die historische Datei

/usr/share/X11/xorg.conf.d/60-novena.conf

aktivierte:

    Option "UseGPU" "true"
    Option "AccelModule" "etnadrm_gpu"

Xorg meldete:

    Failed to load module "etnadrm_gpu" (no hardware found, 0)
    no screens found

## Erfolgreich getestete Änderung

Zusätzliche Datei:

/etc/X11/xorg.conf.d/20-novena-armada.conf

Inhalt:

    Section "Device"
        Identifier "Novena Armada"
        Driver "armada"
        Option "UseGPU" "false"
    EndSection

Die historische Konfiguration wurde zunächst gesichert:

/root/60-novena.conf.jessie57-backup

Anschließend wurde sie aus dem aktiven Xorg-Verzeichnis
verschoben nach:

/usr/share/X11/60-novena.conf.jessie57-disabled

Nach Neustart von LightDM erschien die grafische Anmeldung.
Grafische Anmeldung und Abmeldung wurden erfolgreich getestet.

## Rücknahme

Auf der laufenden Jessie-SD, mit Root-Rechten:

    mv /usr/share/X11/60-novena.conf.jessie57-disabled /usr/share/X11/xorg.conf.d/60-novena.conf
    rm /etc/X11/xorg.conf.d/20-novena-armada.conf
    systemctl restart lightdm

Vor der Rücknahme Dateipfade und vorhandene Sicherung prüfen.

## Einschränkungen

- GPU-Beschleunigung ist deaktiviert.
- Die Ursache des fehlenden etnadrm_gpu-Geräts wurde
  nicht weiter untersucht.
- Die vgaarb-Meldungen bleiben beobachtet, aber
  ohne nachgewiesene Auswirkung auf die Anmeldung.
- Kein zusätzlicher Boot- oder Stabilitätstest erfolgt.
- Kein Nachweis einer dauerhaften Konfiguration nach Reboot.

## Schutz bestehender Systeme

- Keine Änderungen an der SATA-SSD.
- Keine Änderungen am NixOS-SD-Image.
- Keine Änderungen am gesicherten experimentellen
  Jessie-Image auf foobox.

## Ergebnis

JESSIE_SD_BOOT=PASS
JESSIE_GRAPHICAL_LOGIN=PASS
JESSIE_LOGOUT=PASS
JESSIE_INPUT_DEVICES=PASS
