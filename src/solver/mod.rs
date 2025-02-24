use std::collections::HashMap;

use crate::data::dsp::item::Item;
use good_lp::{variable, ProblemVariables, Variable};
use strum::IntoEnumIterator;

#[derive(Clone, Eq, PartialEq, Hash)]
struct Cargo {
    item: Item,
    point: u64, // 增产点数
}

fn tmp() {
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
    let hashmap = cargos
        .iter()
        .map(|cargo| (cargo, problem.add(variable().name(cargo.item.to_string()))))
        .collect::<HashMap<_, _>>();

    //
}
