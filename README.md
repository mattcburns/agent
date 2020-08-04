Build with: 
    
    cross build --target armv5te-unknown-linux-gnueabi --release

Set the initial s/n on openbmc emulator:

    busctl set-property xyz.openbmc_project.Inventory.Manager /xyz/openbmc_project/inventory/system xyz.openbmc_project.Inventory.Decorator.AssetTag AssetTag s "12345"

Get the s/n on openbmc emulator

    busctl get-property xyz.openbmc_project.Inventory.Manager /xyz/openbmc_project/inventory/system xyz.openbmc_project.Inventory.Decorator.AssetTag AssetTag