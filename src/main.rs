use std::error::Error;

use rustbus::{RpcConn, MessageBuilder, client_conn::Timeout, params::Container};
use rustbus::params::{Param, Base};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Serial: {:?}", get_serial_number()?);
    Ok(())
}

fn get_serial_number() -> Result<String, Box<dyn Error>> {
    let mut rpc_con = match RpcConn::system_conn(Timeout::Infinite) {
        Ok(rpc_con) => rpc_con,
        Err(e) => panic!(e),
    };


    let mut sig = MessageBuilder::new()
        .call("Get".into())
        .on("/xyz/openbmc_project/inventory/system".into())
        .with_interface("org.freedesktop.DBus.Properties".into())
        .at("xyz.openbmc_project.Inventory.Manager".into())
        .build();

    // Additional params to retrieve the AssetTag properties.
    sig.body.push_param2(
        String::from("xyz.openbmc_project.Inventory.Decorator.AssetTag").as_str(),
        String::from("AssetTag").as_str())
        .expect("Unable to push_param2");
    let serial = match rpc_con.send_message(&mut sig, Timeout::Infinite){
        Ok(serial) => serial,
        Err(e) => panic!(e),
    };
    let resp = match rpc_con.wait_response(serial, Timeout::Infinite){
        Ok(resp) => resp,
        Err(e) => panic!(e),
    };

    let r = match resp.unmarshall_all(){
        Ok(r) => r,
        Err(e) => panic!(e),
    };

    let s = &r.params[0];


    // This is super ugly, need to figure out how to improve
    if let Param::Container(v) = s {
        if let Container::Variant(x) = v {
            if let Param::Base(y) = &x.value {
                if let Base::String(z) = &y {
                    return Ok(z.to_string());
                }
            }
        }
    }

    panic!("Unable to parse serial");
}