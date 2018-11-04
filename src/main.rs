use iron::prelude::*;
use rand::{thread_rng, Rng};

struct RequestHandler {}
impl alexa::RequestHandler for RequestHandler {
    fn handle_request(&self, req: &alexa::Request) -> alexa::Response {
        match req.body {
            alexa::RequestBody::IntentRequest(ref ir) => match ir.name.as_str() {
                "D" => {
                    let num_o: Option<u64> = ir.slots.get("num").and_then(|n| n.parse().ok());
                    match num_o {
                        Some(num) => dice_roll_response(num),
                        None => no_dice_number(),
                    }
                }
                _ => i_dont_understand(),
            },
            _ => i_dont_understand(),
        }
    }
}
fn dice_roll_response<'a>(num: u64) -> alexa::Response<'a> {
    let mut rng = thread_rng();
    let number = rng.gen_range(1, num);
    alexa::Response {
        session_attributes: None,
        card: None,
        reprompt: None,
        output_speech: Some(alexa::OutputSpeech::Text(
            format!("You roll a {}", number).into(),
        )),
        should_end_session: true,
    }
}

fn no_dice_number<'a>() -> alexa::Response<'a> {
    alexa::Response {
        session_attributes: None,
        card: None,
        reprompt: None,
        output_speech: Some(alexa::OutputSpeech::Text(
            "Please say diceroll D and then number of sides, for example Roll D 6".into(),
        )),
        should_end_session: true,
    }
}

fn i_dont_understand<'a>() -> alexa::Response<'a> {
    alexa::Response {
        session_attributes: None,
        card: None,
        reprompt: None,
        output_speech: Some(alexa::OutputSpeech::Text(
            "Oh no, I don't understand what you said!, Say diceroll D 10 or something".into(),
        )),
        should_end_session: true,
    }
}

fn main() {
    let rh = RequestHandler {};
    let ih = alexa::IronHandler::new("Diceroll".to_owned(), Box::new(rh));
    let chain = Chain::new(ih);
    Iron::new(chain).http("0.0.0.0:3000").unwrap();
}
