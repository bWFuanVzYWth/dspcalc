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
    小太阳,
}

impl BuildingType {
    #[must_use]
    pub const fn from_recipe_item(recipe_item: &RecipeItem) -> Self {
        match recipe_item.type_ {
            1 => Self::熔炉,
            2 => Self::化工,
            3 => Self::精炼厂,
            4 => Self::制造台,
            5 => Self::对撞机,
            15 => Self::科研站,
            _ => panic!("Fatal: unknown building_type"),
        }
    }

    #[must_use]
    pub const fn time_scale(&self) -> f64 {
        match self {
            Self::熔炉 => 1.0 / 3.0,
            Self::化工 => 1.0 / 2.0,
            Self::精炼厂 => 1.0,
            Self::制造台 => 1.0 / 3.0,
            Self::对撞机 => 1.0,
            Self::科研站 => 1.0 / 3.0,
            Self::矿机 => 1.0,
            Self::喷涂机 => 1.0,
            Self::小太阳 => 1.0,
        }
    }

    #[must_use]
    pub const fn power(&self) -> f64 {
        match self {
            Self::熔炉 => 2880.0,
            Self::化工 => 2160.0,
            Self::精炼厂 => 960.0,
            Self::制造台 => 2700.0,
            Self::对撞机 => 12000.0,
            Self::科研站 => 1920.0,
            Self::矿机 => 25100.0,
            Self::喷涂机 => 90.0,
            Self::小太阳 => 288_000.0,
        }
    }

    pub const fn area(&self) -> f64 {
        match self {
            Self::熔炉 => 2880.0,
            Self::化工 => 2160.0,
            Self::精炼厂 => 960.0,
            Self::制造台 => 2700.0,
            Self::对撞机 => 12000.0,
            Self::科研站 => 1920.0,
            Self::矿机 => 25100.0,
            Self::喷涂机 => 90.0,
            Self::小太阳 => 288_000.0,
        }
    }
}
