use crate::{send_settarget, Entity, Socket, SystemHolder, ORDER_TARGET};
use graphics::*;
use hecs::World;

pub struct Target {
    pub entity: Option<Entity>,
    img_index: usize,
}

impl Target {
    pub fn new(systems: &mut SystemHolder) -> Self {
        let mut image = Image::new(
            Some(systems.resource.target.allocation),
            &mut systems.renderer,
            0,
        );
        image.hw = Vec2::new(40.0, 40.0);
        image.pos = Vec3::new(0.0, 0.0, ORDER_TARGET);
        image.uv = Vec4::new(0.0, 40.0, 40.0, 40.0);
        let img_index = systems.gfx.add_image(image, 0);
        systems.gfx.set_visible(img_index, false);

        Target {
            img_index,
            entity: None,
        }
    }

    pub fn recreate(&mut self, systems: &mut SystemHolder) {
        let mut image = Image::new(
            Some(systems.resource.target.allocation),
            &mut systems.renderer,
            0,
        );
        image.hw = Vec2::new(40.0, 40.0);
        image.pos = Vec3::new(0.0, 0.0, ORDER_TARGET);
        image.uv = Vec4::new(0.0, 0.0, 40.0, 40.0);
        let img_index = systems.gfx.add_image(image, 0);
        systems.gfx.set_visible(img_index, false);

        self.img_index = img_index;
        self.entity = None;
    }

    pub fn unload(&self, systems: &mut SystemHolder) {
        systems.gfx.remove_gfx(self.img_index);
    }

    pub fn set_target(
        &mut self,
        socket: &mut Socket,
        systems: &mut SystemHolder,
        target: &Entity,
    ) {
        self.entity = Some(*target);
        systems.gfx.set_visible(self.img_index, true);
        let _ = send_settarget(socket, self.entity);
    }

    pub fn clear_target(
        &mut self,
        socket: &mut Socket,
        systems: &mut SystemHolder,
    ) {
        self.entity = None;
        systems.gfx.set_visible(self.img_index, false);
        let _ = send_settarget(socket, self.entity);
    }

    pub fn set_target_pos(
        &mut self,
        socket: &mut Socket,
        systems: &mut SystemHolder,
        pos: Vec2,
    ) {
        let mut image_pos = systems.gfx.get_pos(self.img_index);
        let image_size = systems.gfx.get_size(self.img_index);
        image_pos.x = pos.x;
        image_pos.y = pos.y;
        systems.gfx.set_pos(self.img_index, image_pos);

        if image_pos.x + image_size.x < 0.0
            || image_pos.y + image_size.y < 0.0
            || image_pos.x > systems.size.width
            || image_pos.y > systems.size.height
        {
            self.clear_target(socket, systems);
        }
    }
}
