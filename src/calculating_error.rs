use crate::read_in::read_egos;
use crate::network_structure::NetworkStructure;

pub fn model_error(network: &NetworkStructure, period: &str, partitions: &Vec<usize>) -> Vec<f64> {
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
    let mut data_breakdown: Vec<usize> = vec![0; partitions.len()];
    for i in 0..data.len() {
        data_breakdown[data[i].0] += 1; 
    }
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
    // calculate group sizes to speed up difference measure
    let mut group_sizes: Vec<usize> = partitions
        .windows(2)
        .map(|pair| {
            pair[1] - pair[0]
        })
        .collect();
    group_sizes.insert(0,partitions[0]);
    // next we need to calculate the difference between each data point and the network,
    let mut error: Vec<usize> = vec![0; partitions.len()];
    for ego in data.iter() {
        let mut tmp_error: usize = std::usize::MAX;
        for i in (partitions[ego.0] - group_sizes[ego.0])..partitions[ego.0] {
            // calculate the difference between the contact networks of the individual and each node
            let diff: usize = ego.1.iter()
                .zip(model[i].1.iter())
                .map(|(&a, &b)| (a as isize - b as isize).abs() as usize)
                .sum::<usize>();
            // update minimum difference value
            if diff == 0 {
                tmp_error = diff;
                break; 
            }
            if tmp_error > diff {
                tmp_error = diff;
            }
        }
        error[ego.0] += tmp_error;
    }
    error.iter().enumerate().map(|(i, &x)| (x as f64)/(data_breakdown[i] as f64)).collect()
    //also should email emma gimma and ask jade if its ok if i can do the talk instead of the phd meeting
}