pub mod cpu {
    use crate::error::RcpuError;
    use tokio::fs; // Changed to tokio's fs
    use tokio::time::{sleep, Duration}; // Changed to tokio's sleep

    const STAT_PATH: &'static str = "/proc/stat";

    async fn get_data() -> Result<(u64, u64), RcpuError> {

        // read the file, omit the 'cpu', take 10 values it has
        // reference: https://www.man7.org/linux/man-pages/man5/proc_stat.5.html
        let contents = fs::read_to_string(STAT_PATH).await?;
        let values: Vec<u64> = contents
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

    pub async fn get_load() -> Result<u8, RcpuError> {
        // get cpu stats at two moments of time
        let (prev_total, prev_idle) = get_data().await?;
        sleep(Duration::from_millis(100)).await; // poll interval
        let (curr_total, curr_idle) = get_data().await?;

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
    use tokio::fs;

    const MEMINFO_PATH: &'static str = "/proc/meminfo";

    async fn get_data() -> Result<(u32, u32), RcpuError> {
        
        // read ram data
        let data = fs::read_to_string(MEMINFO_PATH).await?;
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

    pub async fn get_busy() -> Result<u8, RcpuError>{
        let (free, total) = get_data().await?;

        // do I really need to explain this one?
        Ok((100 - free*100/total) as u8)
    }
}

pub mod disk {
    use crate::error::RcpuError;
    use tokio::task;
    // TODO: implement actual passing
    async fn get_disk_usage(_path: Option<&str>) -> Result<(u64, u64), RcpuError> {

        task::spawn_blocking(move || {
            let act_path = "/";

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
                Err(RcpuError::Disk(result))
            }
        }).await.map_err(|_| RcpuError::Disk(-1))?
        
    }

    pub async fn percentage() -> Result<u8, RcpuError> {
        let (total, free) = get_disk_usage(None).await?;
        
        Ok((100 - (free*100/total)) as u8)
    }

    pub async fn bytes() -> Result<(u64, u64), RcpuError> {
        let (total, free) = get_disk_usage(None).await?;
        Ok((total, total-free))
    }
}