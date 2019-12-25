fn main() {
    let input = include_str!("input");
    let result: i32 = input.split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i32>().unwrap())
        .map(|mass| mass/3 - 2)
        .sum();
    println!("Naive total fuel required: {}", result);

    let result: i64 = input.split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i64>().unwrap())
        .map(|mut mass| {
            let mut fuel: i64 = 0;
            loop {
                mass = mass/3 - 2;
                if mass < 0 {
                    break
                }
                fuel += mass;
            }
            fuel
        })
        .sum();
    println!("Total fuel required: {}", result);
}
