use clap::{App, Arg, SubCommand};
use reqwest::blocking::Response;
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ApiResponse {
    bodies: Vec<CelestialBody>, // Assuming the JSON key for the array is 'bodies'
}

#[derive(Deserialize, Debug)]
struct CelestialBody {
    name: String,
    id: String,
    is_planet: Option<bool>,     // Optional to handle missing data
    mass: Option<Mass>,          // Additional field for mass
    radius: Option<f64>,         // Additional field for radius in kilometers
    orbital_period: Option<f64>, // Additional field for orbital period in days
}

#[derive(Deserialize, Debug)]
struct Mass {
    mass_value: Option<f64>,    // Mass in kilograms
    mass_exponent: Option<i32>, // Exponent for scientific notation
}

// Function to fetch data from the Solar System API
fn fetch_celestial_bodies() -> Result<Vec<CelestialBody>, Error> {
    let url = "https://api.le-systeme-solaire.net/rest/bodies/";
    let response: Response = reqwest::blocking::get(url)?;
    let api_response: ApiResponse = response.json()?; // Deserialize into ApiResponse
    Ok(api_response.bodies) // Return just the array of bodies
}

fn fetch_celestial_body_details(name: &str) -> Result<CelestialBody, Error> {
    let url = format!("https://api.le-systeme-solaire.net/rest/bodies/{}", name);
    let response: Response = reqwest::blocking::get(&url)?;
    response.json::<CelestialBody>()
}

// Main function to execute the application
fn main() {
    let matches = App::new("Solar System Explorer")
        .version("0.1.0")
        .author("Your Name <your_email@example.com>")
        .about("Displays information about planets and other bodies in the solar system")
        .subcommand(
            SubCommand::with_name("details")
                .about("Displays detailed information about a specific celestial body")
                .arg(
                    Arg::with_name("name")
                        .help("The name of the celestial body to fetch details for")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("details") {
        if let Some(name) = matches.value_of("name") {
            match fetch_celestial_body_details(name) {
                Ok(body) => {
                    // Improved display of celestial body details
                    println!(
                        "Name: {}, ID: {}, Is Planet: {}",
                        body.name,
                        body.id,
                        body.is_planet.unwrap_or(false)
                    );
                    if let Some(mass) = &body.mass {
                        if let (Some(value), Some(exponent)) = (mass.mass_value, mass.mass_exponent)
                        {
                            println!("Mass: {}e{}", value, exponent);
                        } else {
                            println!("Mass data is incomplete.");
                        }
                    } else {
                        println!("No mass data available.");
                    }

                    if let Some(radius) = body.radius {
                        println!("Radius: {} kilometers", radius);
                    } else {
                        println!("No radius data available.");
                    }

                    if let Some(period) = body.orbital_period {
                        println!("Orbital Period: {} days", period);
                    } else {
                        println!("No orbital period data available.");
                    }
                }
                Err(e) => println!("Error fetching details for {}: {}", name, e),
            }
        }
    } else {
        match fetch_celestial_bodies() {
            Ok(bodies) => {
                for body in bodies {
                    println!(
                        "Name: {}, ID: {}, Is Planet: {}",
                        body.name,
                        body.id,
                        body.is_planet.unwrap_or(false)
                    );
                }
            }
            Err(e) => println!("Error fetching data: {}", e),
        }
    }
}
