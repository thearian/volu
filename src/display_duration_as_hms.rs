use std::time::Duration;

pub trait Hms {
    fn to_hms(&self) -> String;
}

impl Hms for Duration {
    fn to_hms(&self) -> String {
        let milli_seconds: u32 = self.as_millis() as u32;
        if milli_seconds < 1000 {
            return format!("{}ms", milli_seconds)
        }

        let millis = milli_seconds % 1000;
        let seconds: u32 = milli_seconds / 1_000;
        if seconds < 60 {
            return format!("{}.{}s", seconds, millis)
        }
        
        let secs: u32 = seconds % 60;
        let mins: u32 = seconds / 60;
        return format!("{}:{}s {}ms", mins, secs, millis);
    }
}