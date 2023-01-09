// Define a struct which represents a stop
#[allow(dead_code)]
#[derive(Debug)]
pub struct Stop {
    index: u32,
    id: u32,
    avg_load: f32,
    s_mean: f32,
    routes: Vec<u32>,
    connections: Vec<u32>,
}

// Define a function that takes a list of tuples and returns a vector of Stop instances
pub fn create_stops(stop_data: Vec<(u32, u32, f32, f32, Vec<u32>, Vec<u32>)>) -> Vec<Stop> {
    // Create a new vector to store the Stop instances
    let mut stops = Vec::new();

    // Iterate over the list of tuples
    for (index, id, avg_load, s_mean, routes, connections) in stop_data {
        // Create a new Stop instance and add it to the vector
        stops.push(Stop {
            index,
            id,
            avg_load,
            s_mean,
            routes,
            connections,
        });
    }
    stops
}

//Define a function which creates an adjacency matrix
pub fn create_adjacency_matrix(nodes: Vec<Stop>) -> Vec<Vec<f32>> {
    let num_nodes = nodes.len();
    let mut adjacency_matrix:Vec<Vec<f32>> = vec![vec![0.0; num_nodes]; num_nodes];

    for stop1 in &nodes {
        let stop1 = stop1;
        for &c in stop1.connections.iter() {
            if &c != &0 {
                for stop2 in &nodes{
                    if stop2.id == c {
                        let stop2 = stop2;
                        adjacency_matrix[stop1.index as usize][stop2.index as usize] = stop1.avg_load - stop2.avg_load;
                    }
                }
            }
        }
    }
    adjacency_matrix
}