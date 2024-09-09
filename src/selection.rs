use crate::{entity::Entity, entity_config::EntityConfig, scenario::scenario::Scenario};

fn survival_filter(entity: &Entity, scenario: &Scenario) -> bool {
    if let Some(food) = &scenario.food {
        let food_selection = if food.cull_for_starvation {
            entity.times_eaten > 0
        } else {
            true
        };

        entity.body.is_alive && food_selection
    } else {
        entity.body.is_alive
    }
}

pub fn select_survivors(scenario: &Scenario, entities: Vec<Entity>) -> Vec<Entity> {
    entities
        .into_iter()
        .filter(|e| survival_filter(e, scenario))
        .collect()
}

fn calculate_score(entity: &Entity) -> i32 {
    entity.times_eaten as i32 * 2 - entity.times_irradiated as i32
}

pub fn select_breeders(
    scenario: &Scenario,
    entity_config: &EntityConfig,
    mut survivors: Vec<Entity>,
) -> Vec<Entity> {
    survivors.sort_by(|a, b| calculate_score(b).cmp(&calculate_score(a)));

    let pool_size = if scenario.limit_population {
        entity_config.start_count as usize
    } else {
        (survivors.len() as f32 * entity_config.survivor_breed_rate).floor() as usize
    };

    survivors.into_iter().take(pool_size).collect()
}
