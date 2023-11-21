fn insertion_sort(arr: &Vec<i64>, n:i64)->Vec<i64>
{
    let mut narr = arr.clone();
    let _i:i64 = 0;
    let mut j:i64;
    for _i in 0..n{
        j = _i;
        while j>0 && narr[<i64 as TryInto<usize>>::try_into(j-1).unwrap()] > narr[<i64 as TryInto<usize>>::try_into(j).unwrap()]{
            narr.swap(j.try_into().unwrap(), (j-1).try_into().unwrap());
            j -= 1;
        }
    }
    return narr;
}

fn main() {
    let arr: Vec<i64> = vec!(5, 4, 2, 7, 3, 1, 9, 8, 6);
    println!("{:#?}", arr);
    let new_arr: Vec<i64> = insertion_sort(&arr, 9);
    println!("{:#?}", new_arr);
}