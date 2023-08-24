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
        ("Deogen(close)", 0.4, 60),
        ("Revenant(noLOS),Thaye(10age)", 1.0, 84),
        ("Thaye(9age)", 1.175, 91),
        ("Thaye(8age)", 1.35, 98),
        ("Hantu(>15°C)", 1.4, 101),
        ("TheTwins(slower),Moroi(≥45%)", 1.5, 104),
        ("Thaye(7age)", 1.525, 106),
        ("Moroi(40-45%)", 1.583, 107),
        ("Moroi(35-40%)", 1.66, 110),
        ("Thaye(6age),Spirit,Wraith,Phantom,Poltergeist,Banshee,Jinn,Mare,Shade,Demon,Yurej,Oni,Yokai,Goryo,Myling,Onroyo,Obake", 1.7, 115),
        ("Moroi(30-35%),Hantu(normal)", 1.749, 117),
        ("Hantu(12-15°C)", 1.75, 120),
        ("Moroi(25-30%)", 1.832, 125),
        ("Thaye(5age)", 1.875, 128),
        ("TheTwins(faster)", 1.9, 130),
        ("Moroi(20-25%)", 1.915, 137),
        ("Moroi(15-20%)", 1.998, 131),
        ("Thaye(4age)", 2.05, 144),
        ("Moroi(10-15%)", 2.081, 145),
        ("Hantu(9-12°C)", 2.1, 151),
        ("Moroi(5-15%)", 2.164, 156),
        ("Thaye(3age)", 2.225, 158),
        ("Moroi(0-5%)", 2.25, 163),
        ("Hantu(6-9°C)", 2.3, 174),
        ("Hantu(3-6°C),Thaye(2age)", 2.4, 175),
        ("Raiju(elektronika),Hantu(0-3°C)", 2.5, 180),
        ("Thaye(1age)", 2.575, 185),
        ("Hantu(< 0°C)", 2.7, 194),
        ("Thaye(0age)", 2.75, 212),
        ("Revenant(LOS),Deogen(far)", 3.0, 219),
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


