pub mod cpu {
    use crate::error::RcpuError;
    use std::fs;
    use std::thread;
    use std::time::Duration;
    
    const STAT_PATH: &'static str = "/proc/stat";

    fn get_data() -> Result<(u64, u64), RcpuError> {

        // read the file, omit the 'cpu', take 10 values it has
        // reference: https://www.man7.org/linux/man-pages/man5/proc_stat.5.html
        let values: Vec<u64> = fs::read_to_string(STAT_PATH)?
            .split_whitespace()
            .skip(1)
            .take(10)
            .map(|e| e.parse::<u64>().unwrap())
            .collect();
        
        // idle + iowait
        let idle: u64 = values[3] + values[4];

        // total
        let total: u64 = values.iter().sum();

        Ok((total, idle))
    }

    pub fn get_load() -> Result<i32, RcpuError> {
        // get cpu stats at two moments of time
        let (prev_total, prev_idle) = get_data()?;
        thread::sleep(Duration::from_millis(100)); // poll interval
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