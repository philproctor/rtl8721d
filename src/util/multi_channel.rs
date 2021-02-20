use crate::prelude::*;
use alloc::collections::btree_map::BTreeMap;

#[derive(Debug, Default)]
pub struct MultiChannel<T> {
    cm: Mutex<BTreeMap<u32, Channel<T>>>,
}

unsafe impl<T> Sync for MultiChannel<T> {}
unsafe impl<T> Send for MultiChannel<T> {}

impl<T> MultiChannel<T> {
    pub fn new() -> Self {
        Self {
            cm: Mutex::new(Default::default()),
        }
    }

    pub fn blocking_send(&self, channel_id: u32, val: T) {
        let mut cm = self.cm.spin_lock();
        if let Some(ch) = cm.get(&channel_id) {
            ch.blocking_send(val);
        } else {
            let new_ch = Channel::new();
            new_ch.blocking_send(val);
            cm.insert(channel_id, new_ch);
        }
    }

    pub unsafe fn irq_send(&self, channel_id: u32, val: T) {
        let cm = self.cm.bypass_lock();
        if let Some(ch) = cm.get(&channel_id) {
            ch.irq_send(val);
        } else {
            let new_ch = Channel::new();
            new_ch.irq_send(val);
            cm.insert(channel_id, new_ch);
        }
    }

    pub async fn send(&self, channel_id: u32, val: T) {
        let mut cm = self.cm.lock().await;
        if let Some(ch) = cm.get(&channel_id) {
            ch.send(val).await;
        } else {
            let new_ch = Channel::new();
            new_ch.send(val).await;
            cm.insert(channel_id, new_ch);
        }
    }

    pub fn blocking_recv(&self, channel_id: u32) -> Option<T> {
        let mut cm = self.cm.spin_lock();
        if let Some(ch) = cm.get(&channel_id) {
            ch.blocking_recv()
        } else {
            let new_ch = Channel::new();
            cm.insert(channel_id, new_ch);
            None
        }
    }

    pub async fn recv(&self, channel_id: u32) -> Option<T> {
        let mut cm = self.cm.lock().await;
        if let Some(ch) = cm.get(&channel_id) {
            ch.recv().await
        } else {
            let new_ch = Channel::new();
            cm.insert(channel_id, new_ch);
            None
        }
    }
}
