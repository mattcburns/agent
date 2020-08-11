use rustbus::{RpcConn, MessageBuilder, client_conn::Timeout, params::Container};
use rustbus::params::{Param, Base};

fn main() -> Result<(), rustbus::client_conn::Error> {
    let mut rpc_con = RpcConn::system_conn(Timeout::Infinite)?;

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
    let serial = rpc_con.send_message(&mut sig, Timeout::Infinite)?;
    let resp = rpc_con.wait_response(serial, Timeout::Infinite)?;

    let r = resp.unmarshall_all()?;

    let s = &r.params[0];

    // This is super ugly, need to figure out how to improve
    if let Param::Container(v) = s {
        if let Container::Variant(x) = v {
            if let Param::Base(y) = &x.value {
                if let Base::String(z) = &y {
                    println!("{:?}", z)
                }
            }
        }
    }

    Ok(())
}
