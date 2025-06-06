use dspcalc::{
    calc::Problem,
    dsp::{
        item::{Resource, ResourceType},
        recipe::Recipe,
    },
    error::DspCalError,
    unit_convert::{min_from_tick, sec_from_tick, tick_from_min, tick_from_sec},
};
use dspdb::item::item_name;

fn print_recipes(solutions: Vec<dspcalc::calc::Solution>) {
    let recipes_output = solutions
        .iter()
        .map(|solution| format_recipe(solution.num, &solution.recipe))
        .collect::<Vec<_>>()
        .join("\n");
    println!("增产决策,建筑数量,公式时长,输入输出\n{recipes_output}");
}

pub fn format_recipe(num_scale: f64, recipe: &Recipe) -> String {
    let decision = match &recipe.info.proliferator_type {
        Some(t) => {
            if t.level >= 1 {
                format!(
                    "{}_{}",
                    if t.is_speed_up { "加速" } else { "增产" },
                    t.level
                )
            } else {
                "无增产".to_string()
            }
        }
        None => "不适用".to_string(),
    };

    let recipe_time = sec_from_tick(recipe.time);

    let items_string = recipe
        .items
        .iter()
        .map(|resource| format_resources(num_scale, recipe, resource))
        .collect::<Vec<String>>()
        .join(" + ");

    let results_string = recipe
        .results
        .iter()
        .map(|resource| format_resources(num_scale, recipe, resource))
        .collect::<Vec<String>>()
        .join(" + ");

    format!("{decision},{num_scale:.6?},{recipe_time:.6?},{items_string} -> {results_string}")
}

fn format_resources(num_scale: f64, recipe: &Recipe, resource: &Resource) -> String {
    match resource.resource_type {
        ResourceType::Direct(cargo) => format!(
            "{:.6} * {}_{}",
            tick_from_min(num_scale * resource.num / recipe.time),
            item_name(cargo.item_id).unwrap_or(format!("ItemID{}", cargo.item_id)),
            cargo.level
        ),
        ResourceType::Indirect(indirect_resource) => match indirect_resource {
            dspcalc::dsp::item::IndirectResource::Energy => {
                format!(
                    "{:.6} MW",
                    tick_from_sec(num_scale * resource.num / recipe.time)
                )
            }
            dspcalc::dsp::item::IndirectResource::Area => todo!(),
        },
    }
}

struct Config {
    /// 是否摇匀
    cocktail: bool,
}

fn main() -> Result<(), DspCalError> {
    let need_white_cube = Resource::from_item_level(6006, 4, min_from_tick(1125000.0));
    // let need_proliferator_mk3 = from_item_level(1143, 4, min_from_tick(10000.0));

    let raw_recipes = dspdb::recipe::recipes_data();
    let raw_items = dspdb::item::items_data();

    // 求解方式，暂时只有是否摇匀
    let config = Config { cocktail: true };

    // 生成所有的公式
    let recipes = [
        Recipe::powers(),
        Recipe::flatten_recipes(&raw_recipes, &raw_items, config.cocktail)?,
        Recipe::proliferator_recipes(&raw_items, config.cocktail),
        Recipe::mines(&raw_items),
        Recipe::photons(),
    ]
    .concat();

    let weights: Vec<_> = recipes
        .iter()
        .map(|recipe| recipe.info.building_type.lag())
        .collect();

    // FIXME 检查并确保所有需求都在配方中
    // 声明所有需求
    let needs = vec![need_white_cube];
    // let needs = vec![need_proliferator_mk3];

    // 创建问题并求解
    let problem = Problem {
        recipes,
        needs,
        weights,
    };
    let solutions = problem.solve()?;

    // 输出
    let price = solutions.iter().map(|a| a.num).sum::<f64>();
    print_recipes(solutions);
    print!("总成本：{price}");

    Ok(())
}

// FIXME dspdb的一些公式的生产有问题
// FIXME 重氢，光子，电池：不是原矿，但是有公式生产
// TODO 接入禁用公式列表（直接移除对应的约束）
// TODO 增加真正的原矿化（直接移除相关的公式）

// TODO 群友建议
// 1、原矿化列表显示数量
// 2、生产策略/需求列表保存后，重新加载尚未包含原来“视为原矿的部分”
// 3、
//      “添加现有产线”尚未根据“批量预设”的增产和机器类型显示，还需要重新选择增产剂、增产模式、工厂类型。
//      “添加现有产线”默认工厂数量建议为1，现为10。
//      “添加现有产线”新增的配方和原配方位于同一位置，现在为置顶，
// 4、建筑统计可以不显示建筑名称，鼠标移上去再显示，原矿化列表现在就是移上去显示名称
// 5、支持导出分享策略（跨设备、跨用户共享）
// 6、计算结果导出excel，那么下面这些都简单了
// 7、支持查看多用途物品各配方的占比，例如白糖产线中电路板多少用于蓝糖、多少用于处理器。可以像现在的多来源 氢 那样展示。
// 8、显示单配方根据成品和原料分别需要的传送带（设置中设定传送带堆叠数量）数量
// 9、支持配方勾选、标注、改色等区别于其他配方的操作
// 10、支持方案对比，将两个保存的生产策略/需求列表根据原矿化列表、建筑统计、电力等信息进行对比
// 11、添加需求数量时根据机器数量选择，例如10个熔炉产出的钛合金
