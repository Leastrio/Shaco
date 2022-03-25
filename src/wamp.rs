// use sysinfo::{System, SystemExt};

// use crate::WAMPClient;
// use crate::utils::process_info::*;
// use wamp_async::{self, Client, ClientConfig, SerializerType};

// type Error = Box<dyn std::error::Error>;

// impl WAMPClient<'_> {
//     pub async fn connect() -> Result<(), Error> {
//         let mut sys = System::new_all();
//         sys.refresh_all();
//         let process = find_process(&sys)?;
//         let args = extract_info(process)?;

//         let uri = format!("wss://riot:{}@127.0.0.1:{}", args.0, args.1);
//         println!("{}", &uri);

//         let (mut client, (event_loop, rpc_event_queue)) = Client::connect(uri,
//             Some(
//                 ClientConfig::default()
//                     .set_ssl_verify(false)
//                     .set_serializers(vec![SerializerType::Json])
//             )
//         ).await?;

//         Ok(())
//     }
// }
