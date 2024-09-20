use lua_deserializer::deserializer::Deserializer;

fn main() {
    let mut deserializer = Deserializer::from_file("luac.out");
    let chunk = deserializer.deserialize();

    println!("{:?}", chunk);
}
