use good_lp::solvers::clarabel::ClarabelProblem;

pub fn config_solver(problem: &mut ClarabelProblem) {
    problem
        .settings()
        .verbose(true) // 启用详细输出
        .tol_gap_abs(f64::EPSILON)
        .tol_gap_rel(f64::EPSILON)
        .tol_feas(f64::EPSILON)
        .tol_infeas_abs(f64::EPSILON)
        .tol_infeas_rel(f64::EPSILON)
        .equilibrate_max_iter(256)
        .equilibrate_min_scaling(1.0)
        .equilibrate_max_scaling(1.0)
        .static_regularization_constant(f64::EPSILON)
        .dynamic_regularization_eps(f64::EPSILON)
        .dynamic_regularization_delta(f64::EPSILON)
        .iterative_refinement_reltol(f64::EPSILON)
        .iterative_refinement_abstol(f64::EPSILON)
        .max_iter(u32::MAX);
}
