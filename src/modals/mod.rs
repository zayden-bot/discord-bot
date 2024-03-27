pub mod production_request;
pub mod support_ticket;

use serenity::all::{ActionRow, ActionRowComponent, InputText};
use std::collections::HashMap;

fn parse_modal_data(components: &Vec<ActionRow>) -> HashMap<String, InputText> {
    let mut map: HashMap<String, InputText> = HashMap::new();

    for action_row in components {
        for component in action_row.components.clone() {
            match component {
                ActionRowComponent::InputText(input) => {
                    map.insert(input.custom_id.clone(), input);
                }
                _ => unreachable!("Unsupported component type"),
            }
        }
    }

    map
}
