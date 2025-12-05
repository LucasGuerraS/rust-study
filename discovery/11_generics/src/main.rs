#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile,
}
#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time: String,
}

impl ChatMessage<DigitalContent> {
    fn consume_content(&self) {
        println!("Watching {:?}", self.content)
    }
}

impl<T> ChatMessage<T> {
    fn retrieve_time(&self) -> String {
        self.time.clone()
    }
}

fn main() {
    let slice_content = ChatMessage {
        content: "Nichijou",
        time: String::from("Foo"),
    };
    let string_content = ChatMessage {
        content: String::from("Nichijou"),
        time: String::from("Foo"),
    };
    let enum_content = ChatMessage {
        content: DigitalContent::AudioFile,
        time: String::from("Foo"),
    };

    slice_content.retrieve_time();

    string_content.retrieve_time();

    enum_content.retrieve_time();
    enum_content.consume_content();

    let enum_content = ChatMessage {
        content: DigitalContent::VideoFile,
        time: String::from("Foo"),
    };

    enum_content.retrieve_time();
    enum_content.consume_content();

    examples();
}

fn examples() {
    println!("{}", identity(5));
    println!("{:?}", make_tuple("foo", "bar"));
    let gold_chest = TreasureChest {
        captain: String::from("Jack Sparrow"),
        treasure: "Gold",
    };
    println!(
        "{gold_chest:?} {}, {}",
        gold_chest.captain, gold_chest.treasure
    );
    gold_chest.capital_captain();
}

fn identity<T>(value: T) -> T {
    value
}

fn make_tuple<T, U>(a: T, b: U) -> (T, U) {
    (a, b)
}

#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T,
}

impl<T> TreasureChest<T> {
    fn capital_captain(&self) {
        println!("{}", self.captain.to_uppercase())
    }
}
