use std::collections::HashSet;

use random_stuff::play_with_trait_objects::play_with_trait_objects;
use random_stuff::population_api::{get_pops, Nation};
use random_stuff::sunrise_sunset::scrape_suntimes;
use random_stuff::traits_and_boxes::{List, MyObj, get_nums};
use random_stuff::ResultErr;
use random_stuff::heapsort::HeapSort;

#[tokio::main]
async fn main() -> ResultErr<()> {
    let mut nums = vec![6, 5, 2, 8, 2, 3, 10, 1, 1, 1];
    nums.heap_sort();
    dbg!(nums);
    get_pops().await?;
    play_with_trait_objects()?;
    dbg!(Nation::fake_with_population(5));
    scrape_suntimes().await?;
    let num_cons = List::from_collection(&vec![1, 2, 3, 4, 5, 6]);
    println!("{num_cons}");
    println!("{:?}", List::uncons(num_cons));
    let _num_set: HashSet<usize> = dbg!(get_nums(10));
    let _num_vec: Vec<MyObj> = dbg!(get_nums(5));
    Ok(())
}
