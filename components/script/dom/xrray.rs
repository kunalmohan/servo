/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::dom::bindings::codegen::Bindings::DOMPointBinding::DOMPointInit;
use crate::dom::bindings::codegen::Bindings::XRRayBinding::XRRayMethods;
use crate::dom::bindings::reflector::{reflect_dom_object, DomObject, Reflector};
use crate::dom::bindings::root::DomRoot;
use crate::dom::dompointreadonly::DOMPointReadOnly;
use crate::dom::globalscope::GlobalScope;
use crate::dom::window::Window;
use crate::dom::xrrigidtransform::XRRigidTransform;
use dom_struct::dom_struct;
use euclid::Vector3D;
use webxr_api::{ApiSpace, Ray};

#[dom_struct]
pub struct XRRay {
    reflector_: Reflector,
    #[ignore_malloc_size_of = "defined in webxr"]
    ray: Ray<ApiSpace>,
}

impl XRRay {
    fn new_inherited(ray: Ray<ApiSpace>) -> XRRay {
        XRRay {
            reflector_: Reflector::new(),
            ray,
        }
    }

    pub fn new(global: &GlobalScope, ray: Ray<ApiSpace>) -> DomRoot<XRRay> {
        reflect_dom_object(Box::new(XRRay::new_inherited(ray)), global)
    }

    #[allow(non_snake_case)]
    /// https://immersive-web.github.io/hit-test/#dom-xrray-xrray
    pub fn Constructor(
        window: &Window,
        origin: &DOMPointInit,
        direction: &DOMPointInit,
    ) -> DomRoot<Self> {
        let origin = Vector3D::new(origin.x as f32, origin.y as f32, origin.z as f32);
        let direction =
            Vector3D::new(direction.x as f32, direction.y as f32, direction.z as f32).normalize();

        Self::new(&window.global(), Ray { origin, direction })
    }

    #[allow(non_snake_case)]
    /// https://immersive-web.github.io/hit-test/#dom-xrray-xrray-transform
    pub fn Constructor_(window: &Window, transform: &XRRigidTransform) -> DomRoot<Self> {
        let transform = transform.transform();
        let origin = transform.translation;
        let direction = transform
            .rotation
            .transform_vector3d(Vector3D::new(0., 0., -1.));

        Self::new(&window.global(), Ray { origin, direction })
    }
}

impl XRRayMethods for XRRay {
    /// https://immersive-web.github.io/hit-test/#dom-xrray-origin
    fn Origin(&self) -> DomRoot<DOMPointReadOnly> {
        DOMPointReadOnly::new(
            &self.global(),
            self.ray.origin.x as f64,
            self.ray.origin.y as f64,
            self.ray.origin.z as f64,
            1.,
        )
    }

    /// https://immersive-web.github.io/hit-test/#dom-xrray-direction
    fn Direction(&self) -> DomRoot<DOMPointReadOnly> {
        DOMPointReadOnly::new(
            &self.global(),
            self.ray.direction.x as f64,
            self.ray.direction.y as f64,
            self.ray.direction.z as f64,
            0.,
        )
    }
}
