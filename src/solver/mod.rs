use crate::data::dsp::{item::Item, recipe::{BasicRecipe, BASIC_RECIPES}};
use good_lp::{
    clarabel, solvers::clarabel::ClarabelProblem, variable, variables, Solution, SolverModel,
};
use strum::IntoEnumIterator;

// TODO 根据基础公式生成扁平化的广义公式（带增产剂和耗电的）

// TODO 根据公式表生成约束
// 对每一种物品，生产必须大于需求
fn constraint_recipe(problem: &mut ClarabelProblem) {
    Item::iter().for_each(|item| {
        // 对每一个公式
        // BASIC_RECIPES.iter().map(|recipe| {})
    });
}


// TODO 传入需求和约束，返回求解过程和结果
pub fn solve() {
    // 定义变量，每个变量代表一个公式的调用次数
    // TODO 为每个公式命名，裁剪多余变量
    let mut model = variables!();
    let vars = model.add_vector(variable().min(0), BASIC_RECIPES.len());

    // TODO 多种待优化目标，如最小化加权原矿，最小化占地
    let objective = 2.0 * vars[0] + vars[2];

    let mut problem = model.minimise(objective).using(clarabel);

    // 自动生成 constraint_recipe
    let constraint_2 = problem.add_constraint((vars[2] - vars[1]).eq(0.0));

    // TODO 根据产能需求构建约束
    let constraint_1 = problem.add_constraint((vars[0] + vars[1]).eq(100.0));

    let solution = problem.solve().unwrap();

    println!("x1 (直接烧煤): {}", solution.value(vars[0]));
    println!("x2 (公式1生产): {}", solution.value(vars[1]));
    println!("x3 (公式2生产): {}", solution.value(vars[2]));
}
