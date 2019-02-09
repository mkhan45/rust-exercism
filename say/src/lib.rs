pub fn encode(n: u64) -> String {
    let mut output = String::new();
    let mut n = n;

    while n >= 10{
        n = n/10;
        match n%10 {
            0 => output += "zero",
            1 => output += "one",
            2 => output += "two",
            3 => output += "three",
            4 => output += "four",
            5 => output += "five",
            6 => output += "six",
            7 => output += "seven",
            8 => output += "eight",
            9 => output += "nine",
            _ => output += "???",
        };

        output += " ";
    };

    output += &n.to_string();

    output

}
