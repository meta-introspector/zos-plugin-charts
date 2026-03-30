mod plugin_trait;
use plugin_trait::*;
use serde_json::json;
use std::os::raw::c_char;

#[no_mangle] pub extern "C" fn zos_plugin_name() -> *mut c_char { to_c("charts") }
#[no_mangle] pub extern "C" fn zos_plugin_version() -> *mut c_char { to_c("0.2.0") }
#[no_mangle] pub extern "C" fn zos_plugin_commands() -> *mut c_char { to_c("bar,line,pie,zkperf-history") }

#[no_mangle] pub extern "C" fn zos_plugin_execute(cmd: *const c_char, arg: *const c_char) -> *mut c_char {
    let cmd = unsafe { std::ffi::CStr::from_ptr(cmd) }.to_str().unwrap_or("");
    let arg = unsafe { std::ffi::CStr::from_ptr(arg) }.to_str().unwrap_or("");
    let result = json!({"plugin": "charts", "command": cmd, "arg": arg, "status": "ok"});
    let shard = DA51Shard::from_result("charts", cmd, &result);
    to_c(&serde_json::to_string(&json!({"result": result, "shard": shard})).unwrap())
}

#[no_mangle] pub extern "C" fn zos_plugin_render() -> *mut c_char {
    let gui = vec![
        GuiComponent::Heading { level: 2, text: "📊 Charts".into() },
        GuiComponent::Button { label: "Bar".into(), command: "bar".into() },
    ];
    to_c(&serde_json::to_string(&gui).unwrap())
}

#[no_mangle] pub extern "C" fn zos_plugin_init() -> i32 { 0 }
