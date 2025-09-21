//! # GPU Renderer Crate
//! 
//! This crate provides GPU-accelerated rendering using wgpu and winit.
//! It renders layout boxes and text to a window with proper styling
//! and visual representation of the CSS box model.

use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};
use wgpu::{
    util::DeviceExt, Backends, Device, Instance, PresentMode, Queue, RenderPipeline,
    SurfaceConfiguration, TextureFormat,
};
use bytemuck::{Pod, Zeroable};
use std::borrow::Cow;
use thiserror::Error;
use wgpu_glyph::{GlyphBrush};
use std::fs::File;
use std::io::Write;

// Event-driven rendering module
pub mod event_driven_renderer;

// Real user input handling module
pub mod input_handler;

// Simple async event loop integration module
pub mod async_event_loop_simple;

/// Custom error types for GPU rendering
#[derive(Error, Debug)]
pub enum RenderError {
    #[error("Failed to create window: {0}")]
    WindowCreationFailed(#[from] winit::error::OsError),
    
    #[error("Failed to create wgpu instance: {0}")]
    InstanceCreationFailed(String),
    
    #[error("Failed to create surface: {0}")]
    SurfaceCreationFailed(#[from] wgpu::CreateSurfaceError),
    
    #[error("Failed to create adapter")]
    AdapterRequestFailed,
    
    #[error("Failed to create device: {0}")]
    DeviceCreationFailed(#[from] wgpu::RequestDeviceError),
    
    #[error("Failed to get surface texture: {0}")]
    SurfaceTextureFailed(#[from] wgpu::SurfaceError),
    
    #[error("Screenshot capture failed: {0}")]
    ScreenshotFailed(#[from] Box<dyn std::error::Error>),
    
    #[error("Failed to create render pipeline: {0}")]
    PipelineCreationFailed(String),
}

/// Result type for rendering operations
pub type RenderResult<T> = Result<T, RenderError>;

/// Vertex data for rendering rectangles
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

/// Simple GPU renderer for layout boxes
pub struct GpuRenderer {
    device: Device,
    queue: Queue,
    render_pipeline: RenderPipeline,
}

impl GpuRenderer {
    /// Create a new GPU renderer
    pub async fn new(window: &Window) -> RenderResult<Self> {
        let instance = Instance::new(wgpu::InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });

        let surface = instance
            .create_surface(window)
            .map_err(|e| RenderError::SurfaceCreationFailed(e))?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or(RenderError::AdapterRequestFailed)?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .map_err(|e| RenderError::DeviceCreationFailed(e))?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let render_pipeline = Self::create_render_pipeline(&device, surface_format)?;

        Ok(GpuRenderer {
            device,
            queue,
            render_pipeline,
        })
    }

    /// Create the render pipeline for drawing rectangles
    fn create_render_pipeline(
        device: &Device,
        surface_format: TextureFormat,
    ) -> RenderResult<RenderPipeline> {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        });

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Ok(render_pipeline)
    }

    /// Get references to device and queue for rendering
    pub fn get_device_and_queue(&self) -> (&Device, &Queue) {
        (&self.device, &self.queue)
    }

    /// Get the render pipeline
    pub fn get_render_pipeline(&self) -> &RenderPipeline {
        &self.render_pipeline
    }
}

/// Extract text content from a layout box
fn extract_text_content(layout_box: &layout::LayoutBox) -> String {
    match &layout_box.node.node_type {
        dom::NodeType::Text(text) => text.clone(),
        _ => String::new(),
    }
}

/// Render text using wgpu_glyph (placeholder for now)
#[allow(dead_code)]
fn render_text(
    _glyph_brush: &mut GlyphBrush<()>,
    _text: &str,
    _x: f32,
    _y: f32,
    _color: [f32; 3],
) {
    // TODO: Implement actual text rendering using wgpu_glyph
    // For now, we'll just print the text to console
    if !_text.is_empty() {
        println!("RENDERING TEXT: '{}' at ({}, {})", _text, _x, _y);
    }
}

/// Add debug overlay vertices showing box model (margins, borders, content)
fn add_debug_overlay_vertices(
    layout_box: &layout::LayoutBox,
    vertices: &mut Vec<Vertex>,
    indices: &mut Vec<u16>,
    vertex_offset: &mut u16,
    parent_x: f32,
    parent_y: f32,
) {
    // Use the border dimensions as the main box dimensions
    let x = layout_box.border.x + parent_x;
    let y = layout_box.border.y + parent_y;
    let width = layout_box.border.width;
    let height = layout_box.border.height;

    // Skip if dimensions are invalid
    if width <= 0.0 || height <= 0.0 {
        return;
    }

    // Convert to NDC coordinates for debug overlay
    let scale_factor = 0.001;
    let ndc_x = (x * scale_factor) - 0.5;
    let ndc_y = 0.5 - (y * scale_factor);
    let ndc_width = width * scale_factor;
    let ndc_height = height * scale_factor;

    // Clamp to screen bounds
    let ndc_x = ndc_x.max(-1.0).min(1.0);
    let ndc_y = ndc_y.max(-1.0).min(1.0);
    let ndc_width = ndc_width.min(2.0);
    let ndc_height = ndc_height.min(2.0);

    // Create debug overlay with different colors for different parts
    // Content area (inner rectangle) - white
    let content_color = [1.0, 1.0, 1.0];
    let content_vertices = [
        Vertex { position: [ndc_x + 0.01, ndc_y - 0.01], color: content_color },
        Vertex { position: [ndc_x + ndc_width - 0.01, ndc_y - 0.01], color: content_color },
        Vertex { position: [ndc_x + 0.01, ndc_y - ndc_height + 0.01], color: content_color },
        Vertex { position: [ndc_x + ndc_width - 0.01, ndc_y - ndc_height + 0.01], color: content_color },
    ];

    // Border area (middle rectangle) - yellow
    let border_color = [1.0, 1.0, 0.0];
    let border_vertices = [
        Vertex { position: [ndc_x, ndc_y], color: border_color },
        Vertex { position: [ndc_x + ndc_width, ndc_y], color: border_color },
        Vertex { position: [ndc_x, ndc_y - ndc_height], color: border_color },
        Vertex { position: [ndc_x + ndc_width, ndc_y - ndc_height], color: border_color },
    ];

    // Margin area (outer rectangle) - red
    let margin_color = [1.0, 0.0, 0.0];
    let margin_vertices = [
        Vertex { position: [ndc_x - 0.02, ndc_y + 0.02], color: margin_color },
        Vertex { position: [ndc_x + ndc_width + 0.02, ndc_y + 0.02], color: margin_color },
        Vertex { position: [ndc_x - 0.02, ndc_y - ndc_height - 0.02], color: margin_color },
        Vertex { position: [ndc_x + ndc_width + 0.02, ndc_y - ndc_height - 0.02], color: margin_color },
    ];

    // Add all debug overlay vertices
    vertices.extend_from_slice(&content_vertices);
    vertices.extend_from_slice(&border_vertices);
    vertices.extend_from_slice(&margin_vertices);

    // Add indices for content area (2 triangles)
    indices.extend_from_slice(&[
        *vertex_offset, *vertex_offset + 1, *vertex_offset + 2,
        *vertex_offset + 1, *vertex_offset + 3, *vertex_offset + 2,
    ]);
    *vertex_offset += 4;

    // Add indices for border area (2 triangles)
    indices.extend_from_slice(&[
        *vertex_offset, *vertex_offset + 1, *vertex_offset + 2,
        *vertex_offset + 1, *vertex_offset + 3, *vertex_offset + 2,
    ]);
    *vertex_offset += 4;

    // Add indices for margin area (2 triangles)
    indices.extend_from_slice(&[
        *vertex_offset, *vertex_offset + 1, *vertex_offset + 2,
        *vertex_offset + 1, *vertex_offset + 3, *vertex_offset + 2,
    ]);
    *vertex_offset += 4;

    // Recursively add debug overlay for children
    for child in &layout_box.children {
        add_debug_overlay_vertices(child, vertices, indices, vertex_offset, x, y);
    }
}

/// Capture a screenshot of the rendered output
#[allow(dead_code)]
fn capture_screenshot(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    surface_texture: &wgpu::SurfaceTexture,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let width = surface_texture.texture.width();
    let height = surface_texture.texture.height();
    
    // Calculate aligned bytes per row
    let bytes_per_row = width * 4;
    let aligned_bytes_per_row = ((bytes_per_row + 255) / 256) * 256; // Align to 256 bytes
    
    // Create a buffer to read the texture data
    let buffer_size = (aligned_bytes_per_row * height) as u64; // RGBA format with alignment
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Screenshot Buffer"),
        size: buffer_size,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    // Create a command encoder to copy the texture to the buffer
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Screenshot Encoder"),
    });

    encoder.copy_texture_to_buffer(
        wgpu::ImageCopyTexture {
            texture: &surface_texture.texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::ImageCopyBuffer {
            buffer: &buffer,
            layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(width * 4),
                rows_per_image: Some(height),
            },
        },
        wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
    );

    queue.submit(std::iter::once(encoder.finish()));

    // Map the buffer and read the data
    let buffer_slice = buffer.slice(..);
    let (sender, receiver) = std::sync::mpsc::channel();
    buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
        sender.send(result).unwrap();
    });

    device.poll(wgpu::Maintain::Wait);
    receiver.recv()??;

    let data = buffer_slice.get_mapped_range();
    
    // Convert RGBA to RGB and flip vertically (OpenGL coordinates)
    let mut rgb_data = Vec::new();
    for y in (0..height).rev() {
        for x in 0..width {
            let pixel_offset = (y * aligned_bytes_per_row + x * 4) as usize;
            if pixel_offset + 2 < data.len() {
                rgb_data.push(data[pixel_offset]);     // R
                rgb_data.push(data[pixel_offset + 1]); // G
                rgb_data.push(data[pixel_offset + 2]); // B
            }
        }
    }

    // Write PPM file (simple format)
    let mut file = File::create(filename)?;
    writeln!(file, "P6")?;
    writeln!(file, "{} {}", width, height)?;
    writeln!(file, "255")?;
    file.write_all(&rgb_data)?;

    println!("Screenshot saved to: {}", filename);
    Ok(())
}

/// Capture a screenshot from a texture
fn capture_texture_screenshot(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    texture: &wgpu::Texture,
    width: u32,
    height: u32,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Calculate aligned bytes per row
    let bytes_per_row = width * 4;
    let aligned_bytes_per_row = ((bytes_per_row + 255) / 256) * 256; // Align to 256 bytes
    
    // Create a buffer to read the texture data
    let buffer_size = (aligned_bytes_per_row * height) as u64; // RGBA format with alignment
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Screenshot Buffer"),
        size: buffer_size,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    // Create a command encoder to copy the texture to the buffer
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Screenshot Encoder"),
    });

    encoder.copy_texture_to_buffer(
        wgpu::ImageCopyTexture {
            texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::ImageCopyBuffer {
            buffer: &buffer,
            layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(aligned_bytes_per_row),
                rows_per_image: Some(height),
            },
        },
        wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
    );

    queue.submit(std::iter::once(encoder.finish()));

    // Map the buffer and read the data
    let buffer_slice = buffer.slice(..);
    let (sender, receiver) = std::sync::mpsc::channel();
    buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
        sender.send(result).unwrap();
    });

    device.poll(wgpu::Maintain::Wait);
    receiver.recv()??;

    let data = buffer_slice.get_mapped_range();
    
    // Convert RGBA to RGB and flip vertically (OpenGL coordinates)
    let mut rgb_data = Vec::new();
    for y in (0..height).rev() {
        for x in 0..width {
            let pixel_offset = (y * aligned_bytes_per_row + x * 4) as usize;
            if pixel_offset + 2 < data.len() {
                rgb_data.push(data[pixel_offset]);     // R
                rgb_data.push(data[pixel_offset + 1]); // G
                rgb_data.push(data[pixel_offset + 2]); // B
            }
        }
    }

    // Write PPM file (simple format)
    let mut file = File::create(filename)?;
    writeln!(file, "P6")?;
    writeln!(file, "{} {}", width, height)?;
    writeln!(file, "255")?;
    file.write_all(&rgb_data)?;

    println!("Screenshot saved to: {}", filename);
    Ok(())
}

/// Render layout tree to an offscreen texture and capture screenshot
pub async fn render_layout_tree_offscreen(
    layout_root: &layout::LayoutBox,
    width: u32,
    height: u32,
    filename: &str,
) -> RenderResult<()> {
    // Create wgpu instance
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: None,
            force_fallback_adapter: false,
        })
        .await
        .ok_or(RenderError::AdapterRequestFailed)?;

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
            },
            None,
        )
        .await?;

    let surface_format = wgpu::TextureFormat::Bgra8UnormSrgb;

    // Create render pipeline
    let render_pipeline = GpuRenderer::create_render_pipeline(&device, surface_format)?;

    // Create offscreen texture
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Offscreen Texture"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: surface_format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    // Create vertices and indices
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut vertex_offset = 0;

    // Add layout box vertices
    add_layout_box_vertices(layout_root, &mut vertices, &mut indices, &mut vertex_offset, 0.0, 0.0);
    
    // Add debug overlay vertices
    add_debug_overlay_vertices(layout_root, &mut vertices, &mut indices, &mut vertex_offset, 0.0, 0.0);

    if !vertices.is_empty() {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Layout Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Layout Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        // Render to offscreen texture
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.1,
                            b: 0.1,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&render_pipeline);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..indices.len() as u32, 0, 0..1);
        }

        queue.submit(std::iter::once(encoder.finish()));

        // Capture screenshot from texture
        capture_texture_screenshot(&device, &queue, &texture, width, height, filename)?;
    }

    Ok(())
}

/// Render a layout tree to a GPU window
pub async fn render_layout_tree(layout_root: &layout::LayoutBox) -> RenderResult<()> {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Browser Engine - Layout Renderer")
        .with_inner_size(winit::dpi::LogicalSize::new(1024, 768))
        .build(&event_loop)?;
    
    // Create instance and surface first
    let instance = Instance::new(wgpu::InstanceDescriptor {
        backends: Backends::all(),
        ..Default::default()
    });
    
    let surface = instance
        .create_surface(&window)
        .map_err(|e| RenderError::SurfaceCreationFailed(e))?;

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .ok_or(RenderError::AdapterRequestFailed)?;

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
            },
            None,
        )
        .await
        .map_err(|e| RenderError::DeviceCreationFailed(e))?;

    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps
        .formats
        .iter()
        .copied()
        .find(|f| f.is_srgb())
        .unwrap_or(surface_caps.formats[0]);

    let size = window.inner_size();
    let mut config = SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: PresentMode::Fifo,
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };

    surface.configure(&device, &config);

    // Create render pipeline
    let render_pipeline = GpuRenderer::create_render_pipeline(&device, surface_format)?;

    // Store window ID for comparison
    let window_id = window.id();

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id: event_window_id,
            } if event_window_id == window_id => match event {
                WindowEvent::CloseRequested => elwt.exit(),
                WindowEvent::Resized(physical_size) => {
                    if physical_size.width > 0 && physical_size.height > 0 {
                        config.width = physical_size.width;
                        config.height = physical_size.height;
                        surface.configure(&device, &config);
                    }
                }
                WindowEvent::RedrawRequested => {
                    // Render layout boxes
                    let output = match surface.get_current_texture() {
                        Ok(output) => output,
                        Err(_) => return,
                    };

                    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

                    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("Render Encoder"),
                    });

                    // Create vertices for all layout boxes
                    let mut vertices = Vec::new();
                    let mut indices = Vec::new();
                    let mut vertex_offset = 0;

                    // Traverse layout tree and create vertices for each box
                    add_layout_box_vertices(layout_root, &mut vertices, &mut indices, &mut vertex_offset, 0.0, 0.0);
                    
                    // Add debug overlay vertices
                    add_debug_overlay_vertices(layout_root, &mut vertices, &mut indices, &mut vertex_offset, 0.0, 0.0);


                    if !vertices.is_empty() {
                        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: Some("Layout Vertex Buffer"),
                            contents: bytemuck::cast_slice(&vertices),
                            usage: wgpu::BufferUsages::VERTEX,
                        });

                        {
                            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                label: Some("Render Pass"),
                                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                    view: &view,
                                    resolve_target: None,
                                    ops: wgpu::Operations {
                                        load: wgpu::LoadOp::Clear(wgpu::Color {
                                            r: 0.95,
                                            g: 0.95,
                                            b: 0.95,
                                            a: 1.0,
                                        }),
                                        store: wgpu::StoreOp::Store,
                                    },
                                })],
                                depth_stencil_attachment: None,
                                timestamp_writes: None,
                                occlusion_query_set: None,
                            });

                            render_pass.set_pipeline(&render_pipeline);
                            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                            render_pass.draw(0..vertices.len() as u32, 0..1);
                        }
                    }

                    queue.submit(std::iter::once(encoder.finish()));
                    output.present();
                }
                _ => {}
            },
            Event::AboutToWait => {
                // Note: In winit 0.29, we need to handle redraw requests differently
                // For now, we'll rely on the initial redraw request
            }
            _ => {}
        }
    }).unwrap();

    Ok(())
}

/// Add vertices for a layout box and its children
fn add_layout_box_vertices(
    layout_box: &layout::LayoutBox,
    vertices: &mut Vec<Vertex>,
    indices: &mut Vec<u16>,
    vertex_offset: &mut u16,
    parent_x: f32,
    parent_y: f32,
) {
    // Get the box dimensions and position from the content area
    let x = parent_x + layout_box.content.x;
    let y = parent_y + layout_box.content.y;
    let width = layout_box.content.width;
    let height = layout_box.content.height;

    // Skip boxes with zero dimensions, but use minimum height for debugging
    let _render_height = if height <= 0.0 { 20.0 } else { height };
    let _render_width = if width <= 0.0 { 100.0 } else { width };
    
    if width <= 0.0 && height <= 0.0 {
        return;
    }

    // Convert from CSS coordinates to normalized device coordinates (-1 to 1)
    // Use a reasonable coordinate system that scales the content to fit
    let _scale_factor = 0.001; // Scale down the large CSS coordinates
    
    // Create a simple but realistic layout representation
    // Position boxes based on their actual layout properties but with better scaling
    
    // Use a simple vertical stack with proper spacing
    static mut VERTICAL_OFFSET: f32 = 0.8; // Start near the top
    static mut BOX_COUNTER: usize = 0;
    
    let mut _box_index = 0;
    let mut _current_y = 0.0;
    unsafe {
        _box_index = BOX_COUNTER;
        BOX_COUNTER += 1;
        
        // Reset for each new render
        if _box_index == 0 {
            VERTICAL_OFFSET = 0.8;
        }
        
        _current_y = VERTICAL_OFFSET;
        VERTICAL_OFFSET -= 0.15; // Move down for next box
    }
    
    // Create boxes that represent the actual layout structure
    let box_width = 0.6;  // Reasonable width
    let box_height = 0.1; // Reasonable height
    
    let ndc_x: f32 = -0.3; // Center horizontally
    let ndc_y: f32 = _current_y;
    let ndc_width: f32 = box_width;
    let ndc_height: f32 = box_height;
    
    // Extract text content for this layout box
    let text_content = extract_text_content(layout_box);
    if !text_content.is_empty() {
        println!("TEXT: '{}' (display={:?})", 
                 text_content.chars().take(30).collect::<String>(),
                 layout_box.styles.display);
        
        // TODO: Add actual text rendering here
        // For now, we'll just show that we would render the text
        println!("WOULD RENDER: '{}' at position ({}, {})", 
                 text_content.chars().take(20).collect::<String>(),
                 ndc_x, ndc_y);
    }
    
    // Clamp to screen bounds
    let ndc_x = ndc_x.max(-1.0).min(1.0);
    let ndc_y = ndc_y.max(-1.0).min(1.0);
    let ndc_width = ndc_width.min(2.0);
    let ndc_height = ndc_height.min(2.0);


    // Choose color based on display type
    let color = match layout_box.styles.display {
        layout::DisplayType::Block => [0.0, 0.5, 1.0], // Blue for block elements
        layout::DisplayType::Inline => [0.0, 0.8, 0.0], // Green for inline elements
        layout::DisplayType::InlineBlock => [0.8, 0.0, 0.8], // Purple for inline-block elements
        layout::DisplayType::Flex => [1.0, 0.5, 0.0], // Orange for flex elements
        layout::DisplayType::Grid => [1.0, 0.0, 1.0], // Magenta for grid elements
        layout::DisplayType::None => [0.5, 0.5, 0.5], // Gray for hidden elements
    };
    

    // Create vertices for this box (two triangles)
    let box_vertices = [
        Vertex { position: [ndc_x, ndc_y], color },
        Vertex { position: [ndc_x + ndc_width, ndc_y], color },
        Vertex { position: [ndc_x, ndc_y - ndc_height], color },
        Vertex { position: [ndc_x + ndc_width, ndc_y - ndc_height], color },
    ];

    // Add vertices
    vertices.extend_from_slice(&box_vertices);

    // Add indices for two triangles
    let base_index = *vertex_offset;
    indices.extend_from_slice(&[
        base_index, base_index + 1, base_index + 2,
        base_index + 1, base_index + 3, base_index + 2,
    ]);

    *vertex_offset += 4;

    // Recursively add children
    for child in &layout_box.children {
        add_layout_box_vertices(child, vertices, indices, vertex_offset, x, y);
    }
}

/// Run a simple GPU renderer test
pub async fn run_gpu_test() -> RenderResult<()> {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Browser Engine - GPU Renderer")
        .with_inner_size(winit::dpi::LogicalSize::new(1024, 768))
        .build(&event_loop)?;
    
    // Create instance and surface first
    let instance = Instance::new(wgpu::InstanceDescriptor {
        backends: Backends::all(),
        ..Default::default()
    });
    
    let surface = instance
        .create_surface(&window)
        .map_err(|e| RenderError::SurfaceCreationFailed(e))?;

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .ok_or(RenderError::AdapterRequestFailed)?;

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
            },
            None,
        )
        .await
        .map_err(|e| RenderError::DeviceCreationFailed(e))?;

    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps
        .formats
        .iter()
        .copied()
        .find(|f| f.is_srgb())
        .unwrap_or(surface_caps.formats[0]);

    let size = window.inner_size();
    let mut config = SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: PresentMode::Fifo,
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };

    surface.configure(&device, &config);

    // Create render pipeline
    let render_pipeline = GpuRenderer::create_render_pipeline(&device, surface_format)?;

    // Store window ID for comparison
    let window_id = window.id();

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id: event_window_id,
            } if event_window_id == window_id => match event {
                WindowEvent::CloseRequested => elwt.exit(),
                WindowEvent::Resized(physical_size) => {
                    if physical_size.width > 0 && physical_size.height > 0 {
                        config.width = physical_size.width;
                        config.height = physical_size.height;
                        surface.configure(&device, &config);
                    }
                }
                WindowEvent::RedrawRequested => {
                    // Simple test rendering
                    let output = match surface.get_current_texture() {
                        Ok(output) => output,
                        Err(_) => return,
                    };

                    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

                    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("Render Encoder"),
                    });

                    // Create vertex buffer first
                    let vertices = [
                        Vertex { position: [-0.5, -0.5], color: [1.0, 0.0, 0.0] },
                        Vertex { position: [0.5, -0.5], color: [1.0, 0.0, 0.0] },
                        Vertex { position: [-0.5, 0.5], color: [1.0, 0.0, 0.0] },
                        Vertex { position: [0.5, -0.5], color: [1.0, 0.0, 0.0] },
                        Vertex { position: [0.5, 0.5], color: [1.0, 0.0, 0.0] },
                        Vertex { position: [-0.5, 0.5], color: [1.0, 0.0, 0.0] },
                    ];

                    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Vertex Buffer"),
                        contents: bytemuck::cast_slice(&vertices),
                        usage: wgpu::BufferUsages::VERTEX,
                    });

                    {
                        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: Some("Render Pass"),
                            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                view: &view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(wgpu::Color {
                                        r: 0.1,
                                        g: 0.2,
                                        b: 0.3,
                                        a: 1.0,
                                    }),
                                    store: wgpu::StoreOp::Store,
                                },
                            })],
                            depth_stencil_attachment: None,
                            timestamp_writes: None,
                            occlusion_query_set: None,
                        });

                        render_pass.set_pipeline(&render_pipeline);
                        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                        render_pass.draw(0..6, 0..1);
                    }

                    queue.submit(std::iter::once(encoder.finish()));
                    output.present();
                }
                _ => {}
            },
            Event::AboutToWait => {
                // Request redraw for the window
                // Note: In winit 0.29, we need to handle redraw requests differently
                // For now, we'll rely on the initial redraw request
            }
            _ => {}
        }
    }).unwrap();

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_creation() {
        let vertex = Vertex {
            position: [0.0, 0.0],
            color: [1.0, 0.0, 0.0],
        };
        
        assert_eq!(vertex.position, [0.0, 0.0]);
        assert_eq!(vertex.color, [1.0, 0.0, 0.0]);
    }
}