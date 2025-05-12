enum WineGrapes {
    CabernetFranc,
    Tannat,
    Merlot,
}

fn taste_wine(grapes: WineGrapes) {
    match grapes {
        WineGrapes::CabernetFranc => println!("This is a Cabertnet Franc wine."),
        WineGrapes::Tannat => println!("This is a Tannat wine."),
        WineGrapes::Merlot => println!("This is a Merlot wine"),
        _ => println!("The rest of the world does not know about wine."),
    }
}

fn main() {
    taste_wine(WineGrapes::CabernetFranc);
    taste_wine(WineGrapes::Tannat);
}
