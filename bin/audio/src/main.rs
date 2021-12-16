use clap::{crate_authors, crate_version, AppSettings, Arg, SubCommand};
use sfml::{
    audio::{capture, SoundRecorderDriver, SoundStatus, SoundStreamPlayer},
    system::Time,
};
use std::{
    net::{TcpListener, TcpStream, UdpSocket},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    time::Duration,
};

use crate::{player::AudioStream, recorder::AudioRecorder};

mod io;
mod player;
mod recorder;

const SAMPLE_RATE: u32 = 48000;
const CHANNEL_COUNT: u32 = 2;

const BUFFER_TIME_MS: u32 = 10;

const BUFFER_FRAME_SIZE: usize = (SAMPLE_RATE * CHANNEL_COUNT * BUFFER_TIME_MS / 1000) as usize;

static __DEBUG_ENABLE: AtomicBool = AtomicBool::new(false);

#[allow(dead_code)]
fn debug() -> bool {
    __DEBUG_ENABLE.load(Ordering::Relaxed)
}

fn set_debug(enable: bool) {
    __DEBUG_ENABLE.store(enable, Ordering::Relaxed)
}

fn main() {
    let app = &mut clap::App::new("Audio Forwarding")
        .version(crate_version!())
        .author(crate_authors!())
        .setting(AppSettings::SubcommandsNegateReqs)
        .subcommand(
            SubCommand::with_name("recorder")
                .arg(Arg::with_name("debug").short("d").long("debug"))
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("player")
                        .short("r")
                        .long("player")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("tcp")
                        .short("t")
                        .long("use-tcp")
                        .alias("tcp"),
                ),
        )
        .subcommand(
            SubCommand::with_name("player")
                .arg(Arg::with_name("debug").short("d").long("debug"))
                .arg(
                    Arg::with_name("port")
                        .short("p")
                        .long("port")
                        .takes_value(true)
                        .default_value("0"),
                )
                .arg(
                    Arg::with_name("tcp")
                        .short("t")
                        .long("use-tcp")
                        .alias("tcp"),
                ),
        )
        .subcommand(SubCommand::with_name("devices"));

    let matches = app.clone().get_matches();

    match matches.subcommand() {
        ("recorder", Some(matches)) => {
            if !capture::is_available() {
                panic!("Sorry, audio capture is not supported by your system");
            }

            set_debug(matches.is_present("debug"));

            let devices = capture::available_devices();
            let device_idx = match matches.value_of("input") {
                Some(str) => str.parse::<usize>().expect("invalid input"),
                None => 0,
            };

            let addr = matches.value_of("player").unwrap();
            let use_tcp = matches.is_present("tcp");

            if use_tcp {
                let socket = TcpStream::connect(addr).expect("Failed to connect to player");

                let recorder = &mut AudioRecorder::new(socket);
                let driver = &mut SoundRecorderDriver::new(recorder);

                driver
                    .set_device(&devices[device_idx])
                    .expect("Failed to set device");

                driver.set_processing_interval(Time::milliseconds(1));
                driver.set_channel_count(CHANNEL_COUNT);
                driver.start(SAMPLE_RATE);

                println!("Connected to player");
                std::thread::park();
            } else {
                let socket = UdpSocket::bind(("0.0.0.0", 0)).expect("Failed to init udp socket");
                socket.connect(addr).expect("Failed to connect to player");

                let recorder = &mut AudioRecorder::new(io::UdpWriter::new(socket));
                let driver = &mut SoundRecorderDriver::new(recorder);

                driver
                    .set_device(&devices[device_idx])
                    .expect("Failed to set device");

                driver.set_processing_interval(Time::milliseconds(1));
                driver.set_channel_count(CHANNEL_COUNT);
                driver.start(SAMPLE_RATE);

                println!("Connected to player");
                std::thread::park();
            }
        }
        ("player", matches) => {
            set_debug(matches!(matches.map(|m| m.is_present("debug")), Some(true)));

            let port = match matches {
                Some(matches) => match matches.value_of("port") {
                    Some(port) => port.parse::<u16>().expect("invalid port"),
                    None => 0,
                },
                None => 0,
            };

            let use_tcp: bool = match matches {
                Some(matches) => matches.is_present("tcp"),
                None => false,
            };

            if use_tcp {
                let listener = &mut TcpListener::bind(("0.0.0.0", port))
                    .expect(&format!("Failed to listen on: 0.0.0.0:{}", port));

                println!(
                    "Waiting for recorder at: {}",
                    listener.local_addr().unwrap()
                );

                let (sender, receiver) = mpsc::channel::<TcpStream>();

                std::thread::spawn(move || {
                    for socket in receiver {
                        let stream = &mut AudioStream::new(socket);
                        let mut player = SoundStreamPlayer::new(stream);
                        player.play();

                        while player.status() == SoundStatus::PLAYING {
                            std::thread::sleep(Duration::from_millis(100));
                        }
                    }
                });

                loop {
                    let (socket, addr) = listener.accept().expect("Failed to accept recorder");
                    println!("Recorder found at: {}", addr);

                    sender.send(socket).expect("mpsc failed to send socket");
                }
            } else {
                let socket = UdpSocket::bind(("0.0.0.0", port))
                    .expect(&format!("Failed to bind on: 0.0.0.0:{}", port));

                println!("Waiting for recorder at: {}", socket.local_addr().unwrap());

                let stream = &mut AudioStream::new(io::UdpReader::new(socket));
                let mut player = SoundStreamPlayer::new(stream);
                player.play();

                while player.status() == SoundStatus::PLAYING {
                    std::thread::sleep(Duration::from_millis(100));
                }
            }
        }
        ("devices", _) => {
            let devices = capture::available_devices();

            println!("\n\nAvailable Input Devices:");
            for (idx, device) in devices.iter().enumerate() {
                println!("\t({}) {}", idx, device);
            }
        }
        _ => {
            app.print_long_help().unwrap();
            let devices = capture::available_devices();

            println!("\n\nAvailable Input Devices:");
            for (idx, device) in devices.iter().enumerate() {
                println!("\t({}) {}", idx, device);
            }
        }
    }
}
