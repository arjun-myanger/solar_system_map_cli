// Importing necessary libraries and modules from external crates.
use clap::{App, Arg, SubCommand}; // For creating and managing the command line interface.
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
    // Annotation to map 'englishName' JSON field to 'english_name' Rust field.
    english_name: String,
    #[serde(rename = "isPlanet")]
    is_planet: bool, // Annotation to map 'isPlanet' JSON field to 'is_planet' Rust field.
    mass: Option<Mass>, // Optional Mass struct to accommodate missing data.
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
    let url = "https://api.le-systeme-solaire.net/rest/bodies/"; // API endpoint.
    let response: Response = reqwest::blocking::get(url)?; // Making a blocking GET request.
    let api_response: ApiResponse = response.json()?; // Parsing JSON to ApiResponse struct.
    Ok(api_response.bodies) // Returning a vector of celestial bodies if successful.
}

// Function to fetch detailed information about a specific celestial body by name.
fn fetch_celestial_body_details(name: &str) -> Result<CelestialBody, Error> {
    // Constructing the URL with the given name
    let url = format!("https://api.le-systeme-solaire.net/rest/bodies/{}", name);

    // Making a blocking HTTP GET request to the URL
    let response: Response = reqwest::blocking::get(&url)?;

    // Attempting to deserialize the JSON response into a CelestialBody struct
    // The `?` operator is used to return the error if the request fails
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
                    Arg::with_name("name")  // Taking a 'name' argument to specify which body details to fetch.
                        .help("The name of the celestial body to fetch details for")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches(); // Parses the command-line arguments provided by the user.

    // Handling the 'details' subcommand to fetch and display information about a specific body.
    if let Some(matches) = matches.subcommand_matches("details") {
        if let Some(name) = matches.value_of("name") {
            match fetch_celestial_body_details(name) {
                Ok(body) => {
                    // If data is successfully fetched, it prints the details.
                    // Displaying basic information and checking for each optional field to print or handle missing data.
                    println!(
                        "Name: {}, ID: {}, English Name: {}, Is Planet: {}",
                        body.name, body.id, body.english_name, body.is_planet
                    );
                    if let Some(mass) = &body.mass {
                        if let (Some(value), Some(exponent)) = (mass.mass_value, mass.mass_exponent)
                        {
                            println!("Mass: {}e{}", value, exponent);
                        } else {
                            println!("Mass data is incomplete or not available.");
                        }
                    } else {
                        println!("No mass data provided by the API.");
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
                Err(e) => println!("Error fetching details for {}: {}", name, e), // Error handling if fetching fails.
            }
        }
    } else {
        // If no subcommand is specified, it fetches and displays all celestial bodies.
        match fetch_celestial_bodies() {
            Ok(bodies) => {
                for body in bodies {
                    println!(
                        "Name: {}, ID: {}, Is Planet: {}",
                        body.name, body.id, body.is_planet
                    );
                }
            }
            Err(e) => println!("Error fetching data: {}", e), // Error handling if fetching fails.
        }
    }
}
