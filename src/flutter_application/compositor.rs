use ash::vk::Handle;
use std::{cell::Cell, ffi::c_void, mem::size_of, ptr::null_mut};

use wgpu::{
    CommandEncoderDescriptor, Extent3d, Origin3d, TexelCopyTextureInfo, Texture, TextureAspect,
    TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use wgpu_hal::api::Vulkan;

use crate::{
    flutter_application::FlutterApplication,
    flutter_bindings::{
        FlutterBackingStore, FlutterBackingStoreConfig,
        FlutterBackingStoreType_kFlutterBackingStoreTypeVulkan, FlutterBackingStore__bindgen_ty_1,
        FlutterCompositor, FlutterLayer,
        FlutterLayerContentType_kFlutterLayerContentTypeBackingStore,
        FlutterLayerContentType_kFlutterLayerContentTypePlatformView, FlutterVulkanBackingStore,
        FlutterVulkanImage,
    },
};

use super::FlutterApplicationUserData;

pub struct Compositor {
    platform_view_count: Cell<i64>,
}

impl Compositor {
    pub fn new() -> Self {
        Self {
            platform_view_count: Cell::new(0),
        }
    }

    pub fn flutter_compositor(&self, application: &FlutterApplication) -> FlutterCompositor {
        FlutterCompositor {
            struct_size: size_of::<FlutterCompositor>() as _,
            user_data: &*application.user_data as *const FlutterApplicationUserData as _,
            create_backing_store_callback: Some(Self::create_backing_store_callback),
            collect_backing_store_callback: Some(Self::backing_store_collect_callback),
            present_layers_callback: Some(Self::present_layers_callback),
            avoid_backing_store_cache: false,
            present_view_callback: None,
        }
    }

    extern "C" fn create_backing_store_callback(
        config: *const FlutterBackingStoreConfig,
        backing_store_out: *mut FlutterBackingStore,
        user_data: *mut c_void,
    ) -> bool {
        let application_user_data = unsafe {
            &*(user_data as *const FlutterApplicationUserData) as &FlutterApplicationUserData
        };

        let texture = application_user_data
            .device
            .create_texture(&TextureDescriptor {
                label: Some("Flutter Backing Store"),
                size: wgpu::Extent3d {
                    width: unsafe { *config }.size.width as _,
                    height: unsafe { *config }.size.height as _,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Bgra8Unorm,
                usage: TextureUsages::COPY_SRC
                    | TextureUsages::RENDER_ATTACHMENT
                    | TextureUsages::TEXTURE_BINDING,
                view_formats: &[TextureFormat::Bgra8Unorm],
            });

        let mut image = None;
        unsafe {
            texture.as_hal::<Vulkan, _, _>(|texture| {
                let texture = texture.unwrap();

                image = Some(FlutterVulkanImage {
                    struct_size: size_of::<FlutterVulkanImage>() as _,
                    image: texture.raw_handle().as_raw() as _,
                    format: ash::vk::Format::B8G8R8A8_UNORM.as_raw() as _,
                });
            });
        }
        let image = image.unwrap();
        let user_data = Box::new((texture, image));
        let backing_store = unsafe { &mut *backing_store_out as &mut FlutterBackingStore };
        backing_store.user_data = null_mut();
        backing_store.type_ = FlutterBackingStoreType_kFlutterBackingStoreTypeVulkan;
        backing_store.did_update = true;
        backing_store.__bindgen_anon_1 = FlutterBackingStore__bindgen_ty_1 {
            vulkan: FlutterVulkanBackingStore {
                struct_size: size_of::<FlutterVulkanBackingStore>() as _,
                image: &user_data.1,
                user_data: Box::into_raw(user_data) as _,
                destruction_callback: Some(Self::destroy_texture),
            },
        };
        true
    }
    extern "C" fn destroy_texture(user_data: *mut c_void) {
        let (texture, _image) =
            *unsafe { Box::from_raw(user_data as *mut (Texture, FlutterVulkanImage)) };
        texture.destroy();
    }
    extern "C" fn present_layers_callback(
        layers: *mut *const FlutterLayer,
        layers_count: usize,
        user_data: *mut c_void,
    ) -> bool {
        let application_user_data = unsafe { &*(user_data as *const FlutterApplicationUserData) };

        let frame = application_user_data
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let mut encoder = application_user_data
            .device
            .create_command_encoder(&CommandEncoderDescriptor { label: None });
        {
            // encoder.clear_texture(&frame.texture, &ImageSubresourceRange::default());
            // encoder.begin_render_pass(&RenderPassDescriptor {
            //     label: None,
            //     color_attachments: &[Some(RenderPassColorAttachment {
            //         view: &view,
            //         resolve_target: None,
            //         ops: Operations {
            //             load: LoadOp::Clear(Color {
            //                 r: 0.0,
            //                 g: 1.0,
            //                 b: 0.0,
            //                 a: 1.0,
            //             }),
            //             store: true,
            //         },
            //     })],
            //     depth_stencil_attachment: None,
            // });

            for (idx, &layer) in unsafe { std::slice::from_raw_parts(layers, layers_count as _) }
                .iter()
                .map(|&layer| unsafe { &*layer } as &FlutterLayer)
                .enumerate()
            {
                let offset = layer.offset;
                let size = layer.size;
                log::debug!("Layer {idx} type {}", layer.type_);
                match layer.type_ {
                    x if x == FlutterLayerContentType_kFlutterLayerContentTypeBackingStore => {
                        let backing_store = unsafe { &*layer.__bindgen_anon_1.backing_store };
                        assert_eq!(
                            backing_store.type_,
                            FlutterBackingStoreType_kFlutterBackingStoreTypeVulkan
                        );
                        let backing_store = unsafe { &backing_store.__bindgen_anon_1.vulkan };
                        let texture = unsafe { &*(backing_store.user_data as *mut Texture) };

                        encoder.copy_texture_to_texture(
                            TexelCopyTextureInfo {
                                texture,
                                mip_level: 0,
                                origin: Origin3d::ZERO,
                                aspect: TextureAspect::All,
                            },
                            TexelCopyTextureInfo {
                                texture: &frame.texture,
                                mip_level: 0,
                                origin: Origin3d {
                                    x: offset.x as _,
                                    y: offset.y as _,
                                    z: 0,
                                },
                                aspect: TextureAspect::All,
                            },
                            Extent3d {
                                width: size.width as _,
                                height: size.height as _,
                                depth_or_array_layers: 1,
                            },
                        );
                    }
                    x if x == FlutterLayerContentType_kFlutterLayerContentTypePlatformView => {
                        log::error!(
                            "Rendering platform view {}: not implemented yet!",
                            unsafe { &*layer.__bindgen_anon_1.platform_view }.identifier
                        );
                    }
                    _ => panic!("Invalid layer type"),
                }
            }
        }
        application_user_data.queue.submit(Some(encoder.finish()));
        frame.present();
        true
    }
    extern "C" fn backing_store_collect_callback(
        _renderer: *const FlutterBackingStore,
        _user_data: *mut c_void,
    ) -> bool {
        // let _this = user_data as *const FlutterApplication;
        // destroy the user_data in FlutterBackingStore. Since we passed nullptr there, there's nothing to do
        true
    }
}
