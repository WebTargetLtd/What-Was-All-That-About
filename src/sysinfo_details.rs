use core::fmt;
use std::{collections::BTreeMap, net::IpAddr};

use crate::sysinfo::SystemInfo;

#[cfg(feature = "serde_support")]
use serde::Serialize;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize))]
pub struct RemoteEndpoint {
    pub ip: IpAddr,
    pub sys_info: SystemInfo,
}
impl RemoteEndpoint {
    pub fn new(ip: std::net::IpAddr, sys_info: SystemInfo) -> Self {
        RemoteEndpoint { ip, sys_info }
    }
}
impl fmt::Display for RemoteEndpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ip: {}, Threads: {}", self.ip, self.sys_info.cpu_virtual_cores)
    }
}

pub struct RemoteEndpointSummary {
    pub total_cores: u32,
    pub total_endpoints: u32,
}
impl RemoteEndpointSummary {
    pub fn new() -> Self {
        RemoteEndpointSummary {
            total_cores: 0,
            total_endpoints: 0,
        }
    }
}

#[derive(Debug)]
pub struct RemoteEndpoints {
    pub endpoints: Vec<RemoteEndpoint>,
}
impl RemoteEndpoints {
    pub fn new(ep: Vec<RemoteEndpoint>) -> Self {
        RemoteEndpoints { endpoints: ep }
    }

    pub fn get_core_list(&self) -> BTreeMap<u32, IpAddr> {
        let mut core_list: BTreeMap<u32, IpAddr> = BTreeMap::new();
        let mut counter: u32 = 0;

        for ep in &self.endpoints {
            let cores = ep.sys_info.cpu_virtual_cores.try_into().unwrap_or(0);
            for _ in 0..cores {
                core_list.insert(counter, ep.ip);
                counter += 1;
            }
        }

        core_list
    }

    pub fn thread_summary(&self) -> RemoteEndpointSummary {
        let mut summary: RemoteEndpointSummary = RemoteEndpointSummary::new();
        summary.total_cores = self
            .endpoints
            .iter()
            .map(|ep| ep.sys_info.cpu_virtual_cores.try_into().unwrap_or(0))
            .sum();
        summary.total_endpoints = self.endpoints.len() as u32;

        summary
    }
}
