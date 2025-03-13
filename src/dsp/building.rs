use dspdb::recipe::RecipeItem;

#[derive(Clone, Debug)]
pub enum BuildingType {
    熔炉 = 1,
    化工 = 2,
    精炼厂 = 3,
    制造台 = 4,
    对撞机 = 5,
    分馏塔 = 8,
    科研站 = 15,
    矿机,
    喷涂机,
    小太阳,
}

impl BuildingType {
    #[must_use]
    pub fn from_recipe_item(recipe_item: &RecipeItem) -> Self {
        match recipe_item.type_ {
            1 => Self::熔炉,
            2 => Self::化工,
            3 => Self::精炼厂,
            4 => Self::制造台,
            5 => Self::对撞机,
            8 => Self::分馏塔,
            15 => Self::科研站,
            _ => panic!("Fatal: unknown building_type: {}", recipe_item.type_),
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
            Self::分馏塔 => 1.0,
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
            Self::分馏塔 => 17800.0, // FIXME 这个公式本来就是错误的。只是为了规范而保留，任意时候都不应该使用
            Self::科研站 => 1920.0,
            Self::矿机 => 25400.0, // FIXME 检查小数点后的数字
            Self::喷涂机 => 90.0,
            Self::小太阳 => 288_000.0,
        }
    }

    // TODO 做出更严格的密铺模块后替换
    #[must_use]
    pub const fn area(&self) -> f64 {
        match self {
            Self::熔炉 => 2.31118 * 2.31118,
            Self::化工 => 3.49766 * 6.59729,
            Self::精炼厂 => 6.2555 * 2.79486, // 带爪
            Self::制造台 => 3.0416 * 3.0416,
            Self::对撞机 => 9.35815 * 4.6756,
            Self::分馏塔 => 17.0, // 6个一串联，算上补氢占地，平均单台17
            Self::科研站 => 4.4495 * 4.4495 / 15.0,
            Self::矿机 => 0.0,
            Self::喷涂机 => 0.0,
            Self::小太阳 => 45.576, // 赤道占地
        }
    }
}
