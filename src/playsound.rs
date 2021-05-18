use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};


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