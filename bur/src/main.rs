fn main(){
  use {rand::*,std::io::*};
  loop{
    let i=["Salad","Sauce","Burger","Pickle","Tomato","Hamster","Rock","Sand","Kanade's Ashes","Cat","Soda","Pain"];
    let e=i.len();
    println!("| Bun");
    for _ in 0..thread_rng().gen_range(0..(e*2)){
      let r=i[thread_rng().gen_range(0..e)];
      println!("|   {r}");
    }
    println!("| Bun");
    let mut l=String::new();
    stdin().read_line(&mut l);
    if &l=="@\n"{break};
  }
}
