use factory_calculator::data::dsp::item::{Cargo, Resource, ResourceType};

fn main() {
    let need_white_cube = Resource {
        resource_type: ResourceType::Direct(Cargo {
            item_id: 6006,
            level: 4,
        }),
        num: 10000.0,
    };

    let need_proliferator_mk3 = Resource {
        resource_type: ResourceType::Direct(Cargo {
            item_id: 1143,
            level: 4,
        }),
        num: 10000.0,
    };

    let needs = vec![need_white_cube, need_proliferator_mk3];
    let mines = Vec::new();

    factory_calculator::solver::solve(&needs, &mines);
}
