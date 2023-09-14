use actix_web::{get, post, web::Json, App, HttpServer};
use actix_cors::Cors;
use base64::{decode};
use std::fs;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .service(post_qr)
            .service(get_status)
    })
    .bind(("127.0.0.1", 7878))?
    .run()
    .await
}

#[post("/QR")]
async fn post_qr(ticket_base64: String) -> String {
    let ticket = decode_ticket(ticket_base64);
    println!("Ticket: {}", ticket);
    
    if !is_valid_ticket(&ticket) {
        return String::from("Your ticket is invalid! Contact customer service");
    }

    if have_lounge_access(&ticket) {
        if let Ok(flag) = fs::read_to_string("flag.txt") {
            return format!("Welcome to the SAS Lounge! Here is your flag: {}", flag);
        } else {
            return String::from("Error reading the flag file");
        }
    }
    String::from("You do not have access to the SAS Lounge")
}

#[get("/status")]
async fn get_status()  -> Json<String> {
    Json::from(Json(format!("Welcome to SAS Airlines")))
}

fn is_valid_ticket(ticket: &String) -> bool {
    let valid_flight: bool = is_flight_valid(&ticket);
    let valid_ticket_id: bool = is_valid_ticket_id(&ticket);
    valid_flight && valid_ticket_id
}

fn is_flight_valid(ticket: &String) -> bool {
    let flight_numbers = get_flight_numbers();
    let parts: Vec<&str> = ticket.split('|').collect();
    if parts.len() >= 4 {
        let flight_number = parts[3];
        return flight_numbers.contains(&String::from(flight_number))
    } 
    false
}

fn is_valid_ticket_id(ticket: &String) -> bool {
    let ticket_ids = get_ticket_ids();
    let parts: Vec<&str> = ticket.split('|').collect();
    if parts.len() >= 1 {
        let ticket_id = parts[0].parse::<u32>();
        return match ticket_id {
            Ok(id) => ticket_ids.contains(&id),
            Err(_) => false,
        }
    } 
    false
}

fn have_lounge_access(ticket: &String) -> bool {
    let parts: Vec<&str> = ticket.split('|').collect();
    if parts.len() >= 5 {
        let group = parts[4];
        let lounge_privilege = parts[16];
        let service_class = parts[5];
        group > "C" && lounge_privilege == "Y" && service_class == "Plus"
    } else {
        false
    }
}

fn get_flight_numbers() -> [String; 4] {
    [String::from("SK3766"), String::from("SK1201"), String::from("SK0932"), String::from("SK2138")]
}

fn get_ticket_ids() -> [u32; 4] {
    [987210, 435762, 921784, 692312]
}

fn decode_ticket(ticket: String) -> String {
    match decode(ticket) {
        Ok(decoded_bytes) => {
            let decoded_str = String::from_utf8_lossy(&decoded_bytes);
            println!("Decoded: {}", decoded_str);
            String::from(decoded_str)
        },
        Err(_e) => {
            eprintln!("Error decoding");
            String::from("Could not decode string")
        }
    }
}