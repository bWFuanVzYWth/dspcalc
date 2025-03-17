mod config;
mod constraint;
mod objective;
mod translator;

use std::collections::HashSet;

use good_lp::{clarabel, variable, variables, SolverModel};

use config::config_solver;
use constraint::{constraint_needs, constraint_recipes};
use objective::minimize_by_weight;
use translator::from_clarabel_solution;

use crate::{
    dsp::{
        item::{Resource, ResourceType},
        recipe::Recipe,
    },
    error::DspCalError::{self, LpSolverError},
};

pub struct Problem {
    pub recipes: Vec<Recipe>,
    pub needs: Vec<Resource>,
    pub weights: Vec<f64>,
}

pub struct Solution {
    pub recipe: Recipe,
    pub num: f64,
}

fn find_all_production(recipes: &[Recipe]) -> Vec<ResourceType> {
    let mut items_type = HashSet::new();
    for recipe in recipes {
        for product in &recipe.results {
            items_type.insert(product.resource_type);
        }
    }
    items_type.into_iter().collect()
}

impl Problem {
    pub fn solve(&self) -> Result<Vec<Solution>, DspCalError> {
        let all_productions = find_all_production(&self.recipes);

        // 声明变量，每个变量表示某个公式对应的建筑数量
        let mut model = variables!();
        let recipe_variables = self
            .recipes
            .iter()
            .map(|_| model.add(variable().min(0.0)))
            .collect::<Vec<_>>();

        // let objective = minimize_buildings_count(&recipe_variables);
        let objective = minimize_by_weight(&recipe_variables, &self.weights);

        // 这个方法就叫minimise，不是minimize，奇异搞笑
        let mut clarabel_problem = model.minimise(objective).using(clarabel);

        // 设置线性规划求解精度
        config_solver(&mut clarabel_problem);

        // FIXME 异常处理
        // 根据公式生成并设置相应的约束
        let _ref_constraint = constraint_recipes(
            &self.recipes,
            &recipe_variables,
            &mut clarabel_problem,
            &all_productions,
        );

        // 根据需求列表生成并设置相应的约束
        let _constraint_need = constraint_needs(
            &self.recipes,
            &recipe_variables,
            &mut clarabel_problem,
            &self.needs,
        );

        // 求解
        let clarabel_solution = clarabel_problem.solve().map_err(LpSolverError)?;

        // 把求解器的内部格式转换成通用的格式
        let solution =
            from_clarabel_solution(&recipe_variables, &self.recipes, &clarabel_solution)?;

        Ok(solution)
    }
}
