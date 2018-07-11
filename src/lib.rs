pub extern crate digitalocean;
extern crate reqwest;
extern crate igd;

use igd::{ Gateway };
use digitalocean::prelude::*;
use std::net::Ipv4Addr;
use std::fmt::Display;

// TODO: this struct really isn't needed
pub struct DynDns {
    client: DigitalOcean,
    gateway: Gateway,
}

// TODO: lol this error handling

impl DynDns {
    pub fn new(token: impl Into<String> + Display) -> Result<Self, String> {
        Ok(Self {
            client: DigitalOcean::new(token)
                .map_err(|e| e.cause().to_string())?,
            gateway: igd::search_gateway()
                .map_err(|e| e.to_string())?,
        })
    }

    pub fn set_domain(&mut self, domain: impl AsRef<str> + Display) -> Result<Ipv4Addr, String> {
        let ip = self.gateway.get_external_ip()
            .map_err(|e| e.to_string())?;

        let ipstr = ip.to_string();

        let records = Domain::get(domain).records();

        for record in self.client.execute(records.clone()).map_err(|e| e.to_string())? {
            if record.name() == "@" && record.kind() == "A" && record.data() != &ipstr {
                self.client.execute(records.clone().delete(*record.id()))
                    .map_err(|e| e.to_string())?;
            }
        }

        self.client.execute(records.create("A", "@", &ipstr))
            .map_err(|e| e.to_string())?;

        return Ok(ip);
    }
}
