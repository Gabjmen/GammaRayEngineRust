use wgpu;

pub fn getBackendForSpecificOS() -> wgpu::Backends {
    let backend = if cfg!(target_os = "windows") {
        wgpu::Backends::DX12
    } else if cfg!(target_os = "macos") {
        wgpu::Backends::METAL
    } else if cfg!(target_os = "linux") {
        wgpu::Backends::VULKAN
    } else if cfg!(target_os = "android") {
        wgpu::Backends::VULKAN
    } else if cfg!(target_os = "ios") {
        wgpu::Backends::METAL
    } else {
        wgpu::Backends::VULKAN
    };

    backend
}
