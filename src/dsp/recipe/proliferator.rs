use dspdb::item::ItemData;

use crate::dsp::{
    building::BuildingType, item::Resource, proliferator::Proliferator, recipe::RecipeFmtInfo,
};

use super::Recipe;

impl Recipe {
    fn generate_proliferator_recipe(
        recipes: &mut Vec<Self>,
        item_data: &ItemData,
        proliferator: &Proliferator,
    ) {
        const STACK: f64 = 4.0;
        const PROLIFERATOR_TIME: f64 = 2.0;
        for cargo_level in 1..=Proliferator::inc_level(proliferator) {
            for proliferator_level in 0..=Proliferator::MAX_INC_LEVEL {
                recipes.push(Self {
                    items: {
                        let mut items = vec![
                            Resource::from_item_level(item_data.id, 0, STACK),
                            Resource::from_item_level(
                                Proliferator::item_id(proliferator),
                                proliferator_level,
                                f64::from(cargo_level)
                                    / f64::from(Proliferator::inc_level(proliferator))
                                    * STACK
                                    / f64::from(Proliferator::life(
                                        proliferator,
                                        proliferator_level,
                                    )),
                            ),
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
        }
    }

    #[must_use]
    pub fn proliferator_recipes(items_data: &[ItemData]) -> Vec<Self> {
        let mut recipes = Vec::new();
        for item_data in items_data {
            Self::generate_proliferator_recipe(&mut recipes, item_data, &Proliferator::MK3);
            Self::generate_proliferator_recipe(&mut recipes, item_data, &Proliferator::MK2);
            Self::generate_proliferator_recipe(&mut recipes, item_data, &Proliferator::MK1);
        }
        recipes
    }
}
