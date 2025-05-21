use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum TileType {
    Empty,
    Obstacle,
    Energy,
    Mineral,
    Scientific,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum RobotType {
    Explorer,          // Explore et cartographie le terrain
    EnergyCollector,   // Collecte de l'énergie
    MineralCollector,  // Collecte des minerais
    ScientificCollector, // Collecte des données scientifiques
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum RobotMode {
    Exploring,        // Exploration active
    Collecting,       // Collecte des ressources
    ReturnToStation,  // Retour à la station
    Idle,             // En attente à la station
}

pub const MAP_SIZE: usize = 20;