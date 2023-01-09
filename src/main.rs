mod reader;
mod stop_init;
mod depth_search;

fn main() {
    let data = reader::parse_csv("MBTA_Bus_AM_PEAK0.csv");
    let stops = stop_init::create_stops(data);
    let adjacency_graph: Vec<Vec<f32>> = stop_init::create_adjacency_matrix(stops);
    let depth = depth_search::depth_first_search(&adjacency_graph, 88, 25);
    println!("{:?}", depth);
}

#[test]
fn depth() {
    let test_matrix = vec![
    vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0],
    vec![0.0, 0.0, 1.0, 2.0, 3.0, 4.0],
    vec![1.0, 0.0, 0.0, 1.0, 2.0, 3.0],
    vec![2.0, 1.0, 0.0, 0.0, 1.0, 2.0],
    vec![3.0, 2.0, 1.0, 0.0, 0.0, 1.0],
    vec![4.0, 3.0, 2.0, 1.0, 0.0, 0.0],
    vec![5.0, 4.0, 3.0, 2.0, 1.0, 0.0],
    ];

    let distance = depth_search::depth_first_search(&test_matrix, 5, 3);

    assert_eq!(distance, [5,0,1,2,3]); //[5,0,1,2,3]
}