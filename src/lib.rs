use std::collections::HashMap;

#[cfg_attr(target_os = "linux", link_section = ".init_array")]
pub static LD_PRELOAD_INITIALISE_RUST: extern "C" fn() = self::ld_preload_initialise_fn;

extern "C" fn ld_preload_initialise_fn() {
    // Did some test with a panicking function.
    // panic!("LET'S FAIL!");

    // Just printing something
    println!("HOLA!");

    // Manipulating a complex data structure
    let mut map = HashMap::new();

    map.insert("key1", "value1");
    map.insert("key2", "value2");
    map.insert("key3", "value3");

    println!("{:?}", map);

    // End!
    println!("Bye!");
}
