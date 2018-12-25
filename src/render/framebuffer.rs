// FIXME: Play with different refratin and reflection sizes to see whwat looks good
//static REFLECTION_TEXTURE_WIDTH: i32 = 128;
//static REFLECTION_TEXTURE_HEIGHT: i32 = 128;
static REFLECTION_TEXTURE_WIDTH: i32 = 512; // TODO: DELETE in favor of above
static REFLECTION_TEXTURE_HEIGHT: i32 = 512; // TODO: DELETE

static REFRACTION_TEXTURE_WIDTH: i32 = 512;
static REFRACTION_TEXTURE_HEIGHT: i32 = 512;

use crate::render::TextureUnit;
use crate::render::WebRenderer;
use wasm_bindgen::JsValue;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

pub struct Framebuffer {
    pub framebuffer: Option<WebGlFramebuffer>,
    pub color_texture: Option<WebGlTexture>,
    pub depth_texture: Option<WebGlTexture>,
}

impl WebRenderer {
    pub(in crate::render) fn create_refraction_framebuffer(
        gl: &WebGlRenderingContext,
    ) -> Result<Framebuffer, JsValue> {
        let framebuffer = gl.create_framebuffer();
        gl.bind_framebuffer(GL::FRAMEBUFFER, framebuffer.as_ref());

        gl.active_texture(TextureUnit::Refraction.get());
        let color_texture = gl.create_texture();
        gl.bind_texture(GL::TEXTURE_2D, color_texture.as_ref());

        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::LINEAR as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
            GL::TEXTURE_2D,
            0,
            GL::RGBA as i32,
            REFRACTION_TEXTURE_WIDTH,
            REFRACTION_TEXTURE_HEIGHT,
            0,
            GL::RGBA as u32,
            GL::UNSIGNED_BYTE,
            None,
        )?;

        let depth_texture = gl.create_texture();
        gl.active_texture(TextureUnit::RefractionDepth.get());
        gl.bind_texture(GL::TEXTURE_2D, depth_texture.as_ref());
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::LINEAR as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
            GL::TEXTURE_2D,
            0,
            GL::DEPTH_COMPONENT as i32,
            REFRACTION_TEXTURE_WIDTH,
            REFRACTION_TEXTURE_HEIGHT,
            0,
            GL::DEPTH_COMPONENT as u32,
            // FIXME: UNSIGNED_BYTE should be fine here since we don't need as much precision
            // since it doesn't matter if there are two objects next to eachother and our
            // depth is very slightly off. Precision is more important in shadow mapping
            GL::UNSIGNED_SHORT,
            None,
        )?;

        gl.framebuffer_texture_2d(
            GL::FRAMEBUFFER,
            GL::COLOR_ATTACHMENT0,
            GL::TEXTURE_2D,
            color_texture.as_ref(),
            0,
        );

        gl.framebuffer_texture_2d(
            GL::FRAMEBUFFER,
            GL::DEPTH_ATTACHMENT,
            GL::TEXTURE_2D,
            depth_texture.as_ref(),
            0,
        );

        gl.bind_framebuffer(GL::FRAMEBUFFER, None);

        Ok(Framebuffer {
            framebuffer,
            color_texture,
            depth_texture,
        })
    }

    // FIXME: Normalize with refraction framebuffer
    pub(in crate::render) fn create_reflection_framebuffer(
        gl: &WebGlRenderingContext,
    ) -> Result<Framebuffer, JsValue> {
        let framebuffer = gl.create_framebuffer();
        gl.bind_framebuffer(GL::FRAMEBUFFER, framebuffer.as_ref());

        let color_texture = gl.create_texture();

        gl.active_texture(TextureUnit::Reflection.get());
        gl.bind_texture(GL::TEXTURE_2D, color_texture.as_ref());
        // FIXME: Constant for canvas width and height that we get from the canvas module
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::LINEAR as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
            GL::TEXTURE_2D,
            0,
            GL::RGBA as i32,
            REFLECTION_TEXTURE_WIDTH,
            REFLECTION_TEXTURE_HEIGHT,
            0,
            GL::RGBA as u32,
            GL::UNSIGNED_BYTE,
            None,
        )?;

        // FIXME: Research render buffer so that I understand it and can describe it in comments.
        // Same with pretty much every WebGL API that we call
        let renderbuffer = gl.create_renderbuffer();
        gl.bind_renderbuffer(GL::RENDERBUFFER, renderbuffer.as_ref());
        gl.renderbuffer_storage(
            GL::RENDERBUFFER,
            GL::DEPTH_COMPONENT16,
            REFLECTION_TEXTURE_WIDTH,
            REFLECTION_TEXTURE_HEIGHT,
        );

        gl.framebuffer_texture_2d(
            GL::FRAMEBUFFER,
            GL::COLOR_ATTACHMENT0,
            GL::TEXTURE_2D,
            color_texture.as_ref(),
            0,
        );
        gl.framebuffer_renderbuffer(
            GL::FRAMEBUFFER,
            GL::DEPTH_ATTACHMENT,
            GL::RENDERBUFFER,
            renderbuffer.as_ref(),
        );

        gl.bind_renderbuffer(GL::RENDERBUFFER, None);
        gl.bind_framebuffer(GL::FRAMEBUFFER, None);

        Ok(Framebuffer {
            framebuffer,
            color_texture,
            depth_texture: None,
        })
    }
}