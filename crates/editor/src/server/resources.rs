use std::sync::{Arc, RwLock};

use ouroboros_common::bevy::prelude::{Deref, DerefMut};
use ouroboros_common::bevy::utils::HashMap;

use ouroboros_common::{ReflectObject, RemoteEntity};

#[derive(Clone, Default, Deref, DerefMut)]
pub struct EntityCache(pub Arc<RwLock<HashMap<RemoteEntity, HashMap<String, ReflectObject>>>>);
