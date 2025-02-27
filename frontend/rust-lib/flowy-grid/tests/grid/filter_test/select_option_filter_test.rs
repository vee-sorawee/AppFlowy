use crate::grid::filter_test::script::FilterScript::*;
use crate::grid::filter_test::script::GridFilterTest;
use flowy_grid::entities::SelectOptionCondition;

#[tokio::test]
async fn grid_filter_multi_select_is_empty_test() {
    let mut test = GridFilterTest::new().await;
    let scripts = vec![
        CreateMultiSelectFilter {
            condition: SelectOptionCondition::OptionIsEmpty,
            option_ids: vec![],
        },
        AssertNumberOfRows { expected: 2 },
    ];
    test.run_scripts(scripts).await;
}

#[tokio::test]
async fn grid_filter_multi_select_is_not_empty_test() {
    let mut test = GridFilterTest::new().await;
    let scripts = vec![
        CreateMultiSelectFilter {
            condition: SelectOptionCondition::OptionIsNotEmpty,
            option_ids: vec![],
        },
        AssertNumberOfRows { expected: 3 },
    ];
    test.run_scripts(scripts).await;
}

#[tokio::test]
async fn grid_filter_multi_select_is_test() {
    let mut test = GridFilterTest::new().await;
    let mut options = test.get_multi_select_type_option();
    let scripts = vec![
        CreateMultiSelectFilter {
            condition: SelectOptionCondition::OptionIs,
            option_ids: vec![options.remove(0).id, options.remove(0).id],
        },
        AssertNumberOfRows { expected: 2 },
    ];
    test.run_scripts(scripts).await;
}

#[tokio::test]
async fn grid_filter_multi_select_is_test2() {
    let mut test = GridFilterTest::new().await;
    let mut options = test.get_multi_select_type_option();
    let scripts = vec![
        CreateMultiSelectFilter {
            condition: SelectOptionCondition::OptionIs,
            option_ids: vec![options.remove(1).id],
        },
        AssertNumberOfRows { expected: 1 },
    ];
    test.run_scripts(scripts).await;
}

#[tokio::test]
async fn grid_filter_single_select_is_empty_test() {
    let mut test = GridFilterTest::new().await;
    let scripts = vec![
        CreateSingleSelectFilter {
            condition: SelectOptionCondition::OptionIsEmpty,
            option_ids: vec![],
        },
        AssertNumberOfRows { expected: 2 },
    ];
    test.run_scripts(scripts).await;
}

#[tokio::test]
async fn grid_filter_single_select_is_test() {
    let mut test = GridFilterTest::new().await;
    let mut options = test.get_single_select_type_option();
    let scripts = vec![
        CreateSingleSelectFilter {
            condition: SelectOptionCondition::OptionIs,
            option_ids: vec![options.remove(0).id],
        },
        AssertNumberOfRows { expected: 2 },
    ];
    test.run_scripts(scripts).await;
}
