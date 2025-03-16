mod basic_recipe;
mod mine;
mod photons;
mod power;
mod proliferator;

use dspdb::item::ItemData;
use dspdb::recipe::RecipeItem;

use super::{building::BuildingType, item::Resource};

#[derive(Clone, Debug)]
pub struct RecipeFmtInfo {
    pub name: String, // 公式的名字
    pub proliferator_type: Option<ProliferatorType>,
    pub building_type: BuildingType, // 生产于什么建筑
}

#[derive(Clone, Debug)]
pub struct ProliferatorType {
    pub level: u8,
    pub is_speed_up: bool,
}

impl Default for RecipeFmtInfo {
    fn default() -> Self {
        Self {
            name: String::from("Unknown Building"),
            proliferator_type: None,
            building_type: BuildingType::矿机, // FIXME 不应该出现未知建筑
        }
    }
}

#[derive(Clone, Debug)]
pub struct Recipe {
    pub items: Vec<Resource>,   // 原料
    pub results: Vec<Resource>, // 产物
    pub time: f64,              // 公式耗时，单位帧
    pub info: RecipeFmtInfo,    // 不参与计算的信息
}

impl Recipe {
    #[must_use]
    pub fn flatten_recipes(
        basic_recipes: &[RecipeItem],
        items: &[ItemData],
        cocktail: bool,
    ) -> Vec<Self> {
        let mut recipes = Vec::new();
        for recipe_item in basic_recipes {
            Self::recipe_vanilla(&mut recipes, recipe_item);
            Self::recipes_productive(&mut recipes, recipe_item, items, cocktail);
            Self::recipes_accelerate(&mut recipes, recipe_item, cocktail);
        }
        recipes
    }
}
