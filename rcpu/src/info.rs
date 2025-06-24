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

    pub fn get_load() -> Result<u8, RcpuError> {
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
        let usage: u8 = (100 - idle_diff * 100 / total_diff) as u8;
        Ok(usage)
    }
}

pub mod ram {
    use crate::error::RcpuError;
    use std::fs;

    const MEMINFO_PATH: &'static str = "/proc/meminfo";

    fn get_data() -> Result<(u32, u32), RcpuError> {
        // get the first two lines - MemTotal and MemFree
        let mut values: Vec<String> = fs::read_to_string(MEMINFO_PATH)?
            .lines()
            .take(2)
            .map(|s| s.to_string())
            .collect();

        // get the number itself
        let free: u32 = values.pop().unwrap()
            .split_whitespace()
            .filter_map(|p| match p.parse::<u32>() {
                Ok(val) => Some(val),
                Err(_) => None
            })
            .collect::<Vec<u32>>()[1];

        // get the number itself
        let total: u32 = values.pop().unwrap()
            .split_whitespace()
            .filter_map(|p| match p.parse::<u32>() {
                Ok(val) => Some(val),
                Err(_) => None
            })
            .collect::<Vec<u32>>()[1];

        Ok((free, total))
    }

    pub fn get_busy() -> Result<u8, RcpuError>{
        let (free, total) = get_data()?;

        // do I really need to explain this one?
        Ok((100 - free*100/total) as u8)
    }
}