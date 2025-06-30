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
        let idle: u64 = values.get(3).ok_or_else(|| RcpuError::Cpu("Invalid idle time kernel data"))? 
                        + values.get(4).ok_or_else(|| RcpuError::Cpu("Invalid iowait time kernel data"))?;

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
        
        // read ram data
        let data = fs::read_to_string(MEMINFO_PATH)?;
        let mut total = 1;
        let mut free = 0;
        
        // find every needed line, assign values
        for line in data.lines() {
            let split = line.split_whitespace().collect::<Vec<&str>>();
            match split.get(0).copied().ok_or_else(|| RcpuError::Ram("Invalid kernel data"))? {
                "MemTotal:" => total = split[1].parse()?,
                "MemAvailable:" => free = split[1].parse()?,
                _ => {}
            }
        }

        Ok((free, total))
    }

    pub fn get_busy() -> Result<u8, RcpuError>{
        let (free, total) = get_data()?;

        // do I really need to explain this one?
        Ok((100 - free*100/total) as u8)
    }
}

pub mod disk {
    use crate::error::RcpuError;
    // TODO: implement actual passing
    fn get_disk_usage(path: Option<&str>) -> Result<(u64, u64), RcpuError> {
        let act_path = match path {
            Some(path) => path,
            None => "/"
        };

        let c_path = std::ffi::CString::new(act_path)?;
        let mut stat: libc::statvfs = unsafe { std::mem::zeroed() };

        // execute syscall
        let result = unsafe { libc::statvfs(c_path.as_ptr() as *const libc::c_char, &mut stat) };

        if result == 0 {
            let block_size = stat.f_frsize as u64; // fundamental block size
            let total = stat.f_blocks * block_size;
            let free = stat.f_bfree * block_size;
            Ok((total, free))
        } else {
            Err(RcpuError::Disk)
        }
    }

    pub fn percentage() -> Result<u8, RcpuError> {
        let (total, free) = get_disk_usage(None)?;
        
        Ok((100 - (free*100/total)) as u8)
    }

    pub fn bytes() -> Result<(u64, u64), RcpuError> {
        let (total, free) = get_disk_usage(None)?;
        Ok((total, total-free))
    }
}