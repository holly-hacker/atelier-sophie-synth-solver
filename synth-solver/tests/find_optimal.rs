use synth_solver::{utils::test_data::*, *};

#[test]
pub fn test_basic_uni_bag() {
    let cauldron = cauldron::uni_bag_5x5_bonus1();
    let goals = goals::uni_bag();
    let materials = vec![
        vec![material::uni(), material::uni()],
        vec![material::beehive()],
        vec![material::broken_stone()],
    ];
    let properties = solver::SolverSettings::default();
    let optimal_routes = solver::find_optimal_routes(&cauldron, &materials, &goals, &properties);

    assert_eq!(optimal_routes.len(), 2);
    assert_eq!(
        optimal_routes
            .iter()
            .cloned()
            .filter(|r| r.0.scores.as_ref() == [1, 1, 1])
            .count(),
        1
    );
    assert_eq!(
        optimal_routes
            .iter()
            .cloned()
            .filter(|r| r.0.scores.as_ref() == [2, 0, 0])
            .count(),
        1
    );
}

#[test]
pub fn test_basic_uni_bag_with_rotations() {
    let cauldron = cauldron::uni_bag_5x5_bonus1();
    let goals = goals::uni_bag();
    let materials = vec![
        vec![material::uni(), material::uni()],
        vec![material::beehive()],
        vec![material::broken_stone()],
    ];
    let properties = solver::SolverSettings {
        transformations: TransformationType::Rotate,
        allow_overlaps: false,
    };
    let optimal_routes = solver::find_optimal_routes(&cauldron, &materials, &goals, &properties);

    assert_eq!(optimal_routes.len(), 3);
    assert_eq!(
        optimal_routes
            .iter()
            .cloned()
            .filter(|r| r.0.scores.as_ref() == [1, 1, 1])
            .count(),
        1
    );
    assert_eq!(
        optimal_routes
            .iter()
            .cloned()
            .filter(|r| r.0.scores.as_ref() == [1, 2, 0])
            .count(),
        1
    );
    assert_eq!(
        optimal_routes
            .iter()
            .cloned()
            .filter(|r| r.0.scores.as_ref() == [2, 0, 0])
            .count(),
        1
    );
}
