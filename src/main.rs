use std::io;

mod lib;

fn main() {
    println!("Für was willst du den Preis ausrechnen?\n1 = Akku-Wechsel\n2 = Display-Wechsel");

    let mut auswahl = String::new();
    if let Err(_) = io::stdin().read_line(&mut auswahl) {
        println!("Fehler beim Lesen der Eingabe");
        return;
    }
    let auswahl = auswahl.trim();

    match auswahl {
        "1" => lib::akku_wechsel(),
        "2" => lib::display_wechsel(),
        _ => println!("Wat soll dat denn sein?"),
    }
}
