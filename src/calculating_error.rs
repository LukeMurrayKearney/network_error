use std::vec;

use crate::read_in::read_egos;
use crate::network_structure::NetworkStructure;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

pub fn model_error1(network: &NetworkStructure, period: &str, partitions: &Vec<usize>) -> Vec<f64> {
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


pub fn model_error2(network: &NetworkStructure, period: &str, partitions: &Vec<usize>) -> Vec<f64> {
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
            let mut diff: usize = ego.1.iter()
                .zip(model[i].1.iter())
                .map(|(&a, &b)| (a as isize - b as isize).abs() as usize)
                .sum::<usize>();
            diff += (ego.1.iter().sum::<usize>() as isize - model[i].1.iter().sum::<usize>() as isize).abs() as usize;
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
    error.iter().enumerate().map(|(i, &x)| (x as f64)/((2*data_breakdown[i]) as f64)).collect()
}

pub fn model_error3_prob(network: &NetworkStructure, period: &str, partitions: &Vec<usize>, 
    sample_size: usize, number_of_samples: usize, error_threshold: usize) {
    
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
    let mut rng: ThreadRng = rand::thread_rng();
    let mut log_probs: Vec<f64> = vec![0.0; number_of_samples];
    // random sample from model 
    for i in 0..number_of_samples {
        //probability of each ego 
        let mut log_sums: Vec<f64> = vec![0.0; data.len()];
        let mut sample: Vec<(usize, Vec<usize>)> = model.choose_multiple(&mut rng, sample_size).cloned().collect();
        // number in each age group of samples
        sample.sort_by_key(|k| k.0);
        let mut sample_groups: Vec<usize> = vec![0; partitions.len()];
        for x in sample.iter() {
            sample_groups[x.0] += 1; 
        }
        let mut tmp: usize = 0;
        // partitions in the sample set
        let sample_partitions: Vec<usize> = sample_groups.iter().map(|&x| {
            tmp += x;
            tmp
        }).collect();
        // loop over data ego networks
        for (j, ego) in data.iter().enumerate() {
            let mut sum: usize = 0;
            // calculate difference between ego and model samples of same age group
            for i in (sample_partitions[ego.0] - sample_groups[ego.0])..sample_partitions[ego.0] {
                let diff: usize = ego.1.iter()
                    .zip(sample[i].1.iter())
                    .map(|(&a, &b)| (a as isize - b as isize).abs() as usize)
                    .sum::<usize>();
                if diff <= error_threshold {
                    sum += 1
                }
            }
            if sum != 0 {
                log_sums[j] = ((sum as f64) / (sample_groups[ego.0] as f64)).ln();
            }
        }
        log_probs[i] = log_sums.iter().sum::<f64>();
    }
    println!("{:?}", log_probs);
}