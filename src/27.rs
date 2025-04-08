fn main() {
    let mut count = 0;
    loop {
        println!("Enter 'a' to add an element or 'q' to quit");
        let input = std::io::stdin().read_line().expect("Failed to read line");

        if input.trim_end() == "a" {
            println!("Please enter the number of elements you want to add:");
            count += 1;
        } else if input.trim_end() == "q" {
            break;
        } else {
            let element = std::str::from_utf8(&input).unwrap();
            match count {
                0 => (),
                _ when !element.is_empty() => println!("Adding element: {}", element),
                _ when element.len() < 2 => panic!("Element is too short"),
                _ => println!("Element not enough length, skipping."),
            }
        }
    }

    println!("Finished adding {} elements.", count);
}
