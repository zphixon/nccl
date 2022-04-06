use rand::seq::SliceRandom;

fn main() {
    let content = std::fs::read_to_string("examples/big.nccl").unwrap();

    let start = std::time::Instant::now();

    let config = nccl::parse_config(&content).unwrap();
    let num = walk(&config);

    let end = std::time::Instant::now();
    let elapsed = end - start;
    println!("finished {elapsed:?}");

    assert_eq!(num, 65535);

    let start = std::time::Instant::now();
    for _ in 1..=65535 {
        let _random = random(&config);
    }
    let end = std::time::Instant::now();
    let elapsed = end - start;
    println!("finished {elapsed:?}");
}

fn walk(config: &nccl::Config) -> usize {
    let mut acc = config.children().count();
    for child in config.children() {
        acc += walk(child);
    }
    acc
}

fn random<'a>(config: &nccl::Config<'a>) -> Vec<&'a str> {
    let mut vec = Vec::new();
    random_rec(config, &mut vec);

    vec
}

fn random_rec<'a>(config: &nccl::Config<'a>, acc: &mut Vec<&'a str>) {
    let children = config.children().collect::<Vec<_>>();
    let random = children.choose(&mut rand::thread_rng());

    if let Some(random) = random {
        acc.push(random.key());
        random_rec(random, acc);
    }
}
