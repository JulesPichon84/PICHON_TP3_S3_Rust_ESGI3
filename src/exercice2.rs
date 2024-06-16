// Utilisation de la structure de données HashMap afin de générer des paires de clé (uniques) - valeur pour les livres de la bibliothèque. Chaque livre possède un  
use std::collections::HashMap;

// Création d'un structure "Adhérent".
pub struct Adherent {
    nom: String,
    identifiant: u32,
}

// Création d'un structure "Livre".
pub struct Livre {
    titre: String,
    auteur: String,
    annee: u16,
}

// Création d'un structure "Bibliothèque".
pub struct Bibliotheque {
    nom: String,
    livres: HashMap<String, Livre>,
    adherents: Vec<Adherent>,
}

// Création d'une implémentation qui utilise la structure "Adherent".
impl Adherent {
    // Constructeur pour créer un nouvel adhérent.
    pub fn new(nom: String, identifiant: u32) -> Self {
        Adherent { nom, identifiant }
    }

    // Méthode qui renvoie le nom d'un adhérent.
    pub fn get_nom(&self) -> &str {
        &self.nom
    }

    // Méthode qui renvoie l'identifiant d'un adhérent.
    pub fn get_identifiant(&self) -> u32 {
        self.identifiant
    }
}

// Création d'une implémentation qui utilise la structure "Livre".
impl Livre {
    // Constructeur pour créer un nouveau livre.
    pub fn new(titre: String, auteur: String, annee: u16) -> Self {
        Livre { titre, auteur, annee }
    }

    // Méthode qui renvoie le titre d'un livre.
    pub fn get_titre(&self) -> &str {
        &self.titre
    }

    // Méthode qui renvoie l'auteur d'un livre.
    pub fn get_auteur(&self) -> &str {
        &self.auteur
    }

    // Méthode qui renvoie l'année de parution d'un livre.
    pub fn get_annee(&self) -> u16 {
        self.annee
    }
}

// Création d'une implémentation qui utilise la structure "Bibliotheque".
impl Bibliotheque {
    // Constructeur pour créer une nouvelle bibliothèque.
    pub fn new(nom: String) -> Self {
        Bibliotheque {
            nom,
            livres: HashMap::new(),
            adherents: Vec::new(),
        }
    }

    // Méthode qui ajoute un livre à la bibliothèque.
    pub fn ajouter_livre(&mut self, titre: String, livre: Livre) {
        self.livres.insert(titre, livre);
    }

    // Méthode qui ajoute un adhérent à la bibliothèque.
    pub fn ajouter_adherent(&mut self, adherent: Adherent) {
        self.adherents.push(adherent);
    }

    // Méthode qui affiche un livre de la bibliothèque.
    pub fn afficher_livres(&self) {
        for (titre, livre) in &self.livres {
            println!("\nTitre : {}", livre.get_titre());
            println!("Auteur : {}", livre.get_auteur());
            println!("Année : {}", livre.get_annee());
        }
    }

    // Méthode qui affiche un adhérent de la bibliothèque.
    pub fn afficher_adherents(&self) {
        for adherent in &self.adherents {
            println!("\nNom : {}", adherent.get_nom());
            println!("Identifiant : {}", adherent.get_identifiant());
        }
    }

    // Méthode qui renvoie le nom de la bibliothèque.
    pub fn get_nom(&self) -> &str {
        &self.nom
    }
}

// Création de la fonction principale.
pub fn main() {
    // On définit une nouvelle bibliothèque.
    let mut bibliotheque = Bibliotheque::new("bibliothèque de l'ESGI".to_string());

    // On définit 2 livres.
    let livre1 = Livre::new("Moby-Dick".to_string(), "Herman Melville".to_string(), 1851);
    let livre2 = Livre::new("Vingt Mille Lieues sous les mers".to_string(), "Jules Vernes".to_string(), 1869);

    // On ajoute les 2 livres crées à la bibliothèque.
    bibliotheque.ajouter_livre("Moby-Dick".to_string(), livre1);
    bibliotheque.ajouter_livre("Vingt Mille Lieues sous les mers".to_string(), livre2);

    // On définit 2 nouveaux adhérents à la bibliothèque.
    let adherent1 = Adherent::new("Jules Pichon".to_string(), 001);
    let adherent2 = Adherent::new("Monsieur Palermo".to_string(), 002);

    // On enregistre les 2 adhérents crées à la bibliothèque.
    bibliotheque.ajouter_adherent(adherent1);
    bibliotheque.ajouter_adherent(adherent2);

    // On affiche le contenu de la bibliothèque.
    println!("\nBienvenue à la {}", bibliotheque.get_nom());
    bibliotheque.afficher_livres();
    bibliotheque.afficher_adherents();
}