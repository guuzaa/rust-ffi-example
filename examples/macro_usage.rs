use ffi_example::packet;

fn main() {
    println!("=== packet! Macro Examples ===\n");

    // Create an empty packet
    let empty = packet![];
    println!("Empty packet: {:?}", empty);

    // Create a packet with specific values
    let values = packet![1, 2, 3, 4, 5];
    println!("Packet with values: {:?}", values);

    // Create a packet with repeated values
    let repeated = packet![42; 5];
    println!("Packet with repeated values: {:?}", repeated);

    // Create a single-value packet
    let single = packet![123];
    println!("Single value packet: {:?}", single);

    // Create a large packet with zeros
    let large = packet![0; 10];
    println!("Large packet (first 5 elements): {:?}", &large[..5]);

    // Demonstrate slice syntax
    println!("\nSlice syntax examples:");
    let data = packet![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    println!("  Full packet: {:?}", &data[..]);
    println!("  First 3 elements: {:?}", &data[..3]);
    println!("  Last 3 elements: {:?}", &data[7..]);
    println!("  Middle elements (3-7): {:?}", &data[3..7]);
    println!("  First 5 elements (inclusive): {:?}", &data[..=4]);
    println!("  Last 5 elements (inclusive): {:?}", &data[5..=9]);

    // Demonstrate iteration
    println!("\nIterating over packet values:");
    for (i, &value) in values.into_iter().enumerate() {
        println!("  [{}] = {}", i, value);
    }

    // Demonstrate indexing
    println!("\nAccessing elements by index:");
    println!("  values[0] = {}", values[0]);
    println!("  values[2] = {}", values[2]);
    println!("  repeated[3] = {}", repeated[3]);
}
