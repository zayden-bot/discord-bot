pub mod production_request;
pub mod render_request;
pub mod suggestions;
pub mod support_ticket;

use serenity::all::{ActionRow, ActionRowComponent, InputText};
use std::collections::HashMap;

fn parse_modal_data(components: &Vec<ActionRow>) -> HashMap<Box<str>, InputText> {
    let mut map: HashMap<Box<str>, InputText> = HashMap::with_capacity(components.len());

    for action_row in components {
        for component in action_row.components.clone() {
            match component {
                ActionRowComponent::InputText(input) => {
                    map.insert(input.custom_id.as_str().into(), input);
                }
                _ => unreachable!("Unsupported component type"),
            }
        }
    }

    map
}
