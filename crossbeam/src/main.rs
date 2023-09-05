// use crossbeam;

// //Split Array in half and perform work in separate threads
// fn main() {

//   let arr = &[1, 25, -4, 10];
//   let max = find_max(arr);
//   assert_eq!(max, Some(25))

// }

// fn find_max(arr: &[i32]) -> Option<i32> {
//   const THRESHOLD: usize = 2;

//   if arr.len() <= THRESHOLD {
//     return arr.iter().cloned().max();

//   }

//   let mid = arr.len() / 2;
//   let (left, right) = arr.split_at(mid);

//   crossbeam::scope(|s| {
//     let thread_l = s.spawn(|_| find_max(left));
//     let thread_r = s.spawn(|_| find_max(right));

//     let max_l = thread_l.join().unwrap()?;
//     let max_r = thread_r.join().unwrap()?;

//     Some(max_l.max(max_r));

//   }).unwrap()
// }

// use std::collections::HashMap;

// fn round_price(px: f64) -> f64 {
//     let rounded_price = format!("{:.5e}", px).parse::<f64>().unwrap_or(px);
//     if rounded_price < 0.000001 {
//         // Ensure precision to 6 decimals for very small numbers
//         format!("{:.6}", rounded_price).parse::<f64>().unwrap_or(px)
//     } else {
//         rounded_price
//     }
// }

// /// Round a size (sz) based on szDecimals for the given asset.
// fn round_size(sz: f64, sz_decimals: usize) -> f64 {
//     let multiplier = (10f64).powi(sz_decimals as i32);
//     (sz * multiplier).round() / multiplier
// }

// fn place_order(coin: &str, sz: f64, px: f64, sz_decimals_map: &HashMap<String, usize>) {
//     let rounded_px = round_price(px);
//     let sz_decimals = sz_decimals_map.get(coin).cloned().unwrap_or(0);
//     let rounded_sz = round_size(sz, sz_decimals);

//     println!("Placing order with px {} and sz {}", rounded_px, rounded_sz);
// }
// fn main() {
     
//      let mut sz_decimals_map: HashMap<String, usize> = HashMap::new();
//      sz_decimals_map.insert("BTC".to_string(), 8);
//      sz_decimals_map.insert("ETH".to_string(), 6);
     
 
//      let coin = "BTC"; // Replace with the actual coin name
//     //  let sz = 1.00012345;
//     let sz = 0.0012345 ;
//      let px = 1234.567890123;
 
//      place_order(coin, sz, px, &sz_decimals_map);

    
// }
