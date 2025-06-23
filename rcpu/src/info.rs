pub mod cpu {
    use std::fs;
    use std::io;
    use std::thread;
    use std::time::Duration;
    
    const STAT_PATH: &'static str = "/proc/stat";

    fn get_data() -> io::Result<(u64, u64)> {
        // read the file, omit the 'cpu', take 10 values it has
        // reference: 
        let values: Vec<u64> = fs::read_to_string(STAT_PATH)
            .expect("")
            .split_whitespace()
            .skip(1)
            .take(10)
            .map(|e| e.parse::<u64>().unwrap())
            .collect();
        
        println!("{:?}\n", values);

        // idle + iowait
        let idle: u64 = values[3] + values[4];

        // total
        let total: u64 = values.iter().sum();

        println!("idle: {}\ntotal: {}", &idle, &total);

        Ok((total, idle))
    }

    pub fn get_cpu_load() -> Result<i32, Box<dyn std::error::Error>> {
        // get cpu stats at two moments of time
        let (prev_total, prev_idle) = get_data()?;
        thread::sleep(Duration::from_millis(1000)); // poll interval
        let (curr_total, curr_idle) = get_data()?;

        let total_diff = curr_total - prev_total;
        let idle_diff = curr_idle - prev_idle;

        if total_diff == 0 {
            return Ok(0);
        }

        // compare idle time vs total work time - that basically is the load percentage
        // truncate unnecessary bytes, as percentage is never >100
        let usage: i32 = (100 - idle_diff * 100 / total_diff) as i32;
        Ok(usage)
    }
}

pub mod ram {

}