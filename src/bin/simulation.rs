// src/bin/simulation.rs
use ereea::types::{MAP_SIZE, RobotType, RobotMode};
use ereea::map::Map;
use ereea::robot::Robot;
use ereea::station::Station;
use ereea::network::{SimulationState, DEFAULT_PORT, create_simulation_state};

use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncWriteExt;
use tokio::sync::{mpsc, Mutex as TokioMutex};
use std::process;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Démarrage du serveur de simulation EREEA...");
    
    // Initialisation de la simulation
    println!("Étape 1: Initialisation de la carte...");
    let map = Arc::new(Mutex::new(Map::new()));
    println!("Carte initialisée avec succès.");
    
    println!("Étape 2: Initialisation de la station...");
    let station = Arc::new(Mutex::new(Station::new()));
    println!("Station initialisée avec succès.");
    
    // Extraction des valeurs nécessaires pour créer les robots
    println!("Étape 3: Préparation des données pour la création des robots...");
    let station_x;
    let station_y;
    let global_memory_clone;
    
    {
        let map_lock = map.lock().unwrap();
        let station_lock = station.lock().unwrap();
        
        station_x = map_lock.station_x;
        station_y = map_lock.station_y;
        global_memory_clone = station_lock.global_memory.clone();
        
        println!("Données préparées avec succès: station à ({}, {})", station_x, station_y);
    }
    
    // Création des robots initiaux sans verrouillage répété
    println!("Étape 3b: Création des robots initiaux...");
    let robots = Arc::new(Mutex::new(vec![
        Robot::new_with_memory(
            station_x, station_y, 
            RobotType::Explorer, 1,
            station_x, station_y,
            global_memory_clone.clone()
        ),
        Robot::new_with_memory(
            station_x, station_y, 
            RobotType::EnergyCollector, 2,
            station_x, station_y,
            global_memory_clone.clone()
        ),
        Robot::new_with_memory(
            station_x, station_y, 
            RobotType::MineralCollector, 3,
            station_x, station_y,
            global_memory_clone.clone()
        ),
        Robot::new_with_memory(
            station_x, station_y, 
            RobotType::ScientificCollector, 4,
            station_x, station_y,
            global_memory_clone.clone()
        ),
    ]));
    println!("Robots créés avec succès.");
    
    // Initialiser l'ID du prochain robot
    println!("Étape 4: Configuration de l'ID du prochain robot...");
    station.lock().unwrap().next_robot_id = 5;
    
    // S'assurer que tous les robots sont en mode exploration
    println!("Étape 5: Définition du mode initial des robots...");
    for robot in robots.lock().unwrap().iter_mut() {
        robot.mode = RobotMode::Exploring;
    }
    println!("Mode des robots configuré avec succès.");
    
    // Canal pour envoyer l'état de la simulation aux clients connectés
    println!("Étape 6: Configuration des canaux de communication...");
    let (state_tx, mut state_rx) = mpsc::channel::<SimulationState>(100);
    println!("Canaux de communication configurés avec succès.");
    
    // Canal pour signaler la fin de mission
    let (mission_complete_tx, mut mission_complete_rx) = mpsc::channel::<bool>(1);
    
    // Thread de simulation
    println!("Étape 7: Démarrage du thread de simulation...");
    let map_for_sim = map.clone();
    let station_for_sim = station.clone();
    let robots_for_sim = robots.clone();
    
    println!("Clonage des références effectué avec succès.");
    
    let simulation_thread = thread::spawn(move || {
        println!("Thread de simulation démarré.");
        let mut iteration = 0;
        let mut last_robot_creation = 0;
        let mut mission_complete = false;
        
        // Seuil d'exploration pour considérer la mission comme réussie (95%)
        const EXPLORATION_THRESHOLD: f32 = 95.0;
        
        loop {
            if iteration % 10 == 0 {
                println!("Simulation: Itération {}", iteration);
            }
            
            // Incrémentation de l'horloge de la station
            match station_for_sim.lock() {
                Ok(mut station_lock) => {
                    station_lock.tick();
                    
                    // Vérifier si la mission est complète
                    let exploration_percentage = station_lock.get_exploration_percentage();
                    if exploration_percentage >= EXPLORATION_THRESHOLD && !mission_complete {
                        println!("\n!!! EXPLORATION COMPLÈTE À {:.1}% !!!\n", exploration_percentage);
                        println!("Rappel des robots à la station...");
                        mission_complete = true;
                        
                        // Rappeler tous les robots à la station sans appeler la méthode privée
                        if let Ok(mut robots_lock) = robots_for_sim.lock() {
                            for robot in robots_lock.iter_mut() {
                                // Simplement changer le mode - la méthode update s'occupera du reste
                                robot.mode = RobotMode::ReturnToStation;
                            }
                        }
                    }
                    
                    // Si tous les robots sont à la station après que la mission est complète, 
                    // envoyer le signal de fin de mission
                    if mission_complete {
                        let all_robots_at_station = {
                            if let Ok(robots_lock) = robots_for_sim.lock() {
                                robots_lock.iter().all(|r| r.x == r.home_station_x && r.y == r.home_station_y)
                            } else {
                                false
                            }
                        };
                        
                        if all_robots_at_station {
                            println!("\n!!! MISSION ACCOMPLIE !!!\n");
                            println!("Tous les robots sont de retour à la station.");
                            println!("EXOPLANÈTE ENTIÈREMENT DÉCOUVERTE À VOUS LA TERRE !!!");
                            println!("Transmission des données finales...");
                            
                            // Notifier la boucle principale que la mission est terminée
                            if let Err(e) = mission_complete_tx.blocking_send(true) {
                                eprintln!("Erreur lors de l'envoi du signal de fin de mission: {:?}", e);
                            }
                            
                            // Attendre un peu pour s'assurer que le message est envoyé
                            thread::sleep(Duration::from_secs(2));
                            break;
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Erreur lors du verrouillage de la station: {:?}", e);
                    break;
                }
            }
            
            // Mise à jour des robots
            {
                let robots_result = robots_for_sim.lock();
                let map_result = map_for_sim.lock();
                let station_result = station_for_sim.lock();
                
                match (robots_result, map_result, station_result) {
                    (Ok(mut robots_lock), Ok(mut map_lock), Ok(mut station_lock)) => {
                        for robot in robots_lock.iter_mut() {
                            robot.update(&mut map_lock, &mut station_lock);
                            
                            // Si le robot est à court d'énergie, le ramener à la station
                            if robot.energy <= 0.0 {
                                robot.x = robot.home_station_x;
                                robot.y = robot.home_station_y;
                                robot.energy = robot.max_energy / 2.0;
                                robot.mode = RobotMode::Idle;
                            }
                        }
                        
                        // Vérifier si la station peut créer un nouveau robot (tous les 50 cycles)
                        // Mais seulement si la mission n'est pas complète
                        if iteration - last_robot_creation >= 50 && !mission_complete {
                            if let Some(new_robot) = station_lock.try_create_robot(&map_lock) {
                                robots_lock.push(new_robot);
                                last_robot_creation = iteration;
                                println!("Nouveau robot créé à l'itération {}", iteration);
                            }
                        }
                    },
                    _ => {
                        eprintln!("Erreur lors du verrouillage des ressources dans le thread de simulation");
                        break;
                    }
                }
            }
            
            // Créer l'état de la simulation
            let state_result = {
                match (map_for_sim.lock(), station_for_sim.lock(), robots_for_sim.lock()) {
                    (Ok(map_lock), Ok(station_lock), Ok(robots_lock)) => {
                        Ok(create_simulation_state(&map_lock, &station_lock, &robots_lock, iteration))
                    },
                    _ => {
                        eprintln!("Erreur lors de la création de l'état de simulation");
                        Err(())
                    }
                }
            };
            
            // Envoyer l'état aux clients connectés
            if let Ok(state) = state_result {
                if let Err(e) = state_tx.blocking_send(state) {
                    eprintln!("Erreur lors de l'envoi de l'état: {:?}", e);
                    break;
                }
            }
            
            // Attendre
            thread::sleep(Duration::from_millis(300));
            iteration += 1;
        }
        
        println!("Thread de simulation terminé.");
    });
    
    println!("Thread de simulation lancé avec succès.");
    
    // Serveur TCP pour les connexions des clients
    println!("Étape 8: Configuration du serveur TCP...");
    println!("Tentative d'ouverture du port TCP {}...", DEFAULT_PORT);
    
    let listener = match TcpListener::bind(format!("127.0.0.1:{}", DEFAULT_PORT)).await {
        Ok(l) => {
            println!("Port TCP ouvert avec succès.");
            l
        },
        Err(e) => {
            eprintln!("ERREUR lors de l'ouverture du port TCP {}: {:?}", DEFAULT_PORT, e);
            eprintln!("Vérifiez si le port n'est pas déjà utilisé par un autre programme.");
            return Err(e.into());
        }
    };
    
    println!("Serveur en écoute sur 127.0.0.1:{}", DEFAULT_PORT);
    println!("Démarrez l'interface Terre avec: cargo run --bin earth");
    
    // Utiliser TokioMutex au lieu de std::sync::Mutex pour les opérations asynchrones
    println!("Étape 9: Configuration du stockage des connexions clients...");
    let client_streams = Arc::new(TokioMutex::new(Vec::<TcpStream>::new()));
    let client_streams_clone = client_streams.clone();
    println!("Stockage des connexions configuré avec succès.");
    
    // Thread pour gérer le canal de distribution d'état
    println!("Étape 10: Configuration de la tâche de distribution d'état...");
    tokio::spawn(async move {
        println!("Tâche de distribution d'état démarrée.");
        
        while let Some(state) = state_rx.recv().await {
            let state_json = match serde_json::to_string(&state) {
                Ok(json) => json,
                Err(e) => {
                    eprintln!("Erreur lors de la sérialisation de l'état: {:?}", e);
                    continue;
                }
            };
            
            // Envoyer l'état à tous les clients connectés
            let mut disconnected_indices = Vec::new();
            
            // Obtenir un verrou sur le vecteur de clients - avec TokioMutex c'est async-safe
            let mut streams = client_streams_clone.lock().await;
            
            for (i, stream) in streams.iter_mut().enumerate() {
                if let Err(e) = stream.write_all(state_json.as_bytes()).await {
                    println!("Erreur d'écriture sur un client: {:?}", e);
                    disconnected_indices.push(i);
                } else {
                    if let Err(e) = stream.write_all(b"\n").await {
                        println!("Erreur d'écriture du délimiteur sur un client: {:?}", e);
                        disconnected_indices.push(i);
                    }
                }
            }
            
            // Supprimer les clients déconnectés
            for i in disconnected_indices.iter().rev() {
                println!("Suppression du client déconnecté à l'index {}", i);
                streams.remove(*i);
            }
        }
        
        println!("Tâche de distribution d'état terminée.");
    });
    
    println!("Tâche de distribution d'état configurée avec succès.");
    println!("Étape 11: Démarrage de la boucle d'acceptation des connexions...");
    
    // Accepter les connexions entrantes
    println!("Serveur prêt à accepter des connexions. En attente...");
    
    // Boucle d'acceptation des connexions qui s'exécute jusqu'à ce que la mission soit complète
    tokio::select! {
        _ = async {
            loop {
                match listener.accept().await {
                    Ok((stream, addr)) => {
                        println!("Nouvelle connexion: {}", addr);
                        // Ajouter le nouveau client à la liste - avec TokioMutex c'est async-safe
                        let mut streams = client_streams.lock().await;
                        streams.push(stream);
                        println!("Client ajouté à la liste. Nombre total de clients: {}", streams.len());
                    }
                    Err(e) => {
                        eprintln!("Erreur lors de l'acceptation d'une connexion: {:?}", e);
                    }
                }
            }
        } => {},
        
        _ = async {
            // Attendre le signal de fin de mission
            if mission_complete_rx.recv().await.is_some() {
                println!("\n=== FIN DE LA MISSION ===\n");
                // Attendre un peu pour que les derniers messages soient envoyés
                tokio::time::sleep(Duration::from_secs(5)).await;
                println!("Fermeture du serveur...");
            }
        } => {
            // Mission complète, terminer le programme
            return Ok(());
        }
    }
    
    Ok(())
}