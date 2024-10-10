use std::io;

pub const MWST_ZUSCHLAG: f64 = 1.19; // 119%
pub const MWST_ABZUG: f64 = 0.81; // 100% - 19%
pub const SHOPIFY_GEBÜHR: f64 = 0.979; // 100% - 2.1%
pub const ZAHLUNGSANBIETER_GEBÜHR: f64 = 0.98; // 100% - 2%

pub fn akku_wechsel() -> () {
    let netto_gewinn_ziel: f64 = 35.0;

    loop {
        println!("Kaufpreis in €?");

        // Eingabe-Abfrage:
        let mut eingabe = String::new();
        if let Err(_) = io::stdin().read_line(&mut eingabe) {
            println!("Bruder, nur Zahlen eingeben!");
            continue;
        }

        // Datentyp-Konversion von Text zu Zahl:
        let brutto_kaufpreis: f64 = match eingabe.trim().parse() {
            Ok(zahl) => zahl,
            Err(_) => {
                println!("Bruder, nur Zahlen eingeben!");
                continue;
            }
        };

        // Berechnung des Netto-Kaufpreises
        let netto_kaufpreis: f64 = brutto_kaufpreis * MWST_ZUSCHLAG;

        // Berechnung des Verkaufspreises (mit Preissteigerung)
        let mut verkaufspreis: f64 = netto_kaufpreis.ceil();

        // Berechnung des Netto-Gewinns nach dem Verkaufspreis ohne MwSt.
        let mut netto_gewinn: f64 =
            verkaufspreis as f64 * MWST_ABZUG * SHOPIFY_GEBÜHR * ZAHLUNGSANBIETER_GEBÜHR
                - netto_kaufpreis; // Rundet immer um 1€ auf

        // Schleife, welche den Preis um 1€ erhöht, bis das Netto-Gewinnziel erreicht ist
        while netto_gewinn < netto_gewinn_ziel {
            verkaufspreis += 1.0;
            netto_gewinn =
                verkaufspreis as f64 * MWST_ABZUG * SHOPIFY_GEBÜHR * ZAHLUNGSANBIETER_GEBÜHR
                    - netto_kaufpreis;
        }

        // Ergebnisse anzeigen
        println!("Verkaufspreis: {:.2}€", verkaufspreis);
        println!("Netto-Gewinn: {:.2}€\n", netto_gewinn);
    }
}

pub fn display_wechsel() -> () {
    let netto_gewinn_ziel: f64 = 50.0;

    loop {
        println!("Kaufpreis in €?");

        // Eingabe-Abfrage:
        let mut eingabe = String::new();
        if let Err(_) = io::stdin().read_line(&mut eingabe) {
            println!("Bruder, nur Zahlen eingeben!");
            continue;
        }

        // Datentyp-Konversion von Text zu Zahl:
        let brutto_kaufpreis: f64 = match eingabe.trim().parse() {
            Ok(zahl) => zahl,
            Err(_) => {
                println!("Bruder, nur Zahlen eingeben!");
                continue;
            }
        };

        // Berechnung des Netto-Kaufpreises
        let netto_kaufpreis: f64 = brutto_kaufpreis * MWST_ZUSCHLAG;

        // Berechnung des Verkaufspreises (mit Preissteigerung)
        let mut verkaufspreis: f64 = netto_kaufpreis.ceil();

        // Berechnung des Netto-Gewinns nach dem Verkaufspreis ohne MwSt.
        let mut netto_gewinn: f64 =
            verkaufspreis as f64 * MWST_ABZUG * SHOPIFY_GEBÜHR * ZAHLUNGSANBIETER_GEBÜHR
                - netto_kaufpreis; // Rundet immer um 1€ auf

        // Schleife, welche den Preis um 1€ erhöht, bis das Netto-Gewinnziel erreicht ist
        while netto_gewinn < netto_gewinn_ziel {
            verkaufspreis += 1.0;
            netto_gewinn =
                verkaufspreis as f64 * MWST_ABZUG * SHOPIFY_GEBÜHR * ZAHLUNGSANBIETER_GEBÜHR
                    - netto_kaufpreis;
        }

        // Ergebnisse anzeigen
        println!("Verkaufspreis: {:.2}€", verkaufspreis);
        println!("Netto-Gewinn: {:.2}€\n", netto_gewinn);
    }
}
