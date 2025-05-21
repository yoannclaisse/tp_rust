use crate::types::{TileType, RobotType, MAP_SIZE};
use crate::map::Map;
use crate::robot::Robot;

pub struct Station {
    pub energy_reserves: u32,
    pub collected_minerals: u32,
    pub collected_scientific_data: u32,
}

impl Station {
    pub fn new() -> Self {
        Self {
            energy_reserves: 100,
            collected_minerals: 0,
            collected_scientific_data: 0,
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
            
            println!("Station: Création d'un nouveau robot de type {:?}", robot_type);
            
            // Créer et retourner le robot
            return Some(Robot::new(map.station_x, map.station_y, robot_type));
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
        
        format!("{} | Création robot: {}/{} énergie, {}/{} minerai", 
                status, 
                self.energy_reserves.min(50), 50,  // Afficher la progression vers l'énergie nécessaire
                self.collected_minerals.min(15), 15)  // Afficher la progression vers les minerais nécessaires
    }
}