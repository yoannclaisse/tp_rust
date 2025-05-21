use crate::types::{TileType, RobotType, MAP_SIZE};
use crate::map::Map;
use crate::robot::Robot;
use std::collections::HashMap;

// Structure pour représenter une donnée de terrain avec timestamp
#[derive(Clone)]
pub struct TerrainData {
    pub explored: bool,
    pub timestamp: u32,     // Quand la case a été explorée
    pub robot_id: usize,    // Quel robot a exploré cette case
    pub robot_type: RobotType, // Type du robot qui a exploré cette case
}

pub struct Station {
    pub energy_reserves: u32,
    pub collected_minerals: u32,
    pub collected_scientific_data: u32,
    pub global_memory: Vec<Vec<TerrainData>>, // Mémoire partagée globale
    pub conflict_count: usize,  // Nombre de conflits résolus
    pub next_robot_id: usize,   // ID pour le prochain robot créé
    pub current_time: u32,      // Horloge globale de la simulation
}

impl Station {
    pub fn new() -> Self {
        // Initialiser la mémoire globale avec des données non explorées
        let mut global_memory = Vec::with_capacity(MAP_SIZE);
        for _ in 0..MAP_SIZE {
            let row = vec![
                TerrainData {
                    explored: false,
                    timestamp: 0,
                    robot_id: 0,
                    robot_type: RobotType::Explorer,
                }; 
                MAP_SIZE
            ];
            global_memory.push(row);
        }
        
        Self {
            energy_reserves: 100,
            collected_minerals: 0,
            collected_scientific_data: 0,
            global_memory,
            conflict_count: 0,
            next_robot_id: 1,  // Les ID commencent à 1
            current_time: 0,
        }
    }
    
    // Incrémente l'horloge globale
    pub fn tick(&mut self) {
        self.current_time += 1;
    }
    
    // Partage des connaissances entre un robot et la station (façon git)
    pub fn share_knowledge(&mut self, robot: &mut Robot) {
        // Uniquement si le robot est à la station
        if robot.x == robot.home_station_x && robot.y == robot.home_station_y {
            let mut conflicts = 0;
            
            // Pour chaque case de la carte
            for y in 0..MAP_SIZE {
                for x in 0..MAP_SIZE {
                    // Si le robot a exploré cette case
                    if robot.memory[y][x].explored {
                        // Vérifier si la station a déjà des données sur cette case
                        if self.global_memory[y][x].explored {
                            // CONFLIT: La case est connue par la station et le robot
                            // Résoudre en fonction des timestamps
                            if robot.memory[y][x].timestamp > self.global_memory[y][x].timestamp {
                                // Les données du robot sont plus récentes -> Mettre à jour la station
                                self.global_memory[y][x] = robot.memory[y][x].clone();
                                conflicts += 1;
                            }
                            // Sinon, garder les données de la station (plus récentes)
                        } else {
                            // Pas de conflit, ajouter les connaissances du robot
                            self.global_memory[y][x] = robot.memory[y][x].clone();
                        }
                    }
                }
            }
            
            // Le robot récupère toutes les connaissances de la station
            for y in 0..MAP_SIZE {
                for x in 0..MAP_SIZE {
                    if self.global_memory[y][x].explored {
                        robot.memory[y][x] = self.global_memory[y][x].clone();
                    }
                }
            }
            
            // Mettre à jour les statistiques
            self.conflict_count += conflicts;
            
            println!("Robot {} a partagé ses connaissances. Conflits résolus: {}", robot.id, conflicts);
        }
    }
    
    // Vérifie si la station peut créer un nouveau robot et le fait si possible
    pub fn try_create_robot(&mut self, map: &Map) -> Option<Robot> {
        // Coûts de création d'un robot
        let energy_cost = 50;
        let mineral_cost = 15;
        
        // Vérifier si on a assez de ressources
        if self.energy_reserves >= energy_cost && self.collected_minerals >= mineral_cost {
            // Déterminer le type de robot à créer
            let robot_type = self.determine_needed_robot_type(map);
            
            // Consommer les ressources
            self.energy_reserves -= energy_cost;
            self.collected_minerals -= mineral_cost;
            
            // Créer et retourner le robot
            let new_robot_id = self.next_robot_id;
            self.next_robot_id += 1;
            
            println!("Station: Création d'un nouveau robot #{} de type {:?}", new_robot_id, robot_type);
            
            // Initialiser la mémoire du robot avec les connaissances de la station
            let memory = self.global_memory.clone();
            
            return Some(Robot::new_with_memory(
                map.station_x, 
                map.station_y, 
                robot_type, 
                new_robot_id,
                map.station_x, 
                map.station_y,
                memory
            ));
        }
        
        None
    }
    
    // Détermine le type de robot le plus nécessaire actuellement
    fn determine_needed_robot_type(&self, map: &Map) -> RobotType {
        // Compter le nombre de ressources restantes sur la carte
        let mut energy_count = 0;
        let mut mineral_count = 0;
        let mut scientific_count = 0;
        
        for y in 0..MAP_SIZE {
            for x in 0..MAP_SIZE {
                match map.get_tile(x, y) {
                    TileType::Energy => energy_count += 1,
                    TileType::Mineral => mineral_count += 1,
                    TileType::Scientific => scientific_count += 1,
                    _ => {}
                }
            }
        }
        
        // Si nous avons très peu d'énergie, priorité à un collecteur d'énergie
        if energy_count > 0 && (energy_count <= 3 || self.energy_reserves < 100) {
            return RobotType::EnergyCollector;
        }
        
        // Si nous avons peu de minerais mais assez d'énergie, créer un collecteur de minerais
        if mineral_count > 0 && (mineral_count <= 5 || self.collected_minerals < 30) {
            return RobotType::MineralCollector;
        }
        
        // Si nous avons des points d'intérêt scientifique et assez de ressources
        if scientific_count > 0 && self.energy_reserves >= 100 {
            return RobotType::ScientificCollector;
        }
        
        // Par défaut, si la carte n'est pas bien explorée ou s'il reste peu de ressources,
        // créer un robot explorateur
        RobotType::Explorer
    }
    
    // Déposer des ressources à la station
    pub fn deposit_resources(&mut self, minerals: u32, scientific_data: u32) {
        self.collected_minerals += minerals;
        self.collected_scientific_data += scientific_data;
        self.energy_reserves += minerals; // Convertir des minerais en énergie
    }
    
    // Méthode pour évaluer les besoins actuels de la station
    pub fn get_status(&self) -> String {
        let status = match (self.energy_reserves, self.collected_minerals) {
            (e, m) if e < 30 => "Faible en énergie",
            (e, m) if m < 10 => "Faible en minerais",
            (e, m) if e >= 200 && m >= 50 => "Ressources abondantes",
            _ => "Ressources adéquates",
        };
        
        format!("{} | Création robot: {}/{} énergie, {}/{} minerai | Conflits: {}", 
                status, 
                self.energy_reserves.min(50), 50,  // Afficher la progression vers l'énergie nécessaire
                self.collected_minerals.min(15), 15,  // Afficher la progression vers les minerais nécessaires
                self.conflict_count)  // Afficher les conflits résolus
    }
    
    // Calculer le pourcentage de la carte exploré
    pub fn get_exploration_percentage(&self) -> f32 {
        let mut explored_count = 0;
        
        for y in 0..MAP_SIZE {
            for x in 0..MAP_SIZE {
                if self.global_memory[y][x].explored {
                    explored_count += 1;
                }
            }
        }
        
        (explored_count as f32 / (MAP_SIZE * MAP_SIZE) as f32) * 100.0
    }
}