mod config;
mod constraint;
mod objective;
mod translator;

use std::collections::{HashMap, HashSet};

use good_lp::{clarabel, variable, variables, SolverModel, Variable};

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

/// 绑定公式、线性规划求解器变量、权重
pub struct RecipeBinding {
    pub recipe: Recipe,
    pub variable: Variable,
    pub weight: f64,
}

struct ProcessedRecipes<'a> {
    consumes: HashMap<ResourceType, Vec<(&'a RecipeBinding, f64)>>,
    produces: HashMap<ResourceType, Vec<(&'a RecipeBinding, f64)>>,
}

impl<'a> ProcessedRecipes<'a> {
    fn new(recipes: &'a [RecipeBinding]) -> Self {
        let mut consumes: HashMap<ResourceType, Vec<_>> = HashMap::new();
        let mut produces: HashMap<ResourceType, Vec<_>> = HashMap::new();
        for recipe in recipes {
            // 处理消耗项
            for item in &recipe.recipe.items {
                let rate = item.num / recipe.recipe.time;
                consumes
                    .entry(item.resource_type)
                    .or_default()
                    .push((recipe, rate));
            }
            // 处理生产项
            for result in &recipe.recipe.results {
                let rate = result.num / recipe.recipe.time;
                produces
                    .entry(result.resource_type)
                    .or_default()
                    .push((recipe, rate));
            }
        }
        Self { consumes, produces }
    }
}

impl Problem {
    /// 求解的主逻辑
    ///
    /// # Errors
    /// 有两种情况会返回错误：
    /// - 输入的需求列表与权重列表长度不一致
    /// - 求解失败
    pub fn solve(&self) -> Result<Vec<Solution>, DspCalError> {
        // 验证权重数量
        if self.recipes.len() != self.weights.len() {
            return Err(DspCalError::MismatchedRecipeWeights(
                self.recipes.len(),
                self.weights.len(),
            ));
        }

        // 找出所有公式中出现过的产物
        let productions = Self::find_all_production(&self.recipes);

        // 绑定公式、公式权重和线性规划变量，变量即建筑数量
        let mut model = variables!();
        let recipe_extra = self.bind(&mut model);

        // 定义优化目标，暂时只支持权重表的形式
        let objective = minimize_by_weight(&recipe_extra);

        // 这个方法就叫minimise，不是minimize，奇异搞笑
        let mut clarabel_problem = model.minimise(objective).using(clarabel);

        // 设置线性规划求解精度
        config_solver(&mut clarabel_problem);

        // 预处理，构建从产品种类到相关公式的索引，加速约束构建
        let processed = ProcessedRecipes::new(&recipe_extra);

        // 根据公式生成并设置相应的约束
        let _ref_constraint = constraint_recipes(&processed, &mut clarabel_problem, &productions);

        // 根据需求列表生成并设置相应的约束
        let _ref_constraint_need = constraint_needs(&processed, &mut clarabel_problem, &self.needs);

        // 调用clarabel进行求解
        let clarabel_solution = clarabel_problem.solve().map_err(LpSolverError)?;

        // 把求解器的内部格式转换成求解器无关的格式
        let solution = from_clarabel_solution(&recipe_extra, &clarabel_solution);

        Ok(solution)
    }

    // 绑定公式、公式权重和线性规划变量，变量即建筑数量
    fn bind(&self, model: &mut good_lp::ProblemVariables) -> Vec<RecipeBinding> {
        self.recipes
            .iter()
            .zip(self.weights.iter())
            .map(|(recipe, &weight)| RecipeBinding {
                recipe: recipe.clone(),
                variable: model.add(variable().min(0.0)),
                weight,
            })
            .collect::<Vec<_>>()
    }

    // 找出所有公式中出现过的产物
    fn find_all_production(recipes: &[Recipe]) -> Vec<ResourceType> {
        recipes
            .iter()
            .flat_map(|r| r.results.iter().map(|r| r.resource_type))
            .collect::<HashSet<_>>()
            .into_iter()
            .collect()
    }
}
