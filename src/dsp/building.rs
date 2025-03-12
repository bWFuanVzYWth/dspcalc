use dspdb::recipe::RecipeItem;

#[derive(Clone, Debug)]
pub enum BuildingType {
    熔炉 = 1,
    化工 = 2,
    精炼厂 = 3,
    制造台 = 4,
    对撞机 = 5,
    科研站 = 15,
    矿机,
    喷涂机,
    Unknown,
}

pub const fn get_recipe_building(recipe_item: &RecipeItem) -> BuildingType {
    match recipe_item.type_ {
        1 => BuildingType::熔炉,
        2 => BuildingType::化工,
        3 => BuildingType::精炼厂,
        4 => BuildingType::制造台,
        5 => BuildingType::对撞机,
        15 => BuildingType::科研站,
        _ => BuildingType::Unknown,
    }
}

pub const fn time_scale(building_type: &BuildingType) -> f64 {
    1.0 / match building_type {
        BuildingType::熔炉 => 3.0,
        BuildingType::化工 => 2.0,
        BuildingType::精炼厂 => 1.0,
        BuildingType::制造台 => 3.0,
        BuildingType::对撞机 => 1.0,
        BuildingType::科研站 => 3.0,
        BuildingType::矿机 => 1.0,
        BuildingType::喷涂机 => 1.0,
        BuildingType::Unknown => 1.0,
    }
}