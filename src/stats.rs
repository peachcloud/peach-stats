use std::result::Result;

use probes::*;
use serde::Serialize;
use snafu::ResultExt;

use crate::error::*;

#[derive(Debug, Serialize)]
pub struct CpuStat {
    user: u64,
    system: u64,
    idle: u64,
    nice: u64,
}

#[derive(Debug, Serialize)]
pub struct CpuStatPercentages {
    user: f32,
    system: f32,
    idle: f32,
    nice: f32,
}

#[derive(Debug, Serialize)]
pub struct LoadAverage {
    one: f32,
    five: f32,
    fifteen: f32,
}

pub fn cpu_stats() -> Result<String, StatError> {
    let cpu_stats = cpu::proc::read().context(ReadCpuStat)?;
    let s = cpu_stats.stat;
    let stats = CpuStat {
        user: s.user,
        system: s.system,
        nice: s.nice,
        idle: s.idle,
    };
    let json_stats = serde_json::to_string(&stats).context(SerdeSerialize)?;

    Ok(json_stats)
}

pub fn cpu_stats_percent() -> Result<String, StatError> {
    let cpu_stats = cpu::proc::read().context(ReadCpuStat)?;
    let s = cpu_stats.stat.in_percentages();
    let stats = CpuStatPercentages {
        user: s.user,
        system: s.system,
        nice: s.nice,
        idle: s.idle,
    };
    let json_stats = serde_json::to_string(&stats).context(SerdeSerialize)?;

    Ok(json_stats)
}

pub fn load_average() -> Result<String, StatError> {
    let l = load::read().context(ReadLoadAvg)?;
    let load_avg = LoadAverage {
        one: l.one,
        five: l.five,
        fifteen: l.fifteen,
    };
    let json_load_avg = serde_json::to_string(&load_avg).context(SerdeSerialize)?;

    Ok(json_load_avg)
}
