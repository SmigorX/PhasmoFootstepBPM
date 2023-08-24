use clap::{App, Arg};

fn main() {
    let matches = App::new("Phasmophobia ghost bps cheatsheet")
        .version("v1.0")
        .author("SmigorX")
        .about("Phasmophobia BPM to ghost converter")
        .arg(Arg::with_name("bps")
            .short("b")
            .long("bps")
            .help("Search for ghost with closest bps")
            .takes_value(true))
        .get_matches();
    
    let data = vec![
        ("Deogen(close)", 0.4, 3),
        ("Revenant(noLOS),Thaye(10age)", 1.0, 60),
        ("Thaye(9age)", 1.175, 1),
        ("Thaye(8age)", 1.35, 1),
        ("Hantu(>15°C)", 1.4, 1),
        ("TheTwins(slower),Moroi(≥45%)", 1.5, 101),
        ("Thaye(7age)", 1.525, 1),
        ("Moroi(40-45%)", 1.583, 1),
        ("Moroi(35-40%)", 1.66, 1),
        ("Thaye(6age),Spirit,Wraith,Phantom,Poltergeist,Banshee,Jinn,Mare,Shade,Demon,Yurej,Oni,Yokai,Goryo,Myling,Onroyo,Obake", 1.7, 117),
        ("Moroi(30-35%),Hantu(normal)", 1.749, 1),
        ("Hantu(12-15°C)", 1.75, 1),
        ("Moroi(25-30%)", 1.832, 1),
        ("Thaye(5age)", 1.875, 1),
        ("TheTwins(faster)", 1.9, 130),
        ("Moroi(20-25%)", 1.915, 1),
        ("Moroi(15-20%)", 1.998, 1),
        ("Thaye(4age)", 2.05, 1),
        ("Moroi(10-15%)", 2.081, 1),
        ("Hantu(9-12°C)", 2.1, 1),
        ("Moroi(5-15%)", 2.164, 1),
        ("Thaye(3age)", 2.225, 1),
        ("Moroi(0-5%)", 2.25, 1),
        ("Hantu(6-9°C)", 2.3, 1),
        ("Hantu(3-6°C),Thaye(2age)", 2.4, 1),
        ("Hantu(0-3°C)", 2.5, 1),
        ("Raiju(elektronika)", 2.5, 1),
        ("Thaye(1age)", 2.575, 1),
        ("Hantu(< 0°C)", 2.7, 1),
        ("Thaye(0age)", 2.75, 1),
        ("Deogen(far)", 3.0, 1),
        ("Revenant(LOS)", 3.0, 1),
    ];
    
    if matches.is_present("bps") {
        if let Some(bps_value) = matches.value_of("bps") {
            println!("Searching for ghosts with BPS value: {}", bps_value);
            let int_bps: i32 = bps_value.parse().unwrap();
            let mut delta: i32 = 500;
            let mut ghost_name: (&str, f64, i32) = ("", 0.0, 0);
            for (name, ms, bps ) in &data {
                if (int_bps - bps).abs() < delta {
                    delta = (int_bps - bps).abs();
                    ghost_name = (name, *ms, *bps);
                    
                };
            };
            println!("Name: {}; ms: {} BPS: {}", ghost_name.0, ghost_name.1, ghost_name.2)   
        }
    } else {
        for (name, ms, bps) in &data {
            println!("Name: {}, ms: {}, BPS: {}", name, ms, bps);
        };
    }
}


