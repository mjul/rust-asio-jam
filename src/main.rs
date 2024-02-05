extern crate anyhow;
extern crate cpal;

use cpal::traits::{DeviceTrait, HostTrait};

mod cpal_enumerate {
    pub(crate) fn cpal_enumerate_example() -> Result<(), anyhow::Error> {
        println!("Supported hosts:\n  {:?}", cpal::ALL_HOSTS);
        let available_hosts = cpal::available_hosts();
        println!("Available hosts:\n  {:?}", available_hosts);

        for host_id in available_hosts {
            println!("{}", host_id.name());
            let host = cpal::host_from_id(host_id)?;

            let default_in = host.default_input_device().map(|e| e.name().unwrap());
            let default_out = host.default_output_device().map(|e| e.name().unwrap());
            println!("  Default Input Device:\n    {:?}", default_in);
            println!("  Default Output Device:\n    {:?}", default_out);

            let devices = host.devices()?;
            println!("  Devices: ");
            for (device_index, device) in devices.enumerate() {
                println!("  {}. \"{}\"", device_index + 1, device.name()?);

                // Input configs
                if let Ok(conf) = device.default_input_config() {
                    println!("    Default input stream config:\n      {:?}", conf);
                }
                let input_configs = match device.supported_input_configs() {
                    Ok(f) => f.collect(),
                    Err(e) => {
                        println!("    Error getting supported input configs: {:?}", e);
                        Vec::new()
                    }
                };
                if !input_configs.is_empty() {
                    println!("    All supported input stream configs:");
                    for (config_index, config) in input_configs.into_iter().enumerate() {
                        println!(
                            "      {}.{}. {:?}",
                            device_index + 1,
                            config_index + 1,
                            config
                        );
                    }
                }

                // Output configs
                if let Ok(conf) = device.default_output_config() {
                    println!("    Default output stream config:\n      {:?}", conf);
                }
                let output_configs = match device.supported_output_configs() {
                    Ok(f) => f.collect(),
                    Err(e) => {
                        println!("    Error getting supported output configs: {:?}", e);
                        Vec::new()
                    }
                };
                if !output_configs.is_empty() {
                    println!("    All supported output stream configs:");
                    for (config_index, config) in output_configs.into_iter().enumerate() {
                        println!(
                            "      {}.{}. {:?}",
                            device_index + 1,
                            config_index + 1,
                            config
                        );
                    }
                }
            }
        }

        Ok(())
    }
}

mod asio_sys_enumerate {
    extern crate asio_sys as sys;

    // This is the same data that enumerate
// is trying to find
// Basically these are stubbed versions
//
// Format that each sample has.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SampleFormat {
        // The value 0 corresponds to 0.
        I16,
        // The value 0 corresponds to 32768.
        U16,
        // The boundaries are (-1.0, 1.0).
        F32,
    }

    // Number of channels.
    pub type ChannelCount = u16;

    // The number of samples processed per second for a single channel of audio.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct SampleRate(pub u32);

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Format {
        pub channels: crate::ChannelCount,
        pub sample_rate: crate::SampleRate,
        pub data_type: crate::SampleFormat,
    }

    pub(crate) fn asio_sys_enumerate_example() {
        let asio = sys::Asio::new();
        for name in asio.driver_names() {
            println!("Driver: {:?}", name);
            let driver = asio.load_driver(&name).expect("failed to load driver");
            let channels = driver
                .channels()
                .expect("failed to retrieve channel counts");
            let sample_rate = driver
                .sample_rate()
                .expect("failed to retrieve sample rate");
            let in_fmt = crate::Format {
                channels: channels.ins as _,
                sample_rate: crate::SampleRate(sample_rate as _),
                data_type: crate::SampleFormat::F32,
            };
            let out_fmt = crate::Format {
                channels: channels.outs as _,
                sample_rate: crate::SampleRate(sample_rate as _),
                data_type: crate::SampleFormat::F32,
            };
            println!("  Input {:?}", in_fmt);
            println!("  Output {:?}", out_fmt);
        }
    }
}


fn main() {
    println!("Running example from CPAL:");
    cpal_enumerate::cpal_enumerate_example().unwrap();
    println!("Running example from ASIO_SYS:");
    asio_sys_enumerate::asio_sys_enumerate_example()
}
