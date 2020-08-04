use rustbus::{RpcConn, MessageBuilder, client_conn::Timeout, wire};

fn main() -> Result<(), rustbus::client_conn::Error> {
    // Connect to the session bus
    let mut rpc_con = RpcConn::system_conn(Timeout::Infinite)?;

    // create a signal with the MessageBuilder API
    let mut sig = MessageBuilder::new()
    .call("Get".into())
    .on("/xyz/openbmc_project/inventory/system".into())
    .with_interface("org.freedesktop.DBus.Properties".into())
    .at("xyz.openbmc_project.Inventory.Manager".into())
    .build();

    sig.body.push_param2(String::from("xyz.openbmc_project.Inventory.Decorator.AssetTag").as_str(),
        String::from("AssetTag").as_str()).expect("Unable to push_param2");
    // send a signal to all bus members
    let serial = rpc_con.send_message(&mut sig, Timeout::Infinite)?;
    let resp = rpc_con.wait_response(serial, Timeout::Infinite)?;
    
    println!("{:?}", resp.unmarshall_all()?);
    

    Ok(())
}
