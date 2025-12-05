#[derive(Debug)]
enum Tier {
    Gold,
    Silver,
    Platinum,
}

#[derive(Debug)]
enum Subscription {
    Free,
    Basic(f64, u32),
    Premium { tier: Tier },
}

impl Subscription {
    fn summarize(&self) {
        match self {
            Subscription::Free => {
                println!("You have limited access to the site")
            }

            Subscription::Basic(price, period) => {
                println!(
                    "You have limited access to the site's premium features for {price} for {period} months"
                );
            }

            Subscription::Premium { tier } => {
                println!(
                    "You have full access to the site's premium features. Your tier is {tier:?}"
                )
            }
        }
    }
}

fn main() {
    let free = Subscription::Free;
    let basic = Subscription::Basic(100.00, 12);
    let premium = Subscription::Premium {
        tier: Tier::Platinum,
    };
    premium.summarize();
    let premium = Subscription::Premium { tier: Tier::Gold };
    premium.summarize();
    let premium = Subscription::Premium { tier: Tier::Silver };

    free.summarize();
    basic.summarize();
    premium.summarize();
    examples();
}

#[derive(Debug)]
enum CardSuite {
    Hearts(u8),
    Diamonds { rank: u8 },
    Spades(u8),
    Clubs(u8),
}

impl CardSuite {
    fn read_card(&self) {
        print!("The card is: ");
        let rank = match self {
            CardSuite::Clubs(r) | CardSuite::Hearts(r) | CardSuite::Spades(r) => r,
            CardSuite::Diamonds { rank: r } => r,
        };
        println!("a {} of {:?}", rank, self);
    }
}

enum OperatingSystem {
    MacOS,
    Windows,
    Linux,
}

fn examples() {
    let first_card = CardSuite::Diamonds { rank: 8 };
    let second_card = CardSuite::Hearts(8);
    first_card.read_card();
    second_card.read_card();

    let my_computer = OperatingSystem::MacOS;
    let my_windows = OperatingSystem::Windows;
    let my_linux = OperatingSystem::Linux;
    println!("My OS is {} years old", get_os_age(my_computer));
    println!("My OS is {} years old", get_os_age(my_windows));
    println!("My OS is {} years old", get_os_age(my_linux));

    let my_other_card = CardSuite::Spades(3);

    let my_clubs = CardSuite::Clubs(3);

    if let CardSuite::Spades(rank) = my_other_card {
        println!("The spades card rank is {}", rank);
    };

    let CardSuite::Clubs(rank) = my_clubs else {
        println!("The spades card rank is");
        return;
    };

    println!("Your clubs card rank is {}", rank);
}

fn get_os_age(os: OperatingSystem) -> u8 {
    match os {
        OperatingSystem::Linux => 34,
        OperatingSystem::Windows => 39,
        OperatingSystem::MacOS => 23,
    }
}
