use crate::network_structure::NetworkStructure;
use crate::calculating_error::*;
use crate::output_files::*;
use crate::useful_functions::*;
use std::error::Error;

pub fn test_error_of_stubbing(ns: Vec<usize>) -> Result<(), Box<dyn Error>> {
    let mut results1: Vec<MeanVar> = Vec::new();
    let mut results2: Vec<MeanVar> = Vec::new();
    for (i, n) in ns.iter().enumerate() { 
        let partitions: Vec<usize> = vec![58*n/1000, 145*n/1000, 212*n/1000, 364*n/1000, 497*n/1000, 623*n/1000, 759*n/1000, 866*n/1000, *n];
        let mut tmp1: Vec<(f64,f64)> = Vec::new(); 
        let mut tmp2: Vec<(f64,f64)> = Vec::new();
        for _ in 0..2 {
            let (network,_) = NetworkStructure::new_multinomial(n.clone(), &partitions, "1");
            let tmp_model = model_error2(&network, "1", &partitions);
            let network = NetworkStructure::new_sbm(n.clone(), &partitions, "1");
            let tmp_sbm = model_error2(&network, "1", &partitions);
            tmp1.push((tmp_model.iter().sum::<f64>(), tmp_sbm.iter().sum::<f64>()));
        }
        for _ in 0..2 {
            let (network,_)  = NetworkStructure::new_multinomial(n.clone(), &partitions, "2");
            let tmp_model = model_error2(&network, "2", &partitions);
            let network = NetworkStructure::new_sbm(n.clone(), &partitions, "2");
            let tmp_sbm = model_error2(&network, "2", &partitions);
            tmp2.push((tmp_model.iter().sum::<f64>(), tmp_sbm.iter().sum::<f64>()));
        }
        let mut proportions: Vec<Vec<f64>> = Vec::new();
        proportions.push(tmp1.iter().map(|&(x,y)| (y-x)/(y)).collect::<Vec<f64>>());
        proportions.push(tmp2.iter().map(|&(x,y)| (y-x)/(y)).collect::<Vec<f64>>());
        results1.push(MeanVar::new(mean(&proportions[0]).unwrap(), variance(&proportions[0]).unwrap()));
        results2.push(MeanVar::new(mean(&proportions[1]).unwrap(), variance(&proportions[1]).unwrap()));
        if n % 1_000 == 0 {
            println!("{}", i);
        }
    }
    
    write_json_mean_var(vec![results1], "csv_output/error2_period1.json")?;
    write_json_mean_var(vec![results2], "csv_output/error2_period2.json")?;
    
    Ok(())
}
