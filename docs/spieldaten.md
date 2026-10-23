# Spieldaten pflegen

Monster, Waffen und Übersetzungen liegen als JSON unter `json/`, die passenden
Bilder unter `static/images/`. Im Entwicklungsmodus (`cargo tauri dev`) werden
die Dateien bei jedem Wurf neu gelesen; für Installationspakete ist ein neuer
Build nötig.

## Monster

Eintrag in `json/monsters.json`:

```json
{
  "name": "rathalos",
  "capturable": true,
  "tempered": true,
  "arch_tempered": false
}
```

| Feld            | Wirkung                                                       |
| --------------- | ------------------------------------------------------------- |
| `name`          | Schlüssel für Bild und Übersetzung                            |
| `capturable`    | `true`: Jagdziel wird per 50/50-Wurf „Fangen“ oder „Töten“    |
| `tempered`      | `true`: Stufe 0 (normal) oder 1 (gehärtet) wird ausgelost     |
| `arch_tempered` | `true`: zusätzlich Stufe 2 (erzgehärtet) möglich              |

Für jede mögliche Stufe wird ein Bild
`static/images/monsters/<stufe>_<name>.webp` benötigt, also mindestens
`0_<name>.webp`, bei gehärteten Monstern zusätzlich `1_<name>.webp` und bei
erzgehärteten `2_<name>.webp`.

## Waffen

Eintrag `{ "name": "<name>" }` in `json/weapons.json` und Bild
`static/images/weapons/<name>.webp`.

## Übersetzungen

Jeder neue `name` braucht einen Schlüssel in `json/language/en.json` und
`json/language/de.json`. Fehlt ein Schlüssel, zeigt die Oberfläche den
Schlüssel selbst an.

Eine weitere Sprache erfordert neben der JSON-Datei und einer Flagge unter
`static/images/languages/<kürzel>.webp` Anpassungen im Code
(`LANGUAGES_SUPPORTED` in `src/data/language.rs`) und im Footer-Template
(`templates/components/footer.html`).
