// fonction de creation d'un nombre aléatoire
fn get_nb_mystere() -> i16 {
    use rand::Rng; // déclaration du module
    let mut rng = rand::thread_rng(); // initialisation du générateur aléatoire
    return rng.gen_range(1..101);
}

// fonction d'affichage d'un message attendant une saisie de l'utilisateur
fn input(message: &str) -> String {
    use std::io;
    use std::io::Write;
    print!("{}", message);
    io::stdout().flush().unwrap(); // Vidage du buffer d'affichage
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap(); //attente de la saisie
    return buffer.trim_end().to_string(); // retour du résultat
}

// fonction qui convertit un string en integer et renvoi -1 si le parse a échoué
fn str_to_int(str: &String) -> i16 {
    match str.parse::<i16>() {
        Ok(x) => x,
        Err(_) => -1,
    }
}

fn resultat_restant(nbr: i32) -> String {
    let resultat = match nbr {
        1 => "dernier essai !".to_string(),
        0 => "et c'est perdu".to_string(),
        _ => format!("{nbr} essais restants"),
    };
    return resultat;
}

fn main() {
    std::process::Command::new("clear").status().unwrap();
    let nb_mystere = get_nb_mystere();
    let nom = input("Quel est ton nom : ");
    println!("Bienvenue {nom}, tu vas devoir trouver le nombre auquel je pense entre 1 et 100");
    let mut tentatives = 6;

    loop {
        let mut saisie: i16 = 0;
        if tentatives > 0 {
            let saisie_ok = str_to_int(&input("Quel est ta proposition ? :"));
            if saisie_ok == -1 {
                println!("Seul les nombre en 1 et 100 sont considérés comme des choix valides");
                continue;
            }
            if saisie_ok < 1 || saisie_ok > 100 {
                println!("La valeur saisie doit être comprise entre 1 et 100");
                continue;
            }
            saisie = saisie_ok;
        }

        tentatives -= 1;
        if tentatives < 0 {
            println!(
                "Désolé {nom} tu n'as pas réussi à trouver le nombre {nb_mystere} en 6 tentatives"
            );
            break;
        }

        if saisie == nb_mystere {
            let essai = 6 - tentatives;
            let resultat = match essai {
                1 => "du premier coup !".to_string(),
                _ => "essais".to_string(),
            };
            if essai == 1 {
                println!("Bravo {nom} tu as trouvé {resultat}");
                break;
            } else {
                println!("Bravo {nom} tu as trouvé en {} {}", essai, resultat);
                break;
            }
        } else if saisie < nb_mystere {
            let phrase_resulat = resultat_restant(tentatives);
            println!(
                "Humm {nom} le nombre à trouver est plus grand que {saisie}, {phrase_resulat}"
            );
        } else {
            let phrase_resulat = resultat_restant(tentatives);
            println!("{nom} le nombre à trouver est plus petit que {saisie}, {phrase_resulat}");
        }
    }
}
