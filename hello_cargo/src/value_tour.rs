use std::collections::HashMap;

pub fn value_tour() {
	let authenticated = true;
	if authenticated {
		// todo!()
	}else{
		// todo!()
	}

	// modify value
	let mut total = 0;
	total += 1;
	println!("{}",total);

	// pass to function~
	let name = "KD".to_string();
		print_my_name(name);

	let map:HashMap<String, String> = HashMap::new();
	print_map(&map);

	fn print_my_name(name:String){
		println!("{}",name);
	}

	fn print_map(map:&HashMap<String, String>){
		// todo!();
		println!("{:?}",map);
	}
}