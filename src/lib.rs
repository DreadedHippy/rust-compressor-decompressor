use std::collections::HashMap;

pub fn encode(string_to_read: String) -> Vec<i32> {
	// println!("Hello, world!");
	let mut map_size: i32 = 256;
	let mut sheet_map: HashMap<String, i32> = HashMap::with_capacity(map_size as usize);
	
	for code in 0..map_size {
		sheet_map.insert((char::from_u32(code as u32).unwrap()).to_string(), code);
	}

	// let string_to_read = "geekific-geekific".to_string();
	
	let mut found_chars = "".to_string();
	let mut output: Vec<i32> = Vec::new();

	for character in string_to_read.chars() {
		let chars_to_add = format!("{}{}", found_chars, character);

		if sheet_map.contains_key(&chars_to_add) {
			found_chars = chars_to_add;
		} else {
			output.push(*sheet_map.get(&found_chars).unwrap());
			sheet_map.insert(chars_to_add, map_size);
			map_size+=1;
			found_chars = character.to_string();
		}
	}
	

	if !found_chars.is_empty() {
		output.push(*sheet_map.get(&found_chars).unwrap())
	}

	output
}

pub fn decode(code_to_read: Vec<i32>) -> String{
	let mut code_to_read = code_to_read;
	// println!("Hello, world!");
	// let mut code_to_read = [103, 101, 101, 107, 105, 102, 105, 99, 45, 256, 258, 260, 262].to_vec();
	let mut map_size: i32 = 256;
	let mut sheet_map: HashMap<i32, String> = HashMap::with_capacity(map_size as usize);
	
	for code in 0..map_size {
		sheet_map.insert(code, (char::from_u32(code as u32).unwrap()).to_string());
	}

	let mut characters = (char::from_u32(code_to_read.remove(0) as u32)).unwrap().to_string();
	let mut output: Vec<String> = vec![characters.clone()];

	for character_code in code_to_read {
		let entry = match sheet_map.contains_key(&character_code) {
			true => sheet_map.get(&character_code).unwrap().to_owned(),
			false => format!("{}{}", characters, characters.chars().collect::<Vec<char>>()[0])
		};

		output.push(entry.clone());
		sheet_map.insert(map_size, format!("{}", format!("{}{}", characters,  entry.chars().collect::<Vec<char>>()[0])));
		map_size+=1;
		characters = entry;
	}

	output.join("")

}