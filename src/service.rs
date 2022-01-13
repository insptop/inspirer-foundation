use std::sync::Arc;

use tokio::sync::RwLock;
use type_map::concurrent::TypeMap;

#[derive(Clone)]
pub struct Service {
    inner: ServiceInner,
}

#[derive(Clone)]
struct ServiceInner {
    pub(crate) components: Arc<RwLock<TypeMap>>,
}

impl Service {
    pub async fn component<T: 'static + Clone>(&self) -> T {
        self.inner.components
            .read()
            .await
            .get::<T>()
            .cloned()
            .expect("Component not found!")
    }
}
