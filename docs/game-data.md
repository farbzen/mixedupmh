# Maintaining game data

Monsters, weapons and translations are stored as JSON under `json/`, the
matching images under `static/images/`. In development mode (`cargo tauri dev`)
the files are re-read on every roll; installers require a new build.

## Monsters

Entry in `json/monsters.json`:

```json
{
  "name": "rathalos",
  "capturable": true,
  "tempered": true,
  "arch_tempered": false
}
```

| Field           | Effect                                                         |
| --------------- | -------------------------------------------------------------- |
| `name`          | Key for image and translation                                  |
| `capturable`    | `true`: a 50/50 roll decides between "capture" and "slay"      |
| `tempered`      | `true`: tier 0 (normal) or 1 (tempered) is rolled              |
| `arch_tempered` | `true`: tier 2 (arch-tempered) is possible as well             |

Every possible tier needs an image
`static/images/monsters/<tier>_<name>.webp`, i.e. at least `0_<name>.webp`,
plus `1_<name>.webp` for tempered monsters and `2_<name>.webp` for
arch-tempered ones.

## Weapons

Entry `{ "name": "<name>" }` in `json/weapons.json` and image
`static/images/weapons/<name>.webp`.

## Translations

Every new `name` needs a key in `json/language/en.json` and
`json/language/de.json`. If a key is missing, the UI shows the key itself.

Adding another language requires, besides the JSON file and a flag at
`static/images/languages/<code>.webp`, changes in the code
(`LANGUAGES_SUPPORTED` in `src/data/language.rs`) and in the footer template
(`templates/components/footer.html`).
