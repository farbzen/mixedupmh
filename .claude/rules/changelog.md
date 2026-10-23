# Changelog

- Für sämtliche Anpassungen am Code der Anwendung(en) müssen Einträge im
  Changelog unter `/CHANGELOG.md` hinterlegt werden, die die inhaltliche
  Änderung widerspiegeln. Der Code der Anwendung(en) liegt in der Regel in einem
  der folgenden Verzeichnisse:
  - `/src*`
  - `/app*`
  - `/backend*`
  - `/database*`
  - `/frontend*`
  - `/cli*`

- Anpassungen, welche sich ausschließlich auf die Entwicklungsumgebungen oder
  das Repository selbst auswirken sind vom Changelog ausgenommen. Beispiele
  hierfür sind Änderungen in folgenden Verzeichnissen:
  - `/` (Dateien auf der root-Ebene)
  - `/.assets` (Assets für die Darstellung in der README.md)
  - `/.claude` (CLAUDE Code)
  - `/.gitlab` (GitLab)
  - `/.vscode` (Visual Studio Code)
  - `/docs` (Dokumentation)

- Grundlegende Prinzipien:
  - Changelogs werden für Menschen geschrieben, nicht für Maschinen.
  - Für jede einzelne Version sollte es einen Eintrag geben.
  - Änderungen der selben Art sollten in Bereiche gruppiert werden.
  - Versionen und Bereiche sollten verlinkt werden können.
  - Die neuste Version kommt als erstes.
  - Das Release-Datum jeder Version muss angegeben sein.
  - Gib an, ob du dich an die Semantische Versionierung hältst.

  - Arten von Änderungen
    - `Added` für neue Features.
    - `Changed` für Änderungen an der bestehenden Funktionalität.
    - `Deprecated` für Features, die in zukünftigen Versionen entfernt werden.
    - `Removed` für Deprecated-Features, die in dieser Version entfernt wurden.
    - `Fixed` für alle Bug-Fixes.
    - `Security` um Benutzer im Fall von geschlossenen Sicherheitslücken zu
      einer Aktualisierung aufzufordern.

  - Halte Anpassungen, die noch keiner neuen Version fest zugeordnet sind in
    einem dedizierten `Unreleased`-Abschnitt vor.
