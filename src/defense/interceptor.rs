use bevy::prelude::Resource;

#[derive(Debug, Clone, Copy)]
pub struct Interceptor {
    bullet_id: u32,
    target_bullet_id: u32,
}

impl Interceptor {
    pub fn new(bullet_id: u32, target_bullet_id: u32) -> Self {
        Self {
            bullet_id,
            target_bullet_id,
        }
    }

    pub fn get_bullet_id(&self) -> u32 {
        self.bullet_id
    }

    pub fn get_target_bullet_id(&self) -> u32 {
        self.target_bullet_id
    }
}

#[derive(Resource, Default)]
pub struct InterceptorRegistry {
    interceptors: Vec<Interceptor>,
}

impl InterceptorRegistry {
    pub fn new() -> Self {
        Self {
            interceptors: Vec::new(),
        }
    }

    pub fn add(&mut self, interceptor: Interceptor) {
        self.interceptors.push(interceptor);
    }

    pub fn get_interceptors(&self) -> &[Interceptor] {
        &self.interceptors
    }

    pub fn get_active_count(&self) -> usize {
        self.interceptors.len()
    }

    pub fn is_target_engaged(&self, target_bullet_id: u32) -> bool {
        self.interceptors
            .iter()
            .any(|interceptor| { interceptor.get_target_bullet_id() == target_bullet_id })
    }

    // thanks chatgpt for this, atm I do not understand how to use the retain method, but I will learn it in the future
    pub fn retain(&mut self, predicate: impl FnMut(&Interceptor) -> bool) {
        self.interceptors.retain(predicate);
    }
}
