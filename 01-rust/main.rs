// O_o
fn main()
{
    // Part 1
    let text = std::fs::read_to_string("example_input.txt").unwrap();
    let mut input = text.split('\n')
        .filter(|x| !x.is_empty())
        .map(|y| y.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let input_len = input.len();
    input.sort();

    for i in 0..input_len - 1 
    {
        if let Ok(j) = input.binary_search(&(2020 - input[i]))
        {
            if i != j
            {
                println!("- Part 1:");
                println!("[+] {}\n", input[i] * input[j]);
                break;
            }
        }
    }
    // Part 2
    for i in 0..input_len - 1
    {
        for j in i + 1..input_len 
        {
            if let Ok(k) = input.binary_search(&(2020 - input[i] - input[j]))
            {
                if j != k
                {
                    println!("- Part 2:");
                    println!("[+] {}", input[i] * input[j] * input[k]);
                    return;
                }
            }
        }
    }
}
