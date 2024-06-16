// Création de la structure Livre
pub struct Livre {
    titre: String,
    auteur: String,
    annee: u16,
}

// Création d'une implémentation qui utilise la structure "Livre".
impl Livre {
    // Constructeur pour créer un nouveau livre.
    pub fn new(titre: String, auteur: String, annee: u16) -> Self {
        Livre { titre, auteur, annee }
    }

    // Méthode qui affiche la structure Livre.
    pub fn afficher(&self) {
        println!("\nTitre du livre : {}", self.titre);
        println!("Auteur du livre : {}", self.auteur);
        println!("Année de publication du livre: {}", self.annee);
    }

    // Méthode qui affecte un livre à un autre livre.
    pub fn affecter(&mut self, autre: &Livre) {
        self.titre = autre.titre.clone();
        self.auteur = autre.auteur.clone();
        self.annee = autre.annee;
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

pub fn main() {
    // On définit un nouveau livre.
    let livre1 = Livre::new("Moby-Dick".to_string(), "Herman Melville".to_string(), 1851);
    // On affiche le livre.
    livre1.afficher();

    // On créer un deuxième livre et on lui affecte les attributs du premier avant de l'afficher.
    let mut livre2 = Livre::new("".to_string(), "".to_string(), 0);
    livre2.affecter(&livre1);
    livre2.afficher();

    println!("\nTitre du livre2 : {}", livre2.get_titre());
    println!("Auteur du livre2 : {}", livre2.get_auteur());
    println!("Année du livre2 : {}", livre2.get_annee());
}