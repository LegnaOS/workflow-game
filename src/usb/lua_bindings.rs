//! USB Lua 绑定 - 将 rusb 功能暴露给 Lua
//!
//! 使用 enum 包装 Context 和 GlobalContext，统一处理泛型

#[allow(unused_imports)]
use crate::usb::types::*;
use mlua::{prelude::*, UserData, UserDataMethods};
use rusb::{Context, Device, DeviceHandle, DeviceList, GlobalContext, UsbContext};
use std::sync::{Arc, Mutex};
use std::time::Duration;

// ============ 类型包装：解决 Context vs GlobalContext 泛型问题 ============

/// 包装的设备句柄 - 支持两种上下文
enum AnyDeviceHandle {
    Global(DeviceHandle<GlobalContext>),
    Custom(DeviceHandle<Context>),
}

/// 包装的设备列表
enum AnyDeviceList {
    Global(DeviceList<GlobalContext>),
    Custom(DeviceList<Context>),
}

/// 包装的设备
enum AnyDevice {
    Global(Device<GlobalContext>),
    Custom(Device<Context>),
}

// 为 AnyDeviceHandle 实现所有需要的方法（委托模式）
macro_rules! delegate_handle {
    ($self:expr, $method:ident $(, $arg:expr)*) => {
        match $self {
            AnyDeviceHandle::Global(h) => h.$method($($arg),*),
            AnyDeviceHandle::Custom(h) => h.$method($($arg),*),
        }
    };
}

impl AnyDeviceHandle {
    fn device(&self) -> AnyDevice {
        match self {
            AnyDeviceHandle::Global(h) => AnyDevice::Global(h.device()),
            AnyDeviceHandle::Custom(h) => AnyDevice::Custom(h.device()),
        }
    }

    fn claim_interface(&self, iface: u8) -> rusb::Result<()> {
        delegate_handle!(self, claim_interface, iface)
    }

    fn release_interface(&self, iface: u8) -> rusb::Result<()> {
        delegate_handle!(self, release_interface, iface)
    }

    fn set_active_configuration(&self, config: u8) -> rusb::Result<()> {
        delegate_handle!(self, set_active_configuration, config)
    }

    fn active_configuration(&self) -> rusb::Result<u8> {
        delegate_handle!(self, active_configuration)
    }

    fn set_alternate_setting(&self, iface: u8, alt: u8) -> rusb::Result<()> {
        delegate_handle!(self, set_alternate_setting, iface, alt)
    }

    fn reset(&self) -> rusb::Result<()> {
        delegate_handle!(self, reset)
    }

    fn clear_halt(&self, endpoint: u8) -> rusb::Result<()> {
        delegate_handle!(self, clear_halt, endpoint)
    }

    fn read_bulk(&self, endpoint: u8, buf: &mut [u8], timeout: Duration) -> rusb::Result<usize> {
        delegate_handle!(self, read_bulk, endpoint, buf, timeout)
    }

    fn write_bulk(&self, endpoint: u8, buf: &[u8], timeout: Duration) -> rusb::Result<usize> {
        delegate_handle!(self, write_bulk, endpoint, buf, timeout)
    }

    fn read_interrupt(&self, endpoint: u8, buf: &mut [u8], timeout: Duration) -> rusb::Result<usize> {
        delegate_handle!(self, read_interrupt, endpoint, buf, timeout)
    }

    fn write_interrupt(&self, endpoint: u8, buf: &[u8], timeout: Duration) -> rusb::Result<usize> {
        delegate_handle!(self, write_interrupt, endpoint, buf, timeout)
    }

    fn read_control(&self, request_type: u8, request: u8, value: u16, index: u16, buf: &mut [u8], timeout: Duration) -> rusb::Result<usize> {
        delegate_handle!(self, read_control, request_type, request, value, index, buf, timeout)
    }

    fn write_control(&self, request_type: u8, request: u8, value: u16, index: u16, buf: &[u8], timeout: Duration) -> rusb::Result<usize> {
        delegate_handle!(self, write_control, request_type, request, value, index, buf, timeout)
    }

    fn kernel_driver_active(&self, iface: u8) -> rusb::Result<bool> {
        delegate_handle!(self, kernel_driver_active, iface)
    }

    fn detach_kernel_driver(&self, iface: u8) -> rusb::Result<()> {
        delegate_handle!(self, detach_kernel_driver, iface)
    }

    fn attach_kernel_driver(&self, iface: u8) -> rusb::Result<()> {
        delegate_handle!(self, attach_kernel_driver, iface)
    }

    fn set_auto_detach_kernel_driver(&self, auto: bool) -> rusb::Result<()> {
        delegate_handle!(self, set_auto_detach_kernel_driver, auto)
    }

    fn read_languages(&self, timeout: Duration) -> rusb::Result<Vec<rusb::Language>> {
        delegate_handle!(self, read_languages, timeout)
    }

    fn read_string_descriptor_ascii(&self, index: u8) -> rusb::Result<String> {
        delegate_handle!(self, read_string_descriptor_ascii, index)
    }

    fn read_manufacturer_string_ascii(&self, desc: &rusb::DeviceDescriptor) -> rusb::Result<String> {
        match desc.manufacturer_string_index() {
            Some(idx) => self.read_string_descriptor_ascii(idx),
            None => Err(rusb::Error::InvalidParam),
        }
    }

    fn read_product_string_ascii(&self, desc: &rusb::DeviceDescriptor) -> rusb::Result<String> {
        match desc.product_string_index() {
            Some(idx) => self.read_string_descriptor_ascii(idx),
            None => Err(rusb::Error::InvalidParam),
        }
    }

    fn read_serial_number_string_ascii(&self, desc: &rusb::DeviceDescriptor) -> rusb::Result<String> {
        match desc.serial_number_string_index() {
            Some(idx) => self.read_string_descriptor_ascii(idx),
            None => Err(rusb::Error::InvalidParam),
        }
    }
}

impl AnyDevice {
    fn device_descriptor(&self) -> rusb::Result<rusb::DeviceDescriptor> {
        match self {
            AnyDevice::Global(d) => d.device_descriptor(),
            AnyDevice::Custom(d) => d.device_descriptor(),
        }
    }

    fn config_descriptor(&self, index: u8) -> rusb::Result<rusb::ConfigDescriptor> {
        match self {
            AnyDevice::Global(d) => d.config_descriptor(index),
            AnyDevice::Custom(d) => d.config_descriptor(index),
        }
    }

    fn active_config_descriptor(&self) -> rusb::Result<rusb::ConfigDescriptor> {
        match self {
            AnyDevice::Global(d) => d.active_config_descriptor(),
            AnyDevice::Custom(d) => d.active_config_descriptor(),
        }
    }

    fn bus_number(&self) -> u8 {
        match self {
            AnyDevice::Global(d) => d.bus_number(),
            AnyDevice::Custom(d) => d.bus_number(),
        }
    }

    fn address(&self) -> u8 {
        match self {
            AnyDevice::Global(d) => d.address(),
            AnyDevice::Custom(d) => d.address(),
        }
    }

    fn speed(&self) -> rusb::Speed {
        match self {
            AnyDevice::Global(d) => d.speed(),
            AnyDevice::Custom(d) => d.speed(),
        }
    }

    fn open(&self) -> rusb::Result<AnyDeviceHandle> {
        match self {
            AnyDevice::Global(d) => d.open().map(AnyDeviceHandle::Global),
            AnyDevice::Custom(d) => d.open().map(AnyDeviceHandle::Custom),
        }
    }
}

// ============ USB 上下文 ============

/// USB 上下文 - Lua UserData
pub struct LuaUsbContext {
    ctx: Option<Context>,
}

impl LuaUsbContext {
    pub fn new() -> LuaResult<Self> {
        let ctx = Context::new().map_err(|e| mlua::Error::external(format!("USB init failed: {}", e)))?;
        Ok(Self { ctx: Some(ctx) })
    }

    pub fn global() -> Self {
        Self { ctx: None }
    }

    fn devices(&self) -> rusb::Result<AnyDeviceList> {
        if let Some(ref ctx) = self.ctx {
            ctx.devices().map(AnyDeviceList::Custom)
        } else {
            rusb::devices().map(AnyDeviceList::Global)
        }
    }
}

impl UserData for LuaUsbContext {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        // 列出所有设备
        methods.add_method("devices", |lua, this, ()| {
            let devices = this.devices().map_err(|e| mlua::Error::external(e.to_string()))?;
            build_device_list_table(lua, &devices)
        });

        // 通过 VID/PID 打开设备
        methods.add_method("open", |_, this, (vid, pid): (u16, u16)| {
            let devices = this.devices().map_err(|e| mlua::Error::external(e.to_string()))?;
            let handle = find_and_open_device(&devices, |desc| {
                desc.vendor_id() == vid && desc.product_id() == pid
            })?;
            Ok(LuaUsbDevice::new(handle))
        });

        // 通过 bus/address 打开设备
        methods.add_method("open_by_address", |_, this, (bus, addr): (u8, u8)| {
            let devices = this.devices().map_err(|e| mlua::Error::external(e.to_string()))?;
            let handle = find_and_open_device_by_address(&devices, bus, addr)?;
            Ok(LuaUsbDevice::new(handle))
        });
    }
}

// ============ 辅助函数 ============

fn build_device_list_table(lua: &Lua, devices: &AnyDeviceList) -> LuaResult<LuaTable> {
    let result = lua.create_table()?;

    let iter: Box<dyn Iterator<Item = AnyDevice>> = match devices {
        AnyDeviceList::Global(list) => Box::new(list.iter().map(AnyDevice::Global)),
        AnyDeviceList::Custom(list) => Box::new(list.iter().map(AnyDevice::Custom)),
    };

    for (i, device) in iter.enumerate() {
        let info = lua.create_table()?;
        info.set("bus_number", device.bus_number())?;
        info.set("address", device.address())?;

        if let Ok(desc) = device.device_descriptor() {
            info.set("vendor_id", desc.vendor_id())?;
            info.set("product_id", desc.product_id())?;
            info.set("class_code", desc.class_code())?;
            info.set("subclass_code", desc.sub_class_code())?;
            info.set("protocol_code", desc.protocol_code())?;
            info.set("num_configurations", desc.num_configurations())?;

            if let Ok(handle) = device.open() {
                if let Ok(langs) = handle.read_languages(Duration::from_millis(100)) {
                    if let Some(lang) = langs.first() {
                        read_string_descriptors(lua, &info, &handle, &desc, *lang)?;
                    }
                }
            }
        }

        let speed = match device.speed() {
            rusb::Speed::Low => "low",
            rusb::Speed::Full => "full",
            rusb::Speed::High => "high",
            rusb::Speed::Super => "super",
            rusb::Speed::SuperPlus => "super_plus",
            _ => "unknown",
        };
        info.set("speed", speed)?;
        result.set(i + 1, info)?;
    }
    Ok(result)
}

fn read_string_descriptors(
    _lua: &Lua,
    info: &LuaTable,
    handle: &AnyDeviceHandle,
    desc: &rusb::DeviceDescriptor,
    lang: rusb::Language,
) -> LuaResult<()> {
    // 使用 read_string_descriptor_ascii 简化，避免 language 复杂性
    if let Some(idx) = desc.manufacturer_string_index() {
        if let Ok(s) = handle.read_string_descriptor_ascii(idx) {
            info.set("manufacturer", s)?;
        }
    }
    if let Some(idx) = desc.product_string_index() {
        if let Ok(s) = handle.read_string_descriptor_ascii(idx) {
            info.set("product", s)?;
        }
    }
    if let Some(idx) = desc.serial_number_string_index() {
        if let Ok(s) = handle.read_string_descriptor_ascii(idx) {
            info.set("serial_number", s)?;
        }
    }
    let _ = lang; // 保留参数以备将来使用
    Ok(())
}

fn find_and_open_device<F>(devices: &AnyDeviceList, predicate: F) -> LuaResult<AnyDeviceHandle>
where
    F: Fn(&rusb::DeviceDescriptor) -> bool,
{
    let iter: Box<dyn Iterator<Item = AnyDevice>> = match devices {
        AnyDeviceList::Global(list) => Box::new(list.iter().map(AnyDevice::Global)),
        AnyDeviceList::Custom(list) => Box::new(list.iter().map(AnyDevice::Custom)),
    };

    for device in iter {
        if let Ok(desc) = device.device_descriptor() {
            if predicate(&desc) {
                return device.open().map_err(|e| mlua::Error::external(e.to_string()));
            }
        }
    }
    Err(mlua::Error::external("Device not found"))
}

fn find_and_open_device_by_address(devices: &AnyDeviceList, bus: u8, addr: u8) -> LuaResult<AnyDeviceHandle> {
    let iter: Box<dyn Iterator<Item = AnyDevice>> = match devices {
        AnyDeviceList::Global(list) => Box::new(list.iter().map(AnyDevice::Global)),
        AnyDeviceList::Custom(list) => Box::new(list.iter().map(AnyDevice::Custom)),
    };

    for device in iter {
        if device.bus_number() == bus && device.address() == addr {
            return device.open().map_err(|e| mlua::Error::external(e.to_string()));
        }
    }
    Err(mlua::Error::external("Device not found"))
}

// ============ USB 设备句柄 ============

/// USB 设备句柄 - Lua UserData
pub struct LuaUsbDevice {
    handle: Arc<Mutex<AnyDeviceHandle>>,
    claimed_interfaces: Arc<Mutex<Vec<u8>>>,
}

impl Clone for LuaUsbDevice {
    fn clone(&self) -> Self {
        Self {
            handle: Arc::clone(&self.handle),
            claimed_interfaces: Arc::clone(&self.claimed_interfaces),
        }
    }
}

impl LuaUsbDevice {
    pub fn new(handle: AnyDeviceHandle) -> Self {
        Self {
            handle: Arc::new(Mutex::new(handle)),
            claimed_interfaces: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl UserData for LuaUsbDevice {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
    // 获取设备描述符
    methods.add_method("descriptor", |lua, this, ()| {
        let handle = this.handle.lock().unwrap();
        let device = handle.device();
        let desc = device.device_descriptor().map_err(|e| mlua::Error::external(e.to_string()))?;

        let info = lua.create_table()?;
        info.set("vendor_id", desc.vendor_id())?;
        info.set("product_id", desc.product_id())?;
        info.set("device_version", format!("{}.{}.{}",
            desc.device_version().major(),
            desc.device_version().minor(),
            desc.device_version().sub_minor()))?;
        info.set("class_code", desc.class_code())?;
        info.set("subclass_code", desc.sub_class_code())?;
        info.set("protocol_code", desc.protocol_code())?;
        info.set("max_packet_size", desc.max_packet_size())?;
        info.set("num_configurations", desc.num_configurations())?;
        info.set("usb_version", format!("{}.{}.{}",
            desc.usb_version().major(),
            desc.usb_version().minor(),
            desc.usb_version().sub_minor()))?;
        Ok(info)
    });

    // 获取配置描述符
    methods.add_method("config", |lua, this, config_index: Option<u8>| {
        let handle = this.handle.lock().unwrap();
        let device = handle.device();

        let config = if let Some(idx) = config_index {
            device.config_descriptor(idx)
        } else {
            device.active_config_descriptor()
        }.map_err(|e| mlua::Error::external(e.to_string()))?;

        let info = lua.create_table()?;
        info.set("number", config.number())?;
        info.set("num_interfaces", config.num_interfaces())?;
        info.set("max_power_ma", config.max_power() as u16 * 2)?;
        info.set("self_powered", config.self_powered())?;
        info.set("remote_wakeup", config.remote_wakeup())?;

        // 接口列表
        let interfaces = lua.create_table()?;
        for (i, iface) in config.interfaces().enumerate() {
            let iface_info = lua.create_table()?;
            iface_info.set("number", iface.number())?;

            let alt_settings = lua.create_table()?;
            for (j, alt) in iface.descriptors().enumerate() {
                let alt_info = lua.create_table()?;
                alt_info.set("number", alt.setting_number())?;
                alt_info.set("class_code", alt.class_code())?;
                alt_info.set("subclass_code", alt.sub_class_code())?;
                alt_info.set("protocol_code", alt.protocol_code())?;
                alt_info.set("num_endpoints", alt.num_endpoints())?;

                // 端点列表
                let endpoints = lua.create_table()?;
                for (k, ep) in alt.endpoint_descriptors().enumerate() {
                    let ep_info = lua.create_table()?;
                    ep_info.set("address", ep.address())?;
                    ep_info.set("number", ep.number())?;
                    ep_info.set("direction", match ep.direction() {
                        rusb::Direction::In => "in",
                        rusb::Direction::Out => "out",
                    })?;
                    ep_info.set("transfer_type", match ep.transfer_type() {
                        rusb::TransferType::Control => "control",
                        rusb::TransferType::Isochronous => "isochronous",
                        rusb::TransferType::Bulk => "bulk",
                        rusb::TransferType::Interrupt => "interrupt",
                    })?;
                    ep_info.set("max_packet_size", ep.max_packet_size())?;
                    ep_info.set("interval", ep.interval())?;
                    endpoints.set(k + 1, ep_info)?;
                }
                alt_info.set("endpoints", endpoints)?;
                alt_settings.set(j + 1, alt_info)?;
            }
            iface_info.set("alt_settings", alt_settings)?;
            interfaces.set(i + 1, iface_info)?;
        }
        info.set("interfaces", interfaces)?;
        Ok(info)
    });

    // 声明接口
    methods.add_method("claim_interface", |_, this, iface: u8| {
        let handle = this.handle.lock().unwrap();
        handle.claim_interface(iface).map_err(|e| mlua::Error::external(e.to_string()))?;
        this.claimed_interfaces.lock().unwrap().push(iface);
        Ok(())
    });

    // 释放接口
    methods.add_method("release_interface", |_, this, iface: u8| {
        let handle = this.handle.lock().unwrap();
        handle.release_interface(iface).map_err(|e| mlua::Error::external(e.to_string()))?;
        this.claimed_interfaces.lock().unwrap().retain(|&x| x != iface);
        Ok(())
    });

    // 设置备用设置
    methods.add_method("set_alternate_setting", |_, this, (iface, setting): (u8, u8)| {
        let handle = this.handle.lock().unwrap();
        handle.set_alternate_setting(iface, setting).map_err(|e| mlua::Error::external(e.to_string()))?;
        Ok(())
    });

    // 获取活动配置
    methods.add_method("active_configuration", |_, this, ()| {
        let handle = this.handle.lock().unwrap();
        let config = handle.active_configuration().map_err(|e| mlua::Error::external(e.to_string()))?;
        Ok(config)
    });

    // 设置活动配置
    methods.add_method("set_configuration", |_, this, config: u8| {
        let handle = this.handle.lock().unwrap();
        handle.set_active_configuration(config).map_err(|e| mlua::Error::external(e.to_string()))?;
        Ok(())
    });

    // 重置设备
    methods.add_method("reset", |_, this, ()| {
        let handle = this.handle.lock().unwrap();
        handle.reset().map_err(|e| mlua::Error::external(e.to_string()))?;
        Ok(())
    });

    // 清除端点停止状态
    methods.add_method("clear_halt", |_, this, endpoint: u8| {
        let handle = this.handle.lock().unwrap();
        handle.clear_halt(endpoint).map_err(|e| mlua::Error::external(e.to_string()))?;
        Ok(())
    });

    // ============ 数据传输方法 ============

    // Bulk 读取
    methods.add_method("read_bulk", |lua, this, (endpoint, size, timeout_ms): (u8, usize, Option<u64>)| {
        let handle = this.handle.lock().unwrap();
        let timeout = Duration::from_millis(timeout_ms.unwrap_or(1000));
        let mut buf = vec![0u8; size];

        let n = handle.read_bulk(endpoint, &mut buf, timeout)
            .map_err(|e| mlua::Error::external(e.to_string()))?;

        buf.truncate(n);
        let result = lua.create_table()?;
        result.set("data", lua.create_string(&buf)?)?;
        result.set("length", n)?;
        Ok(result)
    });

    // Bulk 写入
    methods.add_method("write_bulk", |_, this, (endpoint, data, timeout_ms): (u8, mlua::String, Option<u64>)| {
        let handle = this.handle.lock().unwrap();
        let timeout = Duration::from_millis(timeout_ms.unwrap_or(1000));
        let buf: Vec<u8> = data.as_bytes().to_vec();

        let n = handle.write_bulk(endpoint, &buf, timeout)
            .map_err(|e| mlua::Error::external(e.to_string()))?;
        Ok(n)
    });

    // Interrupt 读取
    methods.add_method("read_interrupt", |lua, this, (endpoint, size, timeout_ms): (u8, usize, Option<u64>)| {
        let handle = this.handle.lock().unwrap();
        let timeout = Duration::from_millis(timeout_ms.unwrap_or(1000));
        let mut buf = vec![0u8; size];

        let n = handle.read_interrupt(endpoint, &mut buf, timeout)
            .map_err(|e| mlua::Error::external(e.to_string()))?;

        buf.truncate(n);
        let result = lua.create_table()?;
        result.set("data", lua.create_string(&buf)?)?;
        result.set("length", n)?;
        Ok(result)
    });

    // Interrupt 写入
    methods.add_method("write_interrupt", |_, this, (endpoint, data, timeout_ms): (u8, mlua::String, Option<u64>)| {
        let handle = this.handle.lock().unwrap();
        let timeout = Duration::from_millis(timeout_ms.unwrap_or(1000));
        let buf: Vec<u8> = data.as_bytes().to_vec();

        let n = handle.write_interrupt(endpoint, &buf, timeout)
            .map_err(|e| mlua::Error::external(e.to_string()))?;
        Ok(n)
    });

    // Control 读取
    methods.add_method("read_control", |lua, this, args: mlua::Table| {
        let handle = this.handle.lock().unwrap();

        let request_type: u8 = args.get("request_type")?;
        let request: u8 = args.get("request")?;
        let value: u16 = args.get("value")?;
        let index: u16 = args.get("index")?;
        let size: usize = args.get("size").unwrap_or(64);
        let timeout_ms: u64 = args.get("timeout").unwrap_or(1000);

        let timeout = Duration::from_millis(timeout_ms);
        let mut buf = vec![0u8; size];

        let n = handle.read_control(request_type, request, value, index, &mut buf, timeout)
            .map_err(|e| mlua::Error::external(e.to_string()))?;

        buf.truncate(n);
        let result = lua.create_table()?;
        result.set("data", lua.create_string(&buf)?)?;
        result.set("length", n)?;
        Ok(result)
    });

    // Control 写入
    methods.add_method("write_control", |_, this, args: mlua::Table| {
        let handle = this.handle.lock().unwrap();

        let request_type: u8 = args.get("request_type")?;
        let request: u8 = args.get("request")?;
        let value: u16 = args.get("value")?;
        let index: u16 = args.get("index")?;
        let data: mlua::String = args.get("data")?;
        let timeout_ms: u64 = args.get("timeout").unwrap_or(1000);

        let timeout = Duration::from_millis(timeout_ms);
        let buf: Vec<u8> = data.as_bytes().to_vec();

        let n = handle.write_control(request_type, request, value, index, &buf, timeout)
            .map_err(|e| mlua::Error::external(e.to_string()))?;
        Ok(n)
    });

    // ============ 内核驱动方法 ============

    // 检查内核驱动是否激活
    methods.add_method("kernel_driver_active", |_, this, iface: u8| {
        let handle = this.handle.lock().unwrap();
        let active = handle.kernel_driver_active(iface)
            .map_err(|e| mlua::Error::external(e.to_string()))?;
        Ok(active)
    });

    // 分离内核驱动
    methods.add_method("detach_kernel_driver", |_, this, iface: u8| {
        let handle = this.handle.lock().unwrap();
        handle.detach_kernel_driver(iface).map_err(|e| mlua::Error::external(e.to_string()))?;
        Ok(())
    });

    // 附加内核驱动
    methods.add_method("attach_kernel_driver", |_, this, iface: u8| {
        let handle = this.handle.lock().unwrap();
        handle.attach_kernel_driver(iface).map_err(|e| mlua::Error::external(e.to_string()))?;
        Ok(())
    });

    // 设置自动分离内核驱动
    methods.add_method("set_auto_detach_kernel_driver", |_, this, auto_detach: bool| {
        let handle = this.handle.lock().unwrap();
        handle.set_auto_detach_kernel_driver(auto_detach).map_err(|e| mlua::Error::external(e.to_string()))?;
        Ok(())
    });

    // ============ 字符串描述符 ============

    // 读取语言列表
    methods.add_method("languages", |lua, this, timeout_ms: Option<u64>| {
        let handle = this.handle.lock().unwrap();
        let timeout = Duration::from_millis(timeout_ms.unwrap_or(1000));

        let langs = handle.read_languages(timeout)
            .map_err(|e| mlua::Error::external(e.to_string()))?;

        let result = lua.create_table()?;
        for (i, lang) in langs.iter().enumerate() {
            let info = lua.create_table()?;
            // Language 是 u16，直接使用 lang_id()
            info.set("lang_id", lang.lang_id())?;
            result.set(i + 1, info)?;
        }
        Ok(result)
    });

    // 读取字符串描述符（ASCII）
    methods.add_method("read_string_ascii", |_, this, index: u8| {
        let handle = this.handle.lock().unwrap();
        let s = handle.read_string_descriptor_ascii(index)
            .map_err(|e| mlua::Error::external(e.to_string()))?;
        Ok(s)
    });

    // 读取制造商字符串
    methods.add_method("manufacturer", |_, this, ()| {
        let handle = this.handle.lock().unwrap();
        let device = handle.device();
        let desc = device.device_descriptor().map_err(|e| mlua::Error::external(e.to_string()))?;
        let s = handle.read_manufacturer_string_ascii(&desc)
            .map_err(|e| mlua::Error::external(e.to_string()))?;
        Ok(s)
    });

    // 读取产品字符串
    methods.add_method("product", |_, this, ()| {
        let handle = this.handle.lock().unwrap();
        let device = handle.device();
        let desc = device.device_descriptor().map_err(|e| mlua::Error::external(e.to_string()))?;
        let s = handle.read_product_string_ascii(&desc)
            .map_err(|e| mlua::Error::external(e.to_string()))?;
        Ok(s)
    });

    // 读取序列号字符串
    methods.add_method("serial_number", |_, this, ()| {
        let handle = this.handle.lock().unwrap();
        let device = handle.device();
        let desc = device.device_descriptor().map_err(|e| mlua::Error::external(e.to_string()))?;
        let s = handle.read_serial_number_string_ascii(&desc)
            .map_err(|e| mlua::Error::external(e.to_string()))?;
        Ok(s)
    });

    // 关闭设备（释放所有接口）
    methods.add_method("close", |_, this, ()| {
        let handle = this.handle.lock().unwrap();
        let interfaces = this.claimed_interfaces.lock().unwrap().clone();
        for iface in interfaces {
            let _ = handle.release_interface(iface);
        }
        this.claimed_interfaces.lock().unwrap().clear();
        Ok(())
    });
    }
}

// ============ 全局函数和模块注册 ============

/// 注册 USB 模块到 Lua
pub fn register_usb_module(lua: &Lua) -> LuaResult<()> {
    let usb = lua.create_table()?;

    // usb.version() - 获取 libusb 版本
    usb.set("version", lua.create_function(|lua, ()| {
        let ver = rusb::version();
        let info = lua.create_table()?;
        info.set("major", ver.major())?;
        info.set("minor", ver.minor())?;
        info.set("micro", ver.micro())?;
        info.set("nano", ver.nano())?;
        if let Some(rc) = ver.rc() {
            info.set("rc", rc)?;
        }
        Ok(info)
    })?)?;

    // usb.has_capability(cap) - 检查功能支持
    usb.set("has_hotplug", lua.create_function(|_, ()| {
        Ok(rusb::has_hotplug())
    })?)?;

    usb.set("has_hid_access", lua.create_function(|_, ()| {
        Ok(rusb::has_hid_access())
    })?)?;

    usb.set("supports_detach_kernel_driver", lua.create_function(|_, ()| {
        Ok(rusb::supports_detach_kernel_driver())
    })?)?;

    // usb.devices() - 列出所有设备（使用全局上下文）
    usb.set("devices", lua.create_function(|lua, ()| {
        let devices = rusb::devices().map_err(|e| mlua::Error::external(e.to_string()))?;

        let result = lua.create_table()?;
        for (i, device) in devices.iter().enumerate() {
            let info = lua.create_table()?;
            info.set("bus_number", device.bus_number())?;
            info.set("address", device.address())?;

            if let Ok(desc) = device.device_descriptor() {
                info.set("vendor_id", desc.vendor_id())?;
                info.set("product_id", desc.product_id())?;
                info.set("class_code", desc.class_code())?;
                info.set("subclass_code", desc.sub_class_code())?;
                info.set("protocol_code", desc.protocol_code())?;
                info.set("num_configurations", desc.num_configurations())?;

                if let Ok(handle) = device.open() {
                    if let Ok(langs) = handle.read_languages(Duration::from_millis(100)) {
                        if let Some(lang) = langs.first() {
                            if desc.manufacturer_string_index().is_some() {
                                if let Ok(s) = handle.read_manufacturer_string(*lang, &desc, Duration::from_millis(100)) {
                                    info.set("manufacturer", s)?;
                                }
                            }
                            if desc.product_string_index().is_some() {
                                if let Ok(s) = handle.read_product_string(*lang, &desc, Duration::from_millis(100)) {
                                    info.set("product", s)?;
                                }
                            }
                            if desc.serial_number_string_index().is_some() {
                                if let Ok(s) = handle.read_serial_number_string(*lang, &desc, Duration::from_millis(100)) {
                                    info.set("serial_number", s)?;
                                }
                            }
                        }
                    }
                }
            }

            let speed = match device.speed() {
                rusb::Speed::Low => "low",
                rusb::Speed::Full => "full",
                rusb::Speed::High => "high",
                rusb::Speed::Super => "super",
                rusb::Speed::SuperPlus => "super_plus",
                _ => "unknown",
            };
            info.set("speed", speed)?;

            result.set(i + 1, info)?;
        }
        Ok(result)
    })?)?;

    // usb.open(vid, pid) - 快捷打开设备
    usb.set("open", lua.create_function(|_, (vid, pid): (u16, u16)| {
        let handle = rusb::open_device_with_vid_pid(vid, pid)
            .ok_or_else(|| mlua::Error::external("Device not found"))?;
        Ok(LuaUsbDevice::new(AnyDeviceHandle::Global(handle)))
    })?)?;

    // usb.open_by_address(bus, address) - 通过地址打开
    usb.set("open_by_address", lua.create_function(|_, (bus, addr): (u8, u8)| {
        let devices = rusb::devices().map_err(|e| mlua::Error::external(e.to_string()))?;
        for device in devices.iter() {
            if device.bus_number() == bus && device.address() == addr {
                let handle = device.open().map_err(|e| mlua::Error::external(e.to_string()))?;
                return Ok(LuaUsbDevice::new(AnyDeviceHandle::Global(handle)));
            }
        }
        Err(mlua::Error::external("Device not found"))
    })?)?;

    // usb.context() - 创建独立上下文
    usb.set("context", lua.create_function(|_, ()| {
        LuaUsbContext::new()
    })?)?;

    // usb.request_type(direction, type, recipient) - 构建请求类型字节
    usb.set("request_type", lua.create_function(|_, (direction, req_type, recipient): (String, String, String)| {
        let dir: u8 = match direction.as_str() {
            "in" | "IN" => 0x80,
            "out" | "OUT" => 0x00,
            _ => return Err(mlua::Error::external("direction must be 'in' or 'out'")),
        };
        let typ: u8 = match req_type.as_str() {
            "standard" | "STANDARD" => 0x00,
            "class" | "CLASS" => 0x20,
            "vendor" | "VENDOR" => 0x40,
            _ => return Err(mlua::Error::external("type must be 'standard', 'class', or 'vendor'")),
        };
        let rec: u8 = match recipient.as_str() {
            "device" | "DEVICE" => 0x00,
            "interface" | "INTERFACE" => 0x01,
            "endpoint" | "ENDPOINT" => 0x02,
            "other" | "OTHER" => 0x03,
            _ => return Err(mlua::Error::external("recipient must be 'device', 'interface', 'endpoint', or 'other'")),
        };
        Ok(dir | typ | rec)
    })?)?;

    // 常量
    let constants = lua.create_table()?;

    // 传输类型
    constants.set("TRANSFER_TYPE_CONTROL", 0)?;
    constants.set("TRANSFER_TYPE_ISOCHRONOUS", 1)?;
    constants.set("TRANSFER_TYPE_BULK", 2)?;
    constants.set("TRANSFER_TYPE_INTERRUPT", 3)?;

    // 端点方向
    constants.set("ENDPOINT_IN", 0x80)?;
    constants.set("ENDPOINT_OUT", 0x00)?;

    // 请求类型
    constants.set("REQUEST_TYPE_STANDARD", 0x00)?;
    constants.set("REQUEST_TYPE_CLASS", 0x20)?;
    constants.set("REQUEST_TYPE_VENDOR", 0x40)?;

    // 请求接收者
    constants.set("RECIPIENT_DEVICE", 0x00)?;
    constants.set("RECIPIENT_INTERFACE", 0x01)?;
    constants.set("RECIPIENT_ENDPOINT", 0x02)?;
    constants.set("RECIPIENT_OTHER", 0x03)?;

    // 标准请求
    constants.set("REQUEST_GET_STATUS", 0x00)?;
    constants.set("REQUEST_CLEAR_FEATURE", 0x01)?;
    constants.set("REQUEST_SET_FEATURE", 0x03)?;
    constants.set("REQUEST_SET_ADDRESS", 0x05)?;
    constants.set("REQUEST_GET_DESCRIPTOR", 0x06)?;
    constants.set("REQUEST_SET_DESCRIPTOR", 0x07)?;
    constants.set("REQUEST_GET_CONFIGURATION", 0x08)?;
    constants.set("REQUEST_SET_CONFIGURATION", 0x09)?;
    constants.set("REQUEST_GET_INTERFACE", 0x0a)?;
    constants.set("REQUEST_SET_INTERFACE", 0x0b)?;
    constants.set("REQUEST_SYNCH_FRAME", 0x0c)?;

    // 描述符类型
    constants.set("DESCRIPTOR_TYPE_DEVICE", 0x01)?;
    constants.set("DESCRIPTOR_TYPE_CONFIG", 0x02)?;
    constants.set("DESCRIPTOR_TYPE_STRING", 0x03)?;
    constants.set("DESCRIPTOR_TYPE_INTERFACE", 0x04)?;
    constants.set("DESCRIPTOR_TYPE_ENDPOINT", 0x05)?;

    // 设备类
    constants.set("CLASS_PER_INTERFACE", 0x00)?;
    constants.set("CLASS_AUDIO", 0x01)?;
    constants.set("CLASS_COMM", 0x02)?;
    constants.set("CLASS_HID", 0x03)?;
    constants.set("CLASS_PHYSICAL", 0x05)?;
    constants.set("CLASS_IMAGE", 0x06)?;
    constants.set("CLASS_PRINTER", 0x07)?;
    constants.set("CLASS_MASS_STORAGE", 0x08)?;
    constants.set("CLASS_HUB", 0x09)?;
    constants.set("CLASS_DATA", 0x0a)?;
    constants.set("CLASS_VENDOR_SPEC", 0xff)?;

    usb.set("const", constants)?;

    // 注册到全局
    lua.globals().set("usb", usb)?;

    Ok(())
}

