use std::f32::consts::PI;
use std::f64::consts::PI;
use std::io;

fn main() {
    println!("\n Polygon Chord Calculator");
    println!("Type \"commands\" for list of commands that are available");

    let mut query = String::new();

    loop {
        query.clear();

        println!("\nAwaiting input...");

        io::stdin().read_line(&mut query).expect("Failed to read");

        let query = query[..].trim();

        //println!("You queried: {:#?}", query.as_bytes());

        if query.contains(",") {
            let chord = match destringify(query) {
                Ok(r) => r,
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            };
            println!("{:?}", chord);
            match calculate_chord(chord) {
                Ok(r) => println!("{}", r),
                Err(e) => println!("{}", e),
            }
            continue;
        }

        match query {
            "commands" => {
                println!(
                    "
                Chords:
                n, D --------- Calculates the length of the D-th chord of a unit convex n-gon.
                                  The 1st chord is a side of the convex n-gon,
                                  the 2nd chord spans two sides of the convex n-gon, etc.
                                  d must be less than n and greater than 0.
                
                n/d, D ---- Calculates the length of a D-th chord of a unit n/d-gon.
                                  The 1st chord is the shortest chord of the n/d-gon,
                                  the 2nd chord is the second shortest chord of the n/d-gon, etc,
                                  Both d and D must be less than n and greater than 0.

                Options:
                quit ---------- Exits the tool.
                commands ------ Opens this menu.
                marco --------- Returns \"Polo!\".
                "
                );
            }
            "marco" => println!("Polo!"),
            "quit" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Unknown query"),
        }
    }
}

/// The main carrier of information needed to find the side length
#[derive(Debug)]
struct Frac {
    poly: u32,
    gram: u32,
    chord: u32,
}
impl Frac {
    /// Calculates the `chord`-th chord of a `poly`/`gram`-gon.
    fn calculate_chord(frac: Frac) -> Result<f64, &'static str> {
        if frac.chord >= frac.poly || frac.gram >= frac.poly || frac.chord <= 0 || frac.gram <= 0 {
            return Err("Invalid denominator");
        }

        Ok((PI * frac.chord / frac.poly).sin() * (PI * frac.gram / frac.poly).csc())
    }
}

/// Parses a string slice into a Frac
fn destringify(frac: &str) -> Result<Frac, &'static str> {
    // Flag for if the polygon is starry or not.
    let mut star = false;
    if frac.contains("/") {
        star = true
    }

    let clean = frac.replace(", ", "/");
    let chunks: Vec<&str> = clean.split('/').collect();
    //println!("{:?}", chunks);

    // If the input is not starry, only two chunks are needed.
    if chunks.len() == 2 && star == false {
        //println!("not star");
        let poly = match chunks[0].parse::<u32>() {
            Ok(r) => r,
            Err(_e) => return Err("Failed to parse n"),
        };
        let chord = match chunks[1].parse::<u32>() {
            Ok(r) => r,
            Err(_e) => return Err("Failed to parse D"),
        };
        return Ok(Frac {
            poly: poly,
            gram: 1,
            chord: chord,
        });
    }

    // If the input is starry, there will be three chunks needed.
    if chunks.len() == 3 && star == true {
        //println!("star");
        let poly = match chunks[0].parse::<u32>() {
            Ok(r) => r,
            Err(_e) => return Err("Failed to parse n"),
        };
        let chord = match chunks[2].parse::<u32>() {
            Ok(r) => r,
            Err(_e) => return Err("Failed to parse D"),
        };
        let gram = match chunks[1].parse::<u32>() {
            Ok(r) => r,
            Err(_e) => return Err("Failed to parse d"),
        };
        return Ok(Frac {
            poly: poly,
            gram: gram,
            chord: chord,
        });
    }
    // If the Frac created does not fit either two, it is invalid.
    Err("Invalid input")
}
