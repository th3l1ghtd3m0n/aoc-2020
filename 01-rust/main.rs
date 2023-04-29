fn main()
{
    // Part 1
    let text1 = std::fs::read_to_string("input1.txt").unwrap();
    let input1 = text1.split('\n')
        .filter(|x| !x.is_empty())
        .map(|y| y.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let input1_len = input1.len();

    for i in 0..input1_len - 1
    {
        for j in i + 1..input1_len 
        {
            if input1[i] + input1[j] == 2020
            {
                print!("- Part 1:\n");
                println!("[+] {}\n", input1[i] * input1[j]);
                break;
            }
        }
    }

    // Part 2
    let text2 = std::fs::read_to_string("input2.txt").unwrap();
    let input2 = text2.split('\n')
        .filter(|x| !x.is_empty())
        .map(|y| y.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let input2_len = input2.len();

    for i in 0..input2_len - 2 
    {
        for j in i + 1..input2_len - 1
        {
            for k in j + 1..input2_len 
            {
                if input2[i] + input2[j] + input2[k] == 2020 
                {
                    print!("- Part 2:\n");
                    println!("[+] {}", input2[i] * input2[j] * input2[k]);
                    break;
                }
            }
        }
    }
}
