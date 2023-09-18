use rocket::{get, routes, launch};
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Serialize)]
struct Macros {
    calorie_needs: f64, 
    protein_needs: f64, 
    fat_needs: f64, 
    carb_needs: f64
}

// fn main() {
//     let age = 24; // Remplacez par l'âge de la personne
//     let weight = 68.0; // Remplacez par le poids en kilogrammes
//     let height = 159.0; // Remplacez par la taille en centimètres
//     let body_fat_percentage = 18.0; // Remplacez par le pourcentage de masse grasse
//     let is_male = true; // Remplacez par true si c'est un homme, false si c'est une femme

//     let (calories, protein, fat, carbs) = calculate_calories(age, weight, height, body_fat_percentage, is_male);

//     println!("Besoins quotidiens en calories : {:.2} kcal", calories);
//     println!("Besoins quotidiens en protéines : {:.2} g", protein);
//     println!("Besoins quotidiens en lipides : {:.2} g", fat);
//     println!("Besoins quotidiens en glucides : {:.2} g", carbs);
// }

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Attaching CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .mount("/", routes![index, calculate_calories])
}

#[get("/cal/<age>/<weight>/<height>/<body_fat_percentage>/<is_male>")]
fn calculate_calories(age: u32, weight: f64, height: f64, body_fat_percentage: f64, is_male: bool) -> Json<Macros> {
    // Constantes pour les valeurs de conversion
    const PROTEIN_PER_KG: f64 = 1.9;
    const FAT_PER_KG: f64 = 0.54;
    const CARBS_PER_KG: f64 = 4.0;

    // Calcul de la masse maigre en utilisant le pourcentage de masse grasse
    let lean_mass = weight * (1.0 - (body_fat_percentage / 100.0));

    // Calcul des besoins caloriques de base (BMR) en fonction de l'âge, du poids, de la taille et du sexe
    let bmr: f64 = if is_male {
        10.0 * weight + 6.25 * height - 5.0 * f64::from(age) + 5.0
    } else {
        10.0 * weight + 6.25 * height - 5.0 * f64::from(age) - 161.0
    };

    // Facteur d'activité (1.2 pour une activité sédentaire, à ajuster selon le niveau d'activité)
    let activity_factor = 1.09;

    // Calcul des besoins en calories en fonction de l'activité
    let calorie_needs = bmr * activity_factor;

    // Calcul des besoins en protéines, lipides et glucides
    let protein_needs = lean_mass * PROTEIN_PER_KG;
    let fat_needs = (weight * FAT_PER_KG) + (lean_mass * FAT_PER_KG);
    
    // Calcul des besoins en glucides en fonction des calories restantes
    let remaining_calories = calorie_needs - ((protein_needs * 4.0) + (fat_needs * 9.0));
    let carb_needs = remaining_calories / CARBS_PER_KG;


    Json(Macros {
        calorie_needs,
        protein_needs,
        fat_needs,
        carb_needs,
    })
}