use mergesort::mergesort::sort;

fn main() {
    let mut in_vec = vec![];

    loop {
        let mut inp = String::new();
        std::io::stdin().read_line(&mut inp);

        match inp.trim().parse::<i32>() {
            Ok(num) => {
                in_vec.push(num);
            },
            Err(_) => break
        };
    }

    sort(&mut in_vec);

    println!("Sorted vector: {:?}", in_vec);

}
