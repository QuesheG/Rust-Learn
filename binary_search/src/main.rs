fn binary_search(vec: &Vec<i64>, target: i64)-> i64{
    let mut min; 
    let mut max;
    let mut n_tries: i64 = 0;
    max = vec.len() - 1;
    min = 0;

    while max > min{
        n_tries += 1;
        let avrg = (max + min)/2;
        if vec[avrg] == target{
            println!("{}", n_tries);
            return avrg as i64;
        }
        else{
            if vec[avrg] < target {
                min = avrg+1;
            }
            else{
                max = avrg-1;
            }
        }
    }
    println!("target not in list");
    -1
}

fn main() {
    let vec: Vec<i64> = vec!(2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31);
    println!("{}", binary_search(&vec, 17));
}
