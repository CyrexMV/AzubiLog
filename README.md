# AzubiLog

**Das moderne digitale Berichtsheft für Auszubildende.** AzubiLog verbindet eine hochwertige, responsive Oberfläche mit einem lokalen Rust-Backend und einer SQLite-Datenbank.

## Funktionen
- Ausbildungsprofil mit Name, Beruf, Betrieb, Ausbildungsjahr und Ausbildungsbeginn
- Tagesberichte mit Datum, Tätigkeiten, Stunden, Bereich und Notizen
- Wochenübersicht mit Kalenderwoche, Eintragszahl und dokumentierten Stunden
- Tagesberichte bearbeiten und löschen
- Druckoptimierter Wochenbericht – im Browser als PDF speichern
- Datenhaltung in SQLite auf dem eigenen Rechner
- Responsive Design für Desktop, Tablet und Smartphone

## Technologie
- **Backend:** Rust, Axum, Tokio
- **Datenbank:** SQLite über rusqlite
- **Frontend:** HTML, modernes CSS und Vanilla JavaScript
- **API:** JSON unter `/api`

## Voraussetzungen
- Rust stable (Edition 2021)

## Lokal starten
```bash
git clone https://github.com/CyrexMV/AzubiLog.git
cd AzubiLog
cargo run --release
```
Öffne anschließend **http://127.0.0.1:3000** im Browser.

Beim ersten Start legt AzubiLog automatisch die Datei `azubilog.db` im Arbeitsverzeichnis an. Diese enthält das Profil und die Tagesberichte.

## Wochenbericht als PDF
Wähle die gewünschte Kalenderwoche und klicke auf **PDF exportieren**. Im Druckdialog deines Browsers wählst du „Als PDF speichern“.

## Datenschutz & Einsatz
Diese Version ist für die lokale Nutzung auf einem Rechner konzipiert. Die Datenbank liegt lokal und es gibt keine Übertragung an einen Cloud-Dienst. Mehrbenutzerkonten, Anmeldung und geräteübergreifende Synchronisierung sind derzeit nicht enthalten. Für öffentliches Hosting müssen Authentifizierung, HTTPS, Zugriffsschutz und ein passendes Datenschutzkonzept ergänzt werden.

## Lizenz
MIT
