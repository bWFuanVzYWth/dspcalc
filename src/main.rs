use good_lp::{clarabel, variable, variables, Solution, SolverModel};

fn main() {
    // 定义变量
    let (x1, x2, x3);
    let mut model = variables!();
    x1 = model.add(variable().min(0.0)); // 直接烧煤生产石墨的数量
    x2 = model.add(variable().min(0.0)); // 公式1生产石墨的数量
    x3 = model.add(variable().min(0.0)); // 公式2生产油的数量

    // 设定目标函数：最小化煤的消耗
    let objective = 2.0 * x1 + x3;

    // 添加约束条件
    let mut problem = model.minimise(objective).using(clarabel);

    let constraint_1 = problem.add_constraint((x1 + x2).eq(100.0));
    // 石墨需求约束：x1 + x2 = G（假设G=100）
    // 油平衡约束：x3 = x2
    let constraint_2 = problem.add_constraint((x3 - x2).eq(0.0));

    // 求解
    let solution = problem.solve().unwrap();

    // 输出结果
    println!("x1 (直接烧煤): {}", solution.value(x1));
    println!("x2 (公式1生产): {}", solution.value(x2));
    println!("x3 (公式2生产): {}", solution.value(x3));
    println!(
        "总煤消耗: {}",
        2.0 * solution.value(x1) + solution.value(x3)
    );
}
