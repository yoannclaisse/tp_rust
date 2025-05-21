// src/network/mod.rs
use serde::{Serialize, Deserialize};
use crate::types::{MAP_SIZE, TileType, RobotType, RobotMode};

// Structure pour représenter les données sur la carte
#[derive(Serialize, Deserialize, Clone)]
pub struct MapData {
    pub tiles: Vec<Vec<TileType>>,
    pub station_x: usize,
    pub station_y: usize,
}

// Structure pour représenter les données d'un robot
#[derive(Serialize, Deserialize, Clone)]
pub struct RobotData {
    pub id: usize,
    pub x: usize,
    pub y: usize,
    pub energy: f32,
    pub max_energy: f32,
    pub minerals: u32,
    pub scientific_data: u32,
    pub robot_type: RobotType,
    pub mode: RobotMode,
    pub exploration_percentage: f32,
}

// Structure pour représenter les données de la station
#[derive(Serialize, Deserialize, Clone)]
pub struct StationData {
    pub energy_reserves: u32,
    pub collected_minerals: u32,
    pub collected_scientific_data: u32,
    pub exploration_percentage: f32,
    pub conflict_count: usize,
    pub robot_count: usize,
    pub status_message: String,
}

// Structure pour représenter les données d'exploration
#[derive(Serialize, Deserialize, Clone)]
pub struct ExplorationData {
    pub explored_tiles: Vec<Vec<bool>>,
}

// Structure pour représenter l'état complet de la simulation
#[derive(Serialize, Deserialize, Clone)]
pub struct SimulationState {
    pub map_data: MapData,
    pub robots_data: Vec<RobotData>,
    pub station_data: StationData,
    pub exploration_data: ExplorationData,
    pub iteration: u32,
}

// Port pour la communication TCP
pub const DEFAULT_PORT: u16 = 8081;

// Constantes pour la taille des messages
pub const MAX_MESSAGE_SIZE: usize = 1024 * 1024; // 1 MB

// Fonction utilitaire pour créer une structure MapData à partir de Map
pub fn create_map_data(map: &crate::map::Map) -> MapData {
    MapData {
        tiles: map.tiles.clone(),
        station_x: map.station_x,
        station_y: map.station_y,
    }
}

// Fonction utilitaire pour créer une structure RobotData à partir de Robot
pub fn create_robot_data(robot: &crate::robot::Robot) -> RobotData {
    RobotData {
        id: robot.id,
        x: robot.x,
        y: robot.y,
        energy: robot.energy,
        max_energy: robot.max_energy,
        minerals: robot.minerals,
        scientific_data: robot.scientific_data,
        robot_type: robot.robot_type,
        mode: robot.mode,
        exploration_percentage: robot.get_exploration_percentage(),
    }
}

// Fonction utilitaire pour créer une structure StationData à partir de Station
pub fn create_station_data(station: &crate::station::Station) -> StationData {
    StationData {
        energy_reserves: station.energy_reserves,
        collected_minerals: station.collected_minerals,
        collected_scientific_data: station.collected_scientific_data,
        exploration_percentage: station.get_exploration_percentage(),
        conflict_count: station.conflict_count,
        robot_count: station.next_robot_id - 1, // Estimation du nombre de robots
        status_message: station.get_status(),
    }
}

// Fonction utilitaire pour créer une structure ExplorationData à partir de Station
pub fn create_exploration_data(station: &crate::station::Station) -> ExplorationData {
    let mut explored_tiles = vec![vec![false; MAP_SIZE]; MAP_SIZE];
    
    for y in 0..MAP_SIZE {
        for x in 0..MAP_SIZE {
            explored_tiles[y][x] = station.global_memory[y][x].explored;
        }
    }
    
    ExplorationData {
        explored_tiles,
    }
}

// Fonction utilitaire pour créer l'état complet de la simulation
pub fn create_simulation_state(map: &crate::map::Map, station: &crate::station::Station, 
                              robots: &Vec<crate::robot::Robot>, iteration: u32) -> SimulationState {
    let map_data = create_map_data(map);
    
    let mut robots_data = Vec::with_capacity(robots.len());
    for robot in robots {
        robots_data.push(create_robot_data(robot));
    }
    
    let station_data = create_station_data(station);
    let exploration_data = create_exploration_data(station);
    
    SimulationState {
        map_data,
        robots_data,
        station_data,
        exploration_data,
        iteration,
    }
}