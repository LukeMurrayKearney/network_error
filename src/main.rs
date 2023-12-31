use network_error::network_structure::*;
use network_error::calculating_error::*;
use network_error::run_scenarios::*;
// use std::error::Error;
use std::{time::Instant, io::repeat};

fn  main() {

    let n: usize = 100_000;
    let partitions: Vec<usize> = vec![58*n/1000, 145*n/1000, 212*n/1000, 364*n/1000, 497*n/1000, 623*n/1000, 759*n/1000, 866*n/1000, n];
    let period = "1";
    let (network, _) = NetworkStructure::new_multinomial(n, &partitions, period);
    println!("model:");
    model_error3_prob(&network, period, &partitions, 5_000, 10, 5);
    let network = NetworkStructure::new_sbm(n, &partitions, period);
    println!("sbm:");
    model_error3_prob(&network, period, &partitions, 5_000, 10, 5);

    
    // let start_time = Instant::now();
    // let (network, _) = NetworkStructure::new_multinomial(n, &partitions, period);
    // let elapsed_time = start_time.elapsed();
    // println!("{}", elapsed_time.as_secs() as usize);
    
    // let repeats: Vec<i32> = network.adjacency_matrix
    //     .iter()
    //     .map(|x| {
    //             if contains_duplicates(x) {
    //                 1
    //             } else {
    //                 0
    //             }
    //     })
    //     .collect();
    // println!("{:?}", repeats.iter().sum::<i32>());
    
    // let ns: Vec<usize> = (20..=100).map(|x| x*1_000).collect();
    // test_error_of_stubbing(ns);

}





fn contains_duplicates(items: &[Link]) -> bool {
    for (i, &item1) in items.iter().enumerate() {
        for (j, &item2) in items.iter().enumerate() {
            if i != j && item1.i == item2.i && item1.j == item2.j {
                return true;
            }
        }
    }
    false
}