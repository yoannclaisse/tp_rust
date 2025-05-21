# EREEA - Essaim de Robots pour l'Exploration et l'Étude Astrobiologique

## Description

EREEA est une simulation d'un essaim de robots autonomes spécialisés pour l'exploration spatiale et la recherche astrobiologique. Ces robots collaborent pour mener des missions d'exploration et de recherche sur des corps célestes afin de recueillir des données sur la géologie, la chimie et les potentiels signes de vie.

## Fonctionnalités

- Génération procédurale de cartes avec obstacles basée sur des algorithmes de bruit
- Différents types de robots spécialisés pour l'exploration, la collecte et l'analyse scientifique
- Système de station centrale pour la coordination et le partage d'informations
- Simulation concurrente du comportement des robots
- Visualisation en temps réel de la simulation dans le terminal
- Gestion des ressources: énergie, minerais et lieux d'intérêt scientifique

## Prérequis

- Rust (édition 2021)
- Cargo

## Installation

```bash
git clone https://github.com/votre-username/ereea.git
cd ereea
cargo build --release
```

## Utilisation

```bash
cargo run --release
```

Options de configuration:
- `-s, --seed <SEED>`: Définir une seed spécifique pour la génération de la carte
- `-w, --width <WIDTH>`: Définir la largeur de la carte
- `-h, --height <HEIGHT>`: Définir la hauteur de la carte
- `-r, --robots <COUNT>`: Définir le nombre initial de robots

## Architecture

Le projet est organisé selon les principes de la programmation modulaire et utilise différents patterns de concurrence en Rust.

Voir les ADRs dans le dossier `docs/adr/` pour plus de détails sur les décisions d'architecture.

## Tests

Pour exécuter les tests:

```bash
cargo test
```

Pour les benchmarks:

```bash
cargo bench
```

## Contributeurs

- [Votre Nom](https://github.com/votre-username)
- [Contributeur 2](https://github.com/contributeur2)
- [Contributeur 3](https://github.com/contributeur3)

## Licence

MIT