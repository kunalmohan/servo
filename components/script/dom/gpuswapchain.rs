/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::dom::bindings::cell::DomRefCell;
use crate::dom::bindings::codegen::Bindings::GPUSwapChainBinding::GPUSwapChainMethods;
use crate::dom::bindings::codegen::Bindings::GPUTextureBinding::{
    GPUTextureFormat, GPUTextureMethods,
};
use crate::dom::bindings::reflector::{reflect_dom_object, Reflector};
use crate::dom::bindings::root::{Dom, DomRoot, MutDom};
use crate::dom::bindings::str::DOMString;
use crate::dom::globalscope::GlobalScope;
use crate::dom::gpucanvascontext::GPUCanvasContext;
use crate::dom::gpudevice::GPUDevice;
use crate::dom::gputexture::GPUTexture;
use dom_struct::dom_struct;
use webgpu::{WebGPU, WebGPURequest, WebGPUTexture};

#[dom_struct]
pub struct GPUSwapChain {
    reflector_: Reflector,
    #[ignore_malloc_size_of = "channels are hard"]
    channel: WebGPU,
    label: DomRefCell<Option<DOMString>>,
    context: Dom<GPUCanvasContext>,
    texture: MutDom<GPUTexture>,
    device: Dom<GPUDevice>,
    format: GPUTextureFormat,
    usage: u32,
}

impl GPUSwapChain {
    fn new_inherited(
        channel: WebGPU,
        context: &GPUCanvasContext,
        texture: &GPUTexture,
        device: &GPUDevice,
        format: GPUTextureFormat,
        usage: u32,
    ) -> Self {
        Self {
            reflector_: Reflector::new(),
            channel,
            context: Dom::from_ref(context),
            texture: MutDom::new(texture),
            label: DomRefCell::new(None),
            device: Dom::from_ref(device),
            format,
            usage,
        }
    }

    pub fn new(
        global: &GlobalScope,
        channel: WebGPU,
        context: &GPUCanvasContext,
        texture: &GPUTexture,
        device: &GPUDevice,
        format: GPUTextureFormat,
        usage: u32,
    ) -> DomRoot<Self> {
        reflect_dom_object(
            Box::new(GPUSwapChain::new_inherited(
                channel, context, texture, device, format, usage,
            )),
            global,
        )
    }
}

impl GPUSwapChain {
    pub fn destroy(&self, external_id: u64, image_key: webrender_api::ImageKey) {
        if let Err(e) = self.channel.0.send(WebGPURequest::DestroySwapChain {
            external_id,
            image_key,
        }) {
            warn!(
                "Failed to send DestroySwapChain-ImageKey({:?}) ({})",
                image_key, e
            );
        }
        self.texture.get().Destroy();
    }

    pub fn texture_id(&self) -> WebGPUTexture {
        self.texture.get().id()
    }

    pub fn update_texture(&self, texture: &GPUTexture) {
        self.texture.set(texture);
    }

    pub fn format(&self) -> GPUTextureFormat {
        self.format
    }

    pub fn usage(&self) -> u32 {
        self.usage
    }

    pub fn device(&self) -> &GPUDevice {
        &*self.device
    }
}

impl GPUSwapChainMethods for GPUSwapChain {
    /// https://gpuweb.github.io/gpuweb/#dom-gpuobjectbase-label
    fn GetLabel(&self) -> Option<DOMString> {
        self.label.borrow().clone()
    }

    /// https://gpuweb.github.io/gpuweb/#dom-gpuobjectbase-label
    fn SetLabel(&self, value: Option<DOMString>) {
        *self.label.borrow_mut() = value;
    }

    /// https://gpuweb.github.io/gpuweb/#dom-gpuswapchain-getcurrenttexture
    fn GetCurrentTexture(&self) -> DomRoot<GPUTexture> {
        self.context.mark_as_dirty();
        self.texture.get()
    }
}
