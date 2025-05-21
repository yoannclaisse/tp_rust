mod types;
mod map;
mod robot;
mod display;
mod station; // Nouveau module

use std::{thread, time::Duration};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use types::{MAP_SIZE, RobotType};
use map::Map;
use robot::Robot;
use display::Display;
use station::Station;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configuration du terminal
    enable_raw_mode()?;
    
    // Initialisation avec carte aléatoire
    let mut map = Map::new();
    let mut station = Station::new();
    
    // Créer différents robots
    let mut robots = vec![
        Robot::new(map.station_x, map.station_y, RobotType::Explorer),
        Robot::new(map.station_x, map.station_y, RobotType::EnergyCollector),
        Robot::new(map.station_x, map.station_y, RobotType::MineralCollector),
        Robot::new(map.station_x, map.station_y, RobotType::ScientificCollector),
    ];
    
    // S'assurer que tous les robots sont en mode exploration
    for robot in robots.iter_mut() {
        robot.mode = types::RobotMode::Exploring;
    }
    
    // Boucle principale
    let mut iteration = 0;
    let mut last_robot_creation = 0;
    
    loop {
        // Affichage
        Display::render(&map, &station, &robots)?;
        
        // Mise à jour des robots
        for robot in robots.iter_mut() {
            robot.update(&mut map, &mut station);
            
            // Si le robot est à court d'énergie, le ramener à la station
            if robot.energy <= 0.0 {
                robot.x = map.station_x;
                robot.y = map.station_y;
                robot.energy = robot.max_energy / 2.0;
                robot.mode = types::RobotMode::Idle;
            }
        }
        
        // Vérifier si la station peut créer un nouveau robot (tous les 50 cycles)
        if iteration - last_robot_creation >= 50 {
            if let Some(new_robot) = station.try_create_robot(&map) {
                robots.push(new_robot);
                last_robot_creation = iteration;
            }
        }
        
        // Attendre
        thread::sleep(Duration::from_millis(300));
        iteration += 1;
        
        // Pour quitter
        if iteration > 1000 {
            break;
        }
    }
    
    // Restaurer le terminal
    disable_raw_mode()?;
    Ok(())
}