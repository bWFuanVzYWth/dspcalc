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
    // 生成扁平化的广义合成公式列表，并对每个公式分配一个决策变量

    todo!()

    // model.solve()
}
