use std::collections::HashMap;
use std::sync::Arc;

use {BoxConstraints, Entity, EntityComponentManager, LayoutObject, LayoutResult, Theme};

pub struct DefaultLayoutObject;

impl LayoutObject for DefaultLayoutObject {
    fn layout(
        &self,
        _entity: Entity,
        _ecm: &EntityComponentManager,
        bc: &BoxConstraints,
        children: &[Entity],
        children_pos: &mut HashMap<Entity, (i32, i32)>,
        size: Option<(u32, u32)>,
        _theme: &Arc<Theme>,
    ) -> LayoutResult {
        if let Some(size) = size {
            children_pos.insert(children[0], (0, 0));
            LayoutResult::Size(size)
        } else {
            if children.len() == 0 {
                return LayoutResult::Size((bc.min_width, bc.min_height));
            }
            LayoutResult::RequestChild(children[0], *bc)
        }
    }
}
