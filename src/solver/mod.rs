use std::collections::HashMap;

use crate::data::dsp::item::Item;
use good_lp::{
    clarabel,
    solvers::clarabel::{self, ClarabelProblem},
    variable, Expression, ProblemVariables, SolverModel, Variable,
};
use strum::IntoEnumIterator;

#[derive(Clone, Eq, PartialEq, Hash)]
struct Cargo {
    item: Item,
    point: u64, // 增产点数
}

// 计算当前决策的成本
fn cost(vars: &HashMap<&Cargo, Variable>) -> Expression {
    // TODO 多种预设加权
    vars.values().sum()
}

// TODO 传入需求、约束
fn tmp() -> Result<clarabel::ClarabelSolution, good_lp::solvers::ResolutionError> {
    // 根据需要求解的问题，创建包含了所有相关物品的扁平化列表
    let cargos = Item::iter()
        .map(|item| {
            [0, 1, 2, 4] // 增产点数
                .iter()
                .map(|point| Cargo {
                    item: item.clone(),
                    point: *point,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .concat();

    // 在求解器中声明物品，并返回物品在求解器中的引用
    let mut problem = ProblemVariables::new();
    let vars = cargos
        .iter()
        .map(|cargo| (cargo, problem.add(variable().name(cargo.item.to_string()))))
        .collect::<HashMap<_, _>>();

    // 给问题建模
    let mut model = problem.minimise(cost(&vars)).using(clarabel);

    // TODO 根据合成公式创建所有约束

    model.solve()
}
