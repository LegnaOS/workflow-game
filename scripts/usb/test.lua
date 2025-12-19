-- USB 功能测试脚本
-- 用于验证 USB 模块的基本功能

-- 测试 USB 版本信息
print("=== USB Version ===")
local ver = usb.version()
print(string.format("libusb version: %d.%d.%d.%d", ver.major, ver.minor, ver.micro, ver.nano))

-- 测试功能支持
print("\n=== USB Capabilities ===")
print("Has hotplug: " .. tostring(usb.has_hotplug()))
print("Has HID access: " .. tostring(usb.has_hid_access()))
print("Supports detach kernel driver: " .. tostring(usb.supports_detach_kernel_driver()))

-- 列出所有设备
print("\n=== USB Devices ===")
local devices = usb.devices()
print("Found " .. #devices .. " USB device(s)")

for i, dev in ipairs(devices) do
    print(string.format("\n[%d] Bus %d, Address %d", i, dev.bus_number, dev.address))
    print(string.format("    VID:PID = %04x:%04x", dev.vendor_id or 0, dev.product_id or 0))
    print(string.format("    Class: %d, Subclass: %d, Protocol: %d", 
        dev.class_code or 0, dev.subclass_code or 0, dev.protocol_code or 0))
    print("    Speed: " .. (dev.speed or "unknown"))
    
    if dev.manufacturer then
        print("    Manufacturer: " .. dev.manufacturer)
    end
    if dev.product then
        print("    Product: " .. dev.product)
    end
    if dev.serial_number then
        print("    Serial: " .. dev.serial_number)
    end
end

-- 测试常量
print("\n=== USB Constants ===")
print("ENDPOINT_IN: " .. string.format("0x%02x", usb.const.ENDPOINT_IN))
print("ENDPOINT_OUT: " .. string.format("0x%02x", usb.const.ENDPOINT_OUT))
print("CLASS_HID: " .. string.format("0x%02x", usb.const.CLASS_HID))
print("CLASS_MASS_STORAGE: " .. string.format("0x%02x", usb.const.CLASS_MASS_STORAGE))

-- 测试 request_type 构建
print("\n=== Request Type Builder ===")
local rt = usb.request_type("in", "vendor", "device")
print("IN + VENDOR + DEVICE = " .. string.format("0x%02x", rt))

print("\n=== USB Test Complete ===")

