use std::io::{stdout, Write, Result};
use crossterm::{
    ExecutableCommand,
    terminal::{Clear, ClearType},
    cursor::MoveTo,
    style::{Color, SetForegroundColor},
};
use crate::types::{TileType, MAP_SIZE, RobotType, RobotMode};
use crate::map::Map;
use crate::robot::Robot;
use crate::station::Station;

pub struct Display;

impl Display {
    pub fn render(map: &Map, station: &Station, robots: &Vec<Robot>) -> Result<()> {
        let mut stdout = stdout();
        
        // Effacer l'écran
        stdout.execute(Clear(ClearType::All))?;
        
        // Afficher la carte
        for y in 0..MAP_SIZE {
            for x in 0..MAP_SIZE {
                stdout.execute(MoveTo(x as u16 * 2, y as u16))?;
                
                // Vérifier si un robot est sur cette case
                let robot_here = robots.iter().find(|r| r.x == x && r.y == y);
                
                if x == map.station_x && y == map.station_y {
                    stdout.execute(SetForegroundColor(Color::Yellow))?;
                    print!("[]");
                } else if let Some(robot) = robot_here {
                    // Afficher le robot avec sa couleur
                    stdout.execute(SetForegroundColor(Color::AnsiValue(robot.get_display_color())))?;
                    print!("{}:", robot.get_display_char());
                } else {
                    match map.get_tile(x, y) {
                        TileType::Empty => {
                            stdout.execute(SetForegroundColor(Color::White))?;
                            print!("· ");
                        },
                        TileType::Obstacle => {
                            stdout.execute(SetForegroundColor(Color::DarkGrey))?;
                            print!("██");
                        },
                        TileType::Energy => {
                            stdout.execute(SetForegroundColor(Color::Green))?;
                            print!("♦ ");
                        },
                        TileType::Mineral => {
                            stdout.execute(SetForegroundColor(Color::Magenta))?;
                            print!("★ ");
                        },
                        TileType::Scientific => {
                            stdout.execute(SetForegroundColor(Color::Blue))?;
                            print!("○ ");
                        },
                    }
                }
            }
            println!();
        }
        
        // Afficher les informations de la station
        stdout.execute(MoveTo(0, MAP_SIZE as u16 + 1))?;
        stdout.execute(SetForegroundColor(Color::Yellow))?;
        println!("Station: Minerais: {} | Données Scientifiques: {} | Énergie: {}", 
                station.collected_minerals, 
                station.collected_scientific_data,
                station.energy_reserves);
        
        // Ajouter le statut de la station
        stdout.execute(MoveTo(0, MAP_SIZE as u16 + 2))?;
        stdout.execute(SetForegroundColor(Color::White))?;
        println!("Statut: {}", station.get_status());
        
        // Afficher les informations de chaque robot
        for (i, robot) in robots.iter().enumerate() {
            stdout.execute(MoveTo(0, MAP_SIZE as u16 + 4 + i as u16))?;
            stdout.execute(SetForegroundColor(Color::AnsiValue(robot.get_display_color())))?;
            
            let robot_type = match robot.robot_type {
                RobotType::Explorer => "Explorateur",
                RobotType::EnergyCollector => "Collecteur d'énergie",
                RobotType::MineralCollector => "Collecteur de minerais",
                RobotType::ScientificCollector => "Collecteur scientifique",
            };
            
            let mode = match robot.mode {
                RobotMode::Exploring => "Exploration",
                RobotMode::Collecting => "Collecte",
                RobotMode::ReturnToStation => "Retour",
                RobotMode::Idle => "Inactif",
            };
            
            println!("Robot {}: {} | Énergie: {:.1}/{:.1} | Mode: {} | Min: {} | Sci: {}", 
                    i + 1, robot_type, robot.energy, robot.max_energy, mode, 
                    robot.minerals, robot.scientific_data);
        }
        
        // Afficher la légende
        let legend_y = MAP_SIZE as u16 + 4 + robots.len() as u16 + 1;
        stdout.execute(MoveTo(0, legend_y))?;
        stdout.execute(SetForegroundColor(Color::White))?;
        println!("Légende :");
        
        stdout.execute(MoveTo(0, legend_y + 1))?;
        stdout.execute(SetForegroundColor(Color::Yellow))?;
        print!("[] = Station   ");
        
        stdout.execute(SetForegroundColor(Color::AnsiValue(9)))?;
        print!("E: = Explorateur   ");
        
        stdout.execute(SetForegroundColor(Color::AnsiValue(10)))?;
        print!("P: = Collecteur d'énergie   ");
        
        stdout.execute(SetForegroundColor(Color::AnsiValue(13)))?;
        print!("M: = Collecteur de minerais   ");
        
        stdout.execute(SetForegroundColor(Color::AnsiValue(12)))?;
        println!("S: = Collecteur scientifique");
        
        stdout.execute(MoveTo(0, legend_y + 2))?;
        stdout.execute(SetForegroundColor(Color::Green))?;
        print!("♦ = Énergie   ");
        
        stdout.execute(SetForegroundColor(Color::Magenta))?;
        print!("★ = Minerai   ");
        
        stdout.execute(SetForegroundColor(Color::Blue))?;
        print!("○ = Intérêt scientifique   ");
        
        stdout.execute(SetForegroundColor(Color::DarkGrey))?;
        println!("██ = Obstacle");
        
        stdout.flush()?;
        Ok(())
    }
}