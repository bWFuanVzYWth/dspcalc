use strum::IntoEnumIterator;

use dspdb::item::ItemData;

use crate::dsp::{
    building::BuildingType, item::Resource, proliferator::Proliferator, recipe::RecipeFmtInfo,
};

use super::Recipe;

impl Recipe {
    fn generate_proliferator_recipes(
        recipes: &mut Vec<Self>,
        item_data: &ItemData,
        proliferator: &Proliferator,
        cocktail: bool,
    ) {
        if cocktail {
            for cargo_level in 1..=proliferator.inc_level() {
                for proliferator_level in 0..=Proliferator::MAX_INC_LEVEL {
                    Self::generate_proliferator_recipe(
                        recipes,
                        item_data,
                        proliferator,
                        cargo_level,
                        proliferator_level,
                    );
                }
            }
        } else {
            let cargo_level = proliferator.inc_level();
            Self::generate_proliferator_recipe(recipes, item_data, proliferator, cargo_level, 0);
            Proliferator::iter().for_each(|proliferator_proliferator| {
                Self::generate_proliferator_recipe(
                    recipes,
                    item_data,
                    proliferator,
                    cargo_level,
                    proliferator_proliferator.inc_level(),
                );
            });
        }
    }

    fn generate_proliferator_recipe(
        recipes: &mut Vec<Self>,
        item_data: &ItemData,
        proliferator: &Proliferator,
        cargo_level: u8,
        proliferator_level: u8,
    ) {
        const STACK: f64 = 4.0;
        const PROLIFERATOR_TIME: f64 = 2.0;

        let proliferator_life = f64::from(proliferator.life(proliferator_level));
        let base_amount = (f64::from(cargo_level) / f64::from(proliferator.inc_level())) * STACK;
        let amount = base_amount / proliferator_life;

        recipes.push(Self {
            items: {
                let mut items = vec![
                    Resource::from_item_level(item_data.id, 0, STACK),
                    Resource::from_item_level(proliferator.item_id(), proliferator_level, amount),
                ];
                items.push(Resource::power(BuildingType::喷涂机.power()));
                items
            },
            results: vec![Resource::from_item_level(item_data.id, cargo_level, STACK)],
            time: PROLIFERATOR_TIME,
            info: RecipeFmtInfo {
                name: String::from("喷涂"),
                building_type: BuildingType::喷涂机,
                ..RecipeFmtInfo::default()
            },
        });
    }

    #[must_use]
    pub fn proliferator_recipes(items_data: &[ItemData], cocktail: bool) -> Vec<Self> {
        let mut recipes = Vec::new();
        for item_data in items_data {
            Proliferator::iter().for_each(|proliferator| {
                Self::generate_proliferator_recipes(
                    &mut recipes,
                    item_data,
                    &proliferator,
                    cocktail,
                );
            });
        }
        recipes
    }
}
