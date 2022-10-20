use crate::errors::GDError;
use crate::valve::{ValveProtocol, App, GatheringSettings, Response};

pub struct CSGO;

impl CSGO {
    pub fn query(address: &str, port: Option<u16>) -> Result<Response, GDError> {
        ValveProtocol::query(App::CSGO, address, match port {
            None => 27015,
            Some(port) => port
        }, GatheringSettings {
            players: true,
            rules: true
        })
    }
}