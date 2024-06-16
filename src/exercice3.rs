/* On créer une directive d'attribut qui sert à appliquer plusieurs dérivations automatiques à une structure ou une énumération.
- Debug --> Permet d'utiliser la macro {:?} afin de formater une instance pour rendre le débogage plus facile.
- PartielEq --> Permet de comparer 2 instances d'une structure ou d'une énumération avec "==" et "!=".
- Clone --> Permet de créer une copie d'une instance avec la méthode ".clone()".
- Copy --> Permet de copier une instance sans avoir à utiliser la méthode ".clone()".*/

#[derive(Debug, PartialEq, Clone, Copy)]
// Création de l'énumération "Couleur"
pub enum Couleur {
    PIQUE,
    COEUR,
    CARREAU,
    TREFLE,
}

#[derive(Debug, PartialEq, Clone)]
// Création de la structure "Carte"
pub struct Carte {
    a_couleur: Couleur,
    a_valeur: String,
}

// Création d'une implémentation "Carte" afin de définir plusieurs méthodes propres à la structure "Carte"
impl Carte {
    // Constructeur pour créer une nouvelle carte.
    pub fn new(a_couleur: Couleur, a_valeur: String) -> Self {
        Self { a_couleur, a_valeur }
    }

    // Méthode pour afficher une carte.
    pub fn afficher(&self) {
        println!("La carte est : {:?} de {:?}\n", self.a_valeur, self.a_couleur);
    }

    // Méthode pour affecter une carte à une couleur.
    pub fn affecter(&mut self, carte: Carte) {
        self.a_couleur = carte.a_couleur;
        self.a_valeur = carte.a_valeur;
    }

    // Méthode pour comparer 2 cartes.
    pub fn equal(&self, carte: Carte) -> bool {
        self.a_couleur == carte.a_couleur && self.a_valeur == carte.a_valeur
    }

    // Méthode pour définir la couleur d'une carte.
    pub fn set_type(&mut self, couleur: Couleur) {
        self.a_couleur = couleur;
    }

    // Méthode pour définir le type d'une carte.
    pub fn set_valeur(&mut self, valeur: String) {
        self.a_valeur = valeur;
    }
}

// Création de la fonction principale
pub fn main() {
    // On définit une première carte
    let mut carte1 = Carte::new(Couleur::PIQUE, String::from("As"));
    println!("Carte 1 = {:?}", carte1);

    // On affiche la première carte
    carte1.afficher(); 

    // On définit une seconde carte.
    let carte2 = Carte::new(Couleur::COEUR, String::from("Roi"));
    println!("Carte 2 = {:?}", carte2);

    // On affecte la seconde carte à la première.
    carte1.affecter(carte2.clone());
    println!("Carte 1 = {:?}", carte1);

    // On compare la première et la deuxième carte.
    println!("Carte 1 == Carte 2 ? {:?}\n", carte1.equal(carte2));

    // On définit une troisième carte.
    let mut carte3 = Carte::new(Couleur::TREFLE, String::from("Dame"));
    println!("Carte 3 = {:?}", carte3);

    // On redéfinit le type de la troisième carte.
    carte3.set_type(Couleur::CARREAU);
    println!("Carte 3 = {:?}", carte3);

    // On redéfinit la couleur de la troisième carte.
    carte3.set_valeur(String::from("10"));
    println!("Carte 3 = {:?}", carte3);
}