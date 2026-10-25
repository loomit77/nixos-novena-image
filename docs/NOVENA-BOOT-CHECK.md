# Novena – standardisierter Boot-Check

Stand: 2026-10-25
Status: Erstimplementierung; noch nicht auf Hardware ausgeführt.

## Ziel

Nach jedem geeigneten Hardwareboot dieselben lesenden
Systemprüfungen über den vorhandenen Runner ausführen.

Ausführung auf foobox:

  bash scripts/novena-boot-check.bash root@192.168.11.13

Das Skript ist als Kommando für den vorhandenen
run-and-upload.bash-Wrapper vorgesehen. Der Wrapper
verwaltet TXT-Protokoll und SHA256.

Eine automatisierte JSON-Historie wird erst nach
Festlegung des Schemas implementiert. Bis dahin
bleiben die vollständigen TXT-Protokolle maßgeblich.

## Mindestdaten

- Prüfzeitpunkt
- Kernel, Systemgeneration und Boot-ID
- IP-Konfiguration und Routen
- SD-Partitionen, Root-Mount und Root-Grow
- DRM-Connectoren und Modi
- Backlight-Werte
- IT6251-Treiberbindung
- relevante Kernelmeldungen
- fehlgeschlagene systemd-Einheiten

Die echte U-Boot-Ausgabe wird weiterhin über
den seriellen Mitschnitt belegt.

Ob das Display physisch während SPL oder U-Boot
sichtbar war, ist eine gesonderte Sichtbeobachtung.

## SSH

Der private SSH-Client-Schlüssel bleibt auf foobox.
Nur sein öffentlicher Teil darf im Entwicklungsimage
als authorized_keys-Eintrag vorhanden sein.

Server-Hostschlüssel dürfen nicht über sämtliche
Imageinstallationen vereinheitlicht werden.

Die Hostschlüsselprüfung bleibt aktiviert.
Nach einer Neuinstallation müssen neue Hostschlüssel
vor Nutzung überprüft werden.

## Release-Sicherheit

Das Entwicklungs-SSH-Modul ist standardmäßig
nicht importiert und daher nicht im aktuellen
Image aktiviert.

scripts/novena-release-check.bash ist ein
konservativer Quelltext-Preflight. Er ist weder
eine vollständige Nix-Auswertung noch ein
Freigabenachweis.

Für eine Releasefreigabe sind zusätzlich
die ausgewertete NixOS-Konfiguration und das
fertige Image auf Entwicklungszugänge zu prüfen.

Die aktuelle Konfiguration ist wegen Root-SSH
und des bekannten Initialpassworts nicht
releasefähig.

## Hardwaretest 2026-10-25

- Displayvarianten-SD vollständig geschrieben und
  Byte-für-Byte zurückverifiziert: Block 104C PASS.
- Kaltstart von SD und NixOS-Login: Block 105B/105D.
- Vergleich mit Boot vom 2026-10-24:
  Blöcke 106A und 109.
- Hostschlüssel über serielle Konsole bestätigt:
  Block 107 PASS.
- SSH, Root-Grow und Display-Liveprüfung:
  Block 108 PASS.
- Root auf mmcblk1p2: 59 GiB, ungefähr 54 GiB frei.
- IT6251-Treiber aktiv; stabiler 1920x1080-Link.
- Linux-IT6251-Fehler -6 bleiben sichtbar, aber
  die Initialisierung endet erfolgreich.
- Frühere sichtbare U-Boot-Ausgabe fehlte diesmal;
  Ursache nicht geklärt.
- Novena nach Abschluss kontrolliert ausgeschaltet,
  vom Nutzer bestätigt.

Keine neuen Builds oder Hardwaretests sind
durch dieses Dokument freigegeben.

## Nachtrag 2026-10-25 – Boot-Check-Härtung

Block 113A ergänzt eine maschinenlesbare JSON-Zeile
im vollständigen Runner-TXT-Protokoll.

Schema-Version: 1.
Enthalten: Prüfzeitpunkt, SSH-Ziel, Boot-ID,
Einzelprüfstatus und Gesamtergebnis.

Die Image-Identität bleibt null, bis sie durch
einen eigenen belegbaren Mechanismus bestimmt wird.

Der SSH-Exitstatus und die Vollständigkeit der
Remote-Erfassung werden berücksichtigt.

Serielle Bootausgabe und physische Displaybeobachtung
sind getrennte Evidenz und werden hier ausdrücklich
als NOT_APPLICABLE gekennzeichnet.

Die Releaseprüfung bleibt auch bei bestandener
Quelltext-Vorprüfung gesperrt, bis ausgewertete
NixOS-Konfiguration und fertiges Image zusätzlich
geprüft wurden.

Dieser Stand ist statisch geprüft, aber noch
nicht auf laufender Novena ausgeführt.
