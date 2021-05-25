
use rand::Rng;

pub fn generate_pattern() -> String {
    let mut pattern = String::from("");
    generate_expression(&mut pattern, 100, true);
    pattern
}

pub fn generate_subject() -> String {
    let mut subject = String::from("");
    generate_expression(&mut subject, 100, false);
    subject
}

fn generate_expression(string: &mut String, func_chance: u8, is_pattern: bool) {
    if chance(func_chance) {
        generate_funcion(string, is_pattern);
    } else {
        append_variable(string, is_pattern);
    }
}

fn generate_funcion(string: &mut String, is_pattern: bool) {
    append_function_name(string, is_pattern);
    string.push('[');
    let mut first = true;
    let mut c = 90.0;
    loop {
        if !chance(c as u8) {
            break;
        }
        if !first {
            string.push_str(", ")
        }
        first = false;
        generate_expression(string, 20, is_pattern);
        c *= 0.95;
    }
    string.push(']');
}

fn append_function_name(string: &mut String, is_pattern: bool) {
    let mut rng = rand::thread_rng();
    string.push('f');
    string.push(rng.gen_range('a'..including('z')));
	if is_pattern && chance(30) {
		string.push('_');
	}
}

// Constant: Symbol (a - e) or number
// Variable: 1 2 or 3 _ u - z
fn append_variable(string: &mut String, is_pattern: bool) {
    let mut rng = rand::thread_rng();
    if is_pattern {
        if chance(50) { // Constant
            if chance(40) {
                string.push('s');
                string.push(rng.gen_range('a'..including('z')));
            } else {
                string.push(rng.gen_range('0'..including('9')));
            }
        } else {
            string.push('v');
			let underlines: u8 = rng.gen_range(0..4);
            string.push((underlines + 48) as char);
            string.push(rng.gen_range('a'..including('z')));
			for _ in 0..underlines {
				string.push('_');
			}
		}
    } else {
        if chance(40) {
            string.push('s');
            string.push(rng.gen_range('a'..including('z')));
        } else {
            string.push(rng.gen_range('0'..including('9')));
        }
    }
}

fn including(c: char) -> char {
    (c as u8 + 1) as char
}

fn chance(percent: u8) -> bool {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..101) <= percent
}