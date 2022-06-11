const STARTING_MISSILES:i32 = 8;
const READY_AMOUNT:i32 = 2;

fn main() {
// let mut missiles:i32 = 8;
// let mut ready:i32 = 2;

// println!("Firing {} of my {} missiles ...", ready , missiles);
// missiles = missiles - ready;
// println!("Remaining missiles: {}", missiles);



// missiles = STARTING_MISSILES;
// ready = READY_AMOUNT;

let  (missiles, ready, _obsolete):(i32,i32, bool) = (STARTING_MISSILES, READY_AMOUNT, true);
println!("Firing {} of my {} missiles ...", ready , missiles);
println!("Remaining missiles: {}", missiles-ready);

}
