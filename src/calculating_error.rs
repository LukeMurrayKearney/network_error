use crate::read_in::read_egos;
use crate::network_structure::NetworkStructure;

pub fn model_error(network: &NetworkStructure, period: &str, partitions: Vec<usize>) {
    let (contacts, ages) = match period {
        "1" => {
            let contacts = read_egos("data_preprocessing/data/egos_1c.csv");
            let ages = read_egos("data_preprocessing/data/egos_1a.csv");
            (contacts, ages)
        },
        _ => {
            let contacts = read_egos("data_preprocessing/data/egos_2c.csv");
            let ages = read_egos("data_preprocessing/data/egos_2a.csv");
            (contacts, ages)
        }
    };
    let ages: Vec<usize> = ages.into_iter().flatten().collect();
    // Combine vec1 and vec2 into a vector of tuples
    let mut data: Vec<(usize,Vec<usize>)> = ages.into_iter().zip(contacts.into_iter()).collect();
    // Sort combined based on the first element of each tuple
    data.sort_by_key(|k| k.0);
    
    // next we create a similar set of ego networks from the network
    let mut model: Vec<(usize, Vec<usize>)> = Vec::new();
    for (i, links) in network.adjacency_matrix.iter().enumerate() {
        // define individual with age and 0 contacts
        model.push((network.age_brackets[i], vec![0; partitions.len()]));
        for link in links.iter(){
            // add links individuals have with coinciding age group
            model[i].1[network.age_brackets[link.j]] += 1;
        }
    }
    println!("{:?}",model);
}