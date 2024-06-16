// Appel des 4 exercices.
// Utilisation du module std::io pour traiter les input et les output.
mod exercice1;
mod exercice2;
mod exercice3;
mod exercice4;
use std::io;


// Fonction Menu
fn main() {
    // On fait une boucle loop pour que l'utilisateur puisse faire tous les exercices.
    loop {
        println!("\n----------TP3 RUST By Jules PICHON----------");
        println!("Menu:");
        println!("1. Exercice 3.1");
        println!("2. Exercice 3.2");
        println!("3. Exercice 3.3");
        println!("4. Exercice 3.4");
        println!("5. Quitter");
        println!("Veuillez choisir une option :");

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).expect("Erreur lors de la lecture de l'entrée");  // On gère les erreurs de saisie.
        let choix: u32 = match choix.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Veuillez saisir un nombre valide.");
                continue;
            }
        };

        // On associe le choix de l'utilisateur à un exercice.
        match choix {
            1 => exercice1::main(),
            2 => exercice2::main(),
            3 => exercice3::main(),
            4 => exercice4::main(),
            5 => {
                println!("Merci d'avoir participé au TP et au revoir !");
                break;
            }
            _ => println!("Option invalide. Veuillez choisir une option valide."),  // On gère les erreurs de saisie.
        } 
    }
}