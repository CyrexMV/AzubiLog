# AzubiLog

AzubiLog ist ein responsives, installierbares Web-Berichtsheft für Auszubildende.

## Funktionen
- Azubi-Profil mit Name, Ausbildungsberuf, Betrieb, Ausbildungsjahr, Ausbilder und Beginn
- Tagesberichte mit Datum, Stunden, Tätigkeit, Beschreibung, Ausbildungsart und Abteilung
- Wochenübersicht Montag bis Sonntag
- Berichte bearbeiten und löschen
- PDF-Wochenbericht mit Profil, Tätigkeiten, Gesamtstunden und Unterschriftsfeldern
- CSV-Export aller Einträge
- Speicherung lokal im Browser (localStorage)
- PWA-Grundgerüst mit Service Worker und Web-App-Manifest

## Starten
GitHub Pages aktivieren: Repository → Settings → Pages → Deploy from a branch → `main` → `/ (root)`.

## Datenschutz
Die erste Version speichert Profil- und Berichtsdaten lokal im Browser. Es gibt keine Registrierung, keinen Server und keine Synchronisierung zwischen Geräten. Für Mehrbenutzerkonten mit Cloud-Synchronisierung wird ein Backend mit Authentifizierung und Datenbank benötigt.

## Technologien
HTML, CSS und Vanilla JavaScript; jsPDF wird für den PDF-Export über jsDelivr eingebunden.
