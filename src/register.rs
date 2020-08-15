use rustbus::{RpcConn, MessageBuilder, client_conn::Timeout, params::Container};
use rustbus::params::{Param, Base};

use crate::errors::{RegistrationError, SerialError};

pub fn register_bmc() -> Result<(), RegistrationError> {
    let mut conn = match openconn() {
        Ok(conn) => conn,
        Err(_) => return Err(RegistrationError::new("Unable to open connection.")),
    };

    let serial = match get_serial_number(&mut conn) {
        Ok(serial) => serial,
        Err(_) => return Err(RegistrationError::new("Unable to read serial number.")),
    };
    println!("{:?}", serial);

    Ok(())

}

fn openconn() -> Result<RpcConn, rustbus::client_conn::Error> {
    match RpcConn::system_conn(Timeout::Infinite) {
        Ok(rpc_conn) => return Ok(rpc_conn),
        Err(e) => return Err(e),
    };
}

fn get_serial_number(rpc_conn: &mut RpcConn) -> Result<String, SerialError> {


    let mut sig = MessageBuilder::new()
        .call("Get".into())
        .on("/xyz/openbmc_project/inventory/system".into())
        .with_interface("org.freedesktop.DBus.Properties".into())
        .at("xyz.openbmc_project.Inventory.Manager".into())
        .build();

    // Additional params to retrieve the AssetTag properties.
    match sig.body.push_param2(
        String::from("xyz.openbmc_project.Inventory.Decorator.AssetTag").as_str(),
        String::from("AssetTag").as_str()) {
            Ok(()) => {},
            Err(_) => return Err(SerialError::new("Unable to push params.")),
    };

    let serial = match rpc_conn.send_message(&mut sig, Timeout::Infinite){
        Ok(serial) => serial,
        Err(_) => return Err(SerialError::new("Unable to send message to bus.")),
    };
    let resp = match rpc_conn.wait_response(serial, Timeout::Infinite){
        Ok(resp) => resp,
        Err(_) => return Err(SerialError::new("Never got a response from bus.")),
    };

    let message = match resp.unmarshall_all(){
        Ok(message) => message,
        Err(_) => return Err(SerialError::new("Unable to unmarshall message")),
    };

    let first_param = &message.params[0];


    // This is super ugly, need to figure out how to improve
    if let Param::Container(v) = first_param {
        if let Container::Variant(x) = v {
            if let Param::Base(y) = &x.value {
                if let Base::String(z) = &y {
                    return Ok(z.to_string());
                }
            }
        }
    }

    Err(SerialError::new("Unable to parse serial"))
}