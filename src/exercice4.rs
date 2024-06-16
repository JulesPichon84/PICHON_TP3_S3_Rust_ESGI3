// Importation du module `fmt` pour la mise en forme des affichages.
use std::fmt;

// On définit un trait IForme qui spécifie les comportements que doivent implémenter les formes.
trait IForme {
    // Méthode pour afficher les informations sur une forme.
    fn afficher(&self);
    
    // Méthode pour calculer le périmètre d'une forme.
    fn perimetre(&self) -> f64;
    
    // Méthode pour afficher toutes les informations sur une forme.
    fn afficher_tout(&self) {
        println!("Affichage de la forme :");
        self.afficher();
        println!("Périmètre : {}", self.perimetre());
    }
}

// On définit une structure Rectangle.
struct Rectangle {
    largeur: f64,
    longueur: f64,
}

// On définit une structure "Cercle".
struct Cercle {
    rayon: f64,
}

// On définit une implémentation des méthodes spécifiques à la structure "Rectangle".
impl Rectangle {
    // Constructeur pour créer un nouveau rectangle.
    fn new(largeur: f64, longueur: f64) -> Rectangle {
        Rectangle { largeur, longueur }
    }
}

// On définit une implémentation des méthodes spécifiques à la structure "Cercle".
impl Cercle {
    // Constructeur pour créer un nouveau cercle.
    fn new(rayon: f64) -> Cercle {
        Cercle { rayon }
    }
}

// On définit une implémentation du trait fmt::Display pour la structure "Rectangle".
impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Le rectangle fait {} de largeur et {} de longueur.", self.largeur, self.longueur)
    }
}

// On définit une implémentation du trait IForme pour la structure "Rectangle".
impl IForme for Rectangle {
    // Méthode pour afficher les informations d'un rectangle en utilisant fmt::Display.
    fn afficher(&self) {
        println!("{}", self);
    }

    // Méthode pour calculer le périmètre d'un rectangle.
    fn perimetre(&self) -> f64 {
        2.0 * (self.largeur + self.longueur)
    }
}

// On définit une implémentation du trait fmt::Display pour la structure "Cercle".
impl fmt::Display for Cercle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cercle de rayon {}", self.rayon)
    }
}

// la structure une implémentation du trait IForme pour pour la structure "Cercle".
impl IForme for Cercle {
    // Méthode pour afficher les informations d'un cercle en utilisant fmt::Display.
    fn afficher(&self) {
        println!("{}", self);
    }

    // Méthode pour calculer le périmètre d'un cercle.
    fn perimetre(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.rayon
    }
}

// Création de la fonction principale.
pub fn main() {
    // On créer une instance Rectangle.
    let rect = Rectangle::new(5.25, 15.75);
    
    // On créer une instance Cercle.
    let cercle = Cercle::new(3.14);

    // On affiche les informations complètes sur le rectangle.
    println!("\nAffichage du rectangle:");
    rect.afficher_tout();

    // On affiche les informations complètes sur le cercle
    println!("\nAffichage du cercle:");
    cercle.afficher_tout();
}
