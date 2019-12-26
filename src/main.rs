fn main() {
  let s : String = "123456789".to_string();
  //There are 8 places in which we can place a + or a -
  //Either we put a +, a -, or we put nothing.
  //This gives us 3 options for each space, which results in 3^8 total possibilities.
  //To iterate over all these possiblilities, they can each be represented by a base 3 number in the range 0..3^8-1
  //Then, each possibility can be computed, and printed if it evaluates to 100.
  let end : i32 = 3_i32.pow(8)-1;
  let mut solutions : i32 = 0;
  for x in 0..end {
    let mut equation : String = String::new();
    let mut ternary : i32 = x;
    let mut value : i32 = 0;
    let mut result : i32 = 0;
    let mut operation : i32 = 1; //1 for addition, 0 for subtraction
    for c in s.chars() {
      equation.push(c);
      value = value * 10 + c as i32 - '0' as i32;
      if ternary % 3 == 1 {
        equation += " + ";
        if operation == 1 {
          result += value;
        } else {
          result -= value;
        }
        operation = 1;
        value = 0;
      } else if ternary % 3 == 2 {
        equation += " - ";
        if operation == 1 {
          result += value;
        } else {
          result -= value;
        }
        operation = 0;
        value = 0;
      }
      ternary /= 3;
    }
    if operation == 1 {
      result += value;
    } else {
      result -= value;
    }
    if result == 100 {
      println!("{} = 100",equation);
      solutions += 1;
    }
  }
  println!("There are {} total solutions.",solutions);
}
