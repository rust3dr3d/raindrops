# Raindrops

### **Introduction**
This is based on the easy challenge hosted on [excercism.io](https://exercism.io/) Rust track.

I'm simply playing the actual sound file instead of onomatopoeia using [rodio crate](https://crates.io/crates/rodio).

Code is commented for your convenience. Have fun!

### **Cargo.toml**

```toml
[package]
name = "raindrops"
version = "0.1.0"
authors = ["RustedRed"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rand = "0.8.3"
rodio = "0.13.1"
```

### **Playsound.rs**

```rust
use rodio::{Decoder, OutputStream, source::Source};
use std::fs::File;
use std::io::BufReader;


// Play specified file on the Primary sound device
pub fn play_ogg(ogg_file:&str){
   
    //Get default sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    
    //Open .ogg files in the root directory
    let ogg_file = BufReader::new(File::open(ogg_file).unwrap());
    
    //Use the decoder to decode the above ogg file from BufReader
    let source = Decoder::new(ogg_file).unwrap();

    //Play the decoded ogg file using the sound device handler(line 8)
    stream_handle.play_raw(source.convert_samples()).unwrap();
    
    //Keep main thread alive while sound plays on a different thread
    std::thread::sleep(std::time::Duration::from_secs(2));
}
```

### **Main.rs**

```rust
use rand::Rng;

mod playsound;

fn gen_rand_raindrops(){
    let mut rnd_rng = rand::thread_rng();
    
    for i in 0..6{
        //Generate 5 Random numbers(Note in the for loop 0 is inclusive and 6 is exclusive)
        let rnd_no = rnd_rng.gen::<u8>(); 
        
        //If there's a string output Some(n) play respective sound file
        if let Some(n) = raindrops(rnd_no as u32){
            println!("{}. Raindrop Sound for N:{} is {}",i, rnd_no, n);

            // This is not really necessary but added as a precaution
            std::thread::sleep(std::time::Duration::from_secs(2));
        }else{
            println!("{}. No sound generated for N:{}",i, rnd_no);
        }
    }
}

//If n is divisible by 3 add "Pling to the returning string. Play Drop1.ogg sound"
//If n is divisible by 5 add "Plang" to the returning string. Play Drop2.ogg sound"
//If n is divisible by 7 add "Plong" to the returning string. Play Drop3.ogg sound"
fn raindrops(n: u32) -> Option<String>{
    //Returning String Buffer
    let mut raindrop_sound = String::new();
    //We are using a tuple that contains...
    // 1. Divisor
    // 2. Onomatopoeia for raindrops for each divisor
    // 3. Actual sound file name with the extension
    // NOTE: Sound files sould be in the root directory or you'll have to specify the full path here
    let divisors:[(u32, &str, &str); 3] = [
        (3, "Pling", "Drop1.ogg"),
        (5, "Plang", "Drop2.ogg"), 
        (7, "Plong", "Drop3.ogg")
        ];

    // For each divisor in the tuple array, divide n by divisor...
    // If there's no remainder, add the respective onomatopoeia to the string(raindrop_sound)
    // Play the respective sound
    // If n is not divisible by any divisor in the array (without a remainder), Return None
    for divisor in divisors.iter(){
        if n % divisor.0 == 0{
            raindrop_sound.push_str(divisor.1);
            playsound::play_ogg(divisor.2);
        }
    }

    if !raindrop_sound.is_empty(){
        Some(raindrop_sound)
    }else{
        None
    }
}


fn main() {
    gen_rand_raindrops();
}

```
