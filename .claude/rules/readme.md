# README-Handhabung

- Betrachte das gesamte Projekt innerhalb des Repositories und schreibe eine
  `README.md`, die es vollständig im Überblick abdeckt (alle Apps, Features,
  Schnellstart, wichtigste Befehle), aber knapp bleibt.
- Details wie vollständige Listen von Umgebungsvariablen,
  Schnittstellenprotokolle, Test- und Werkzeugdetails oder Betriebs- und
  Notfallanleitungen gehören in eigene Dateien unter `docs/` und werden aus der
  README nur verlinkt.
  Passt keine, eine neue Datei unter `docs/` anlegen und unter „Dokumentation“
  verlinken; Unterprojekte mit eigener README (z. B. `examples/*`) pflegen ihre
  Details dort.
- Sollte bereits eine `README.md` vorhanden sein, die über eine bloße Vorlage
  hinausgeht (Beispiel: Inhalt beschränkt sich auf "# VORLAGE"), ergänze die
  vorhandene `README.md` um neue Apps, Befehle und Verweise; Details in die
  passende Datei unter `docs/`.
- Da die Themen `LICENSE`, `CHANGELOG` und `CONTRIBUTING` in eigenen Dateien
  behandelt werden, darf maximal auf diese Dateien verwiesen werden.

## README-Struktur

Destilliert aus den Vorbildern:

- Azure-Samples/serverless-chat-langchainjs
- Azure-Samples/serverless-recipes-javascript
- sinedied/run-on-output
- sinedied/smoke

## Was die Vorbilder gemeinsam haben

- **Zentrierter Kopf**: Titel, ein Satz Tagline, Badges, Sprungnavigation
  (`[Überblick](#überblick) • [Schnellstart](#schnellstart) • …`).
- **Ein Absatz direkt nach dem Kopf**, der sagt, was das Projekt ist und für wen.
- **Überblick mit Komponentenliste**: je Komponente ein Stichpunkt mit
  Technologie (verlinkt) und Verzeichnis (`Der Code liegt in packages/api`).
- **Features** als fettgedruckte Stichwörter mit einem Halbsatz Nutzen.
- **Getting Started** mit Voraussetzungen und mehreren Wegen in die
  Entwicklungsumgebung, danach **Run/Usage** mit kopierbaren Befehlsblöcken.
- **GitHub-Hinweisboxen** (`> [!TIP]`, `> [!NOTE]`, `> [!IMPORTANT]`) für
  Stolpersteine statt Fließtext.
- **Resources / Troubleshooting / Getting Help** am Ende, Contributing und
  License nur als kurzer Verweis.

## Gliederung für dieses Monorepo

Abschnitte ohne Inhalt im Repo weglassen statt mit Füllsätzen zu bestücken.

~~~md
<div align="center">

# <Projektname>

_<Tagline in einem Satz>_

[![Version](https://img.shields.io/badge/version-<X.Y.Z>-blue?style=flat-square)](CHANGELOG.md)
<!-- weitere Badges nur für tatsächlich genutzte Technologien -->

[Überblick](#überblick) • [Schnellstart](#schnellstart) • [Anwendungen](#anwendungen) • [Entwicklung](#entwicklung) • [Dokumentation](#dokumentation)

</div>

<Ein Absatz: Was ist das Projekt, für wen, welches Problem löst es.>

## Überblick

<Wofür das Projekt da ist, ein bis zwei Absätze.>

Das Monorepo besteht aus folgenden Anwendungen:

- **<app>** – <Zweck>, gebaut mit [<Technologie>](<url>). Der Code liegt in `<verzeichnis>/`.
- …

## Features

- **<Stichwort>**: <Nutzen in einem Halbsatz>.

## Schnellstart

### Voraussetzungen

- [devenv](https://devenv.sh) (stellt Python, Node, MariaDB … bereit)

### Entwicklungsumgebung

```sh
git clone <repo>
cd <repo>
devenv shell
```

> [!TIP]
> <Stolperstein oder Abkürzung, falls vorhanden>

## Anwendungen

### <app>

<Zweck, Starten, nur die wichtigste Konfiguration; Details per Link nach docs/>

```sh
<Startbefehl>
```

## Entwicklung

### Tests

```sh
<Testbefehl je App>
```

### Code-Qualität

<Linter/Formatter/Typprüfung mit Befehlen, aus ruff.toml, pyrightconfig.json usw.>

### API-Dokumentation

<z. B. Doxygen, OpenAPI unter /docs – nur wenn vorhanden>

## Deployment

<Kurzbeschreibung und Verweis auf docs/deployment.md – Abschnitt nicht
entfernen, Meldungen der Deploy-Skripte verweisen auf „README (section
Deployment)“>

## Dokumentation

- [<Thema>](docs/<datei>.md)
- [Architecture Decision Records](docs/adr/)
- [Changelog](CHANGELOG.md)

## Mitwirken

Siehe [CONTRIBUTING.md](CONTRIBUTING.md).

## Lizenz

Siehe [LICENSE](LICENSE).
~~~

## Hinweise

- **Changelog, Contributing, Lizenz** stehen in eigenen Dateien. In der README
  nur verlinken – kopierte Inhalte laufen sonst auseinander. Existiert eine
  der Dateien nicht, den Verweis weglassen.
- **Befehle** nur aufnehmen, wenn sie sich aus dem Repo belegen lassen
  (`devenv.nix`, `pyproject.toml`, `package.json`, `pytest.ini`, …).
- **Badges** nur für Fakten, die im Repo stehen (Version aus `/version`,
  eingesetzte Sprachen/Frameworks). Keine Build-Badges ohne bekannte CI-URL.
- **Bilder/Logos** nur referenzieren, wenn die Datei existiert (z. B. unter
  `.assets/`).
