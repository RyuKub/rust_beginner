mod meetings;
mod greetings;

use greetings::evening::*;
use greetings::morning::*;
fn main() {
    meetings::hello();
    meetings::goodbye();
    
    good_morning();
    good_evening();
}
