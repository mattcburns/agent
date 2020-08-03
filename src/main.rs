use rustbus::{get_system_bus_path, standard_messages, Conn, MessageBuilder, message};

 fn main() -> Result<(), rustbus::client_conn::Error> {
     // Connect to the session bus
     let session_path = get_system_bus_path()?;
     let con = Conn::connect_to_bus(session_path, true)?;

     // Wrap the con in an RpcConnection which provides many convenient functions
     let mut rpc_con = rustbus::client_conn::RpcConn::new(con);



     // send the obligatory hello message
    rpc_con.send_message(&mut standard_messages::hello(), None)?;

    let serial = rpc_con.send_message(&mut get_serial(), None)?;
    let resp = rpc_con.wait_response(serial, None)?;

    
    println!("{:?}", resp);

    //  // Request a bus name if you want to
    //  rpc_con.send_message(&mut standard_messages::request_name(
    //      "io.killing.spark".into(),
    //      0,
    //  ), None)?;

    //  // send a signal to all bus members
    //  let mut sig = MessageBuilder::new()
    //  .signal(
    //      "io.killing.spark".into(),
    //      "TestSignal".into(),
    //      "/io/killing/spark".into(),
    //  )
    //  .with_params(vec![
    //      Container::Struct(vec![162254319i32.into(), "AABB".to_owned().into()]).into(),
    //  ])
    //  .build();
    //  rpc_con.send_message(&mut sig, None)?;
     Ok(())
 }

 pub fn get_serial<'a, 'e>() -> message::Message<'a, 'e> {
    MessageBuilder::new()
        .call("Get".into())
        .on("/xyz/openbmc_project/inventory/system".into())
        .with_interface("xyz.openbmc_project.Inventory.Manager".into())
        .at("xyz.openbmc_project.Inventory.Decorator.AssetTag.AssetTag".into())
        .build()
 }

 // cross build --target armv5te-unknown-linux-gnueabi --release