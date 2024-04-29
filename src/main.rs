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
    is_planet: Option<bool>, // Changed to Option<bool> to handle missing data
}

// Function to fetch data from the Solar System API
fn fetch_celestial_bodies() -> Result<Vec<CelestialBody>, Error> {
    let url = "https://api.le-systeme-solaire.net/rest/bodies/";
    let response: Response = reqwest::blocking::get(url)?;
    let api_response: ApiResponse = response.json()?; // Deserialize into ApiResponse
    Ok(api_response.bodies) // Return just the array of bodies
}

// Main function to execute the application
fn main() {
    match fetch_celestial_bodies() {
        Ok(bodies) => {
            for body in bodies {
                println!(
                    "Name: {}, ID: {}, Is Planet: {}",
                    body.name,
                    body.id,
                    body.is_planet.unwrap_or(false) // Use `unwrap_or(false)` for display
                );
            }
        }
        Err(e) => println!("Error fetching data: {}", e),
    }
}
