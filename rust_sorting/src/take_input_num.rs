pub fn take_num_input() -> u32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap(); // NOTE: unwrap means program will panic if
                                                     // string cannot be read
    input.trim().parse::<u32>().unwrap() // NOTE: Use unwrap carefully cause it make the program
                                         // panic if an error occurs PANIC == PROGRAM CRASHES!!!
}
