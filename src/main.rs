// Importing necessary libraries and modules from external crates.
use clap::{App, Arg, SubCommand}; // For creating and managing the command line interface.
use colored::*; // To add colored text in the console.
use reqwest::blocking::Response; // To handle HTTP responses in a blocking manner.
use reqwest::Error; // To handle errors from reqwest operations.
use serde::Deserialize; // To enable deserialization of JSON data into Rust structures.

// Struct to hold the API response for multiple celestial bodies.
#[derive(Deserialize, Debug)]
struct ApiResponse {
    bodies: Vec<CelestialBody>, // Vector of CelestialBody structs to store multiple bodies.
}

// Struct to describe a celestial body with potential fields from the API.
#[derive(Deserialize, Debug)]
struct CelestialBody {
    name: String,
    id: String,
    #[serde(rename = "englishName")]
    english_name: String,
    #[serde(rename = "isPlanet")]
    is_planet: bool,
    mass: Option<Mass>,
    density: Option<f64>,
    gravity: Option<f64>,
    escape: Option<f64>,
    mean_radius: Option<f64>,
    equa_radius: Option<f64>,
    polar_radius: Option<f64>,
    flattening: Option<f64>,
    sideral_orbit: Option<f64>,
    sideral_rotation: Option<f64>,
    axial_tilt: Option<f64>,
    avg_temp: Option<i32>,
    body_type: Option<String>,
}

// Struct to describe mass, accommodating optional fields for mass value and exponent.
#[derive(Deserialize, Debug)]
struct Mass {
    mass_value: Option<f64>,
    mass_exponent: Option<i32>,
}

// Function to fetch a list of all celestial bodies from the API.
fn fetch_celestial_bodies() -> Result<Vec<CelestialBody>, Error> {
    let url = "https://api.le-systeme-solaire.net/rest/bodies/";
    let response: Response = reqwest::blocking::get(url)?;
    let api_response: ApiResponse = response.json()?;
    Ok(api_response.bodies)
}

// Function to fetch detailed information about a specific celestial body by name.
fn fetch_celestial_body_details(name: &str) -> Result<CelestialBody, Error> {
    let url = format!("https://api.le-systeme-solaire.net/rest/bodies/{}", name);
    let response: Response = reqwest::blocking::get(&url)?;
    response.json::<CelestialBody>()
}

// The main function sets up the command-line interface and processes user input.
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
                    println!(
                        "{}: {}, {}: {}, {}: {}, {}: {}",
                        "Name".green().bold(),
                        body.name,
                        "ID".green().bold(),
                        body.id,
                        "English Name".green().bold(),
                        body.english_name,
                        "Is Planet".green().bold(),
                        body.is_planet.to_string().blue()
                    );
                    if let Some(mass) = &body.mass {
                        if let (Some(value), Some(exponent)) = (mass.mass_value, mass.mass_exponent)
                        {
                            println!("{}: {}e{}", "Mass".yellow().bold(), value, exponent);
                        } else {
                            println!("{}", "Mass data is incomplete or not available.".red());
                        }
                    } else {
                        println!("{}", "No mass data provided by the API.".red());
                    }
                    println!("Density: {} g/cm³", body.density.unwrap_or(0.0));
                    println!("Gravity: {} m/s²", body.gravity.unwrap_or(0.0));
                    println!("Escape Velocity: {} m/s", body.escape.unwrap_or(0.0));
                    println!("Mean Radius: {} km", body.mean_radius.unwrap_or(0.0));
                    println!("Equatorial Radius: {} km", body.equa_radius.unwrap_or(0.0));
                    println!("Polar Radius: {} km", body.polar_radius.unwrap_or(0.0));
                    println!("Flattening: {}", body.flattening.unwrap_or(0.0));
                    println!("Orbital Period: {} days", body.sideral_orbit.unwrap_or(0.0));
                    println!(
                        "Rotation Period: {} hours",
                        body.sideral_rotation.unwrap_or(0.0)
                    );
                    println!("Axial Tilt: {} degrees", body.axial_tilt.unwrap_or(0.0));
                    println!("Average Temperature: {} K", body.avg_temp.unwrap_or(0));
                    println!(
                        "Body Type: {}",
                        body.body_type.as_deref().unwrap_or("Not specified")
                    );
                }
                Err(e) => println!("{} {}: {}", "Error fetching details for".red(), name, e),
            }
        }
    } else {
        match fetch_celestial_bodies() {
            Ok(bodies) => {
                for body in bodies {
                    println!(
                        "{}: {}, {}: {}, {}: {}",
                        "Name".green().bold(),
                        body.name,
                        "ID".green().bold(),
                        body.id,
                        "Is Planet".green().bold(),
                        body.is_planet.to_string().blue()
                    );
                }
            }
            Err(e) => println!("{}: {}", "Error fetching data".red(), e),
        }
    }
}
