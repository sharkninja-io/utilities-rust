use mantle_utilities::execute_and_join_jobs;

#[test]
fn test_execute_and_join_jobs() {
    let expected_results = ["first", "second", "third"];
    let jobs_iter = expected_results.iter().map(|&result| move || result);
    let result = execute_and_join_jobs(jobs_iter);

    assert_eq!(expected_results.len(), result.len());
    assert!(expected_results.into_iter().all(|r| result.contains(&r)));
}
