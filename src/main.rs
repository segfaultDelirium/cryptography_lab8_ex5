fn convert_hex_string_to_values_vec(hex_string: String) -> Vec<u8>{
    let mut res_vec = vec![];

    let limit = hex_string.len() / 2;
    let mut chars = hex_string.chars();
    for i in 0..limit{
        let char1 = chars.next().unwrap();
        let char2 = chars.next().unwrap();
        let value = i64::from_str_radix(&format!("{char1}{char2}"), 16).unwrap();
        res_vec.push(value as u8);
    }

    res_vec
}

fn create_binary(value: u32, bits_count: i32) -> Vec<u8>{
    fn create_hex_binary_rec(hex_value: u32, counter: i32, acc: Vec<u8>) -> Vec<u8>{
        if counter < 0 {
            return acc;
        }
        let two_value = (2 as u32).pow(counter as u32) as u8;
        let new_counter = counter - 1;
        if hex_value >= two_value as u32{
            let new_acc = functional_push_right(acc, 1);
            create_hex_binary_rec(hex_value - two_value as u32, new_counter, new_acc)
        }else{
            let new_acc = functional_push_right(acc, 0);
            create_hex_binary_rec(hex_value, new_counter, new_acc)
        }
    }
    create_hex_binary_rec(value as u32, bits_count - 1, vec![])
}


fn functional_push_right(vec: Vec<u8>, value: u8) -> Vec<u8> {

    let mut vec_clone = vec.clone();
    vec_clone.push(value);
    vec_clone
}

fn euclid_modulo(x: u32, m: u32) -> u32{
    x % m
}

fn find_first_congruent(start: u32, m: u32, to_compare: u32) -> Option<u32>{
    for i in start..m{
        if euclid_modulo(i, m) == to_compare {
            return Some(i);
        }
    }
    return None;
}

fn main() {

    let h0 = "67452301";
    let h1 = "EFCDAB89";
    let h2 = "98BADCFE";
    let h3 = "10325476";
    let h4 = "C3D2E1F0";

    let h0_as_values_vec = convert_hex_string_to_values_vec(h0.to_string());
    println!("h0_as_values_vec: {:?}", h0_as_values_vec);

    let mut binary: Vec<u8> = h0_as_values_vec.into_iter()
        .map(|value| create_binary(value as u32, 8))
        .flatten()
        .collect();
    binary.push(1);

    let current_binary_len = binary.len();
    let first_congruent = find_first_congruent(current_binary_len as u32, 512, 448).unwrap();
    println!("first_congruent: {:?}", first_congruent);
    let count_of_zeros_to_push = first_congruent - current_binary_len as u32d;
    for i in 0..count_of_zeros_to_push{
        binary.push(0);
    }

    let current_binary_len_as_binary = create_binary(current_binary_len as u32, 64);
    println!("current_binary_len_as_binary: {:?}", current_binary_len_as_binary);


    println!("binary: {:?}", binary);

    println!("Hello, world!");
}
// zrobic to zadanie (5 punktow) do przyszlego tygodnia do konca niedzieli 23:50