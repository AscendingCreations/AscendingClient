use crate::{
    GlobalKey, HPBar, ORDER_TARGET, Result, SystemHolder, data_types::*,
    send_settarget, systems::Poller,
};
use graphics::*;

pub struct Target {
    pub entity: Option<GlobalKey>,
    img_index: GfxType,
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

        let img_index = systems.gfx.add_image(image, 0, "Target Image", false);

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

        let img_index = systems.gfx.add_image(image, 0, "Target Image", false);

        self.img_index = img_index;
        self.entity = None;
    }

    pub fn unload(&self, systems: &mut SystemHolder) {
        systems
            .gfx
            .remove_gfx(&mut systems.renderer, &self.img_index);
    }

    pub fn set_target(
        &mut self,
        socket: &mut Poller,
        systems: &mut SystemHolder,
        target: GlobalKey,
    ) -> Result<()> {
        self.entity = Some(target);
        systems.gfx.set_visible(&self.img_index, true);
        send_settarget(socket, self.entity)
    }

    pub fn clear_target(
        &mut self,
        socket: &mut Poller,
        systems: &mut SystemHolder,
        hpbar: &mut HPBar,
    ) -> Result<()> {
        self.entity = None;
        systems.gfx.set_visible(&self.img_index, false);

        hpbar.visible = false;
        systems.gfx.set_visible(&hpbar.bar_index, false);
        systems.gfx.set_visible(&hpbar.bg_index, false);

        send_settarget(socket, self.entity)
    }

    pub fn set_target_pos(
        &mut self,
        socket: &mut Poller,
        systems: &mut SystemHolder,
        pos: Vec2,
        hpbar: &mut HPBar,
    ) -> Result<()> {
        let mut image_pos = systems.gfx.get_pos(&self.img_index);
        let image_size = systems.gfx.get_size(&self.img_index);
        image_pos.x = pos.x;
        image_pos.y = pos.y;
        systems.gfx.set_pos(&self.img_index, image_pos);

        if image_pos.x + image_size.x < 0.0
            || image_pos.y + image_size.y < 0.0
            || image_pos.x > systems.size.width
            || image_pos.y > systems.size.height
        {
            self.clear_target(socket, systems, hpbar)?;
        }

        Ok(())
    }
}
