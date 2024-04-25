use parking_lot::{lock_api::ArcMutexGuard, Mutex, RawMutex};
use std::{
  ops::{Deref, DerefMut},
  sync::Arc,
};

#[derive(Debug)]
pub enum SlotState<T> {
  Value(T),
  Empty,
  Locked,
}

pub struct SlotRef<T>(ArcMutexGuard<RawMutex, Option<T>>);

impl<T> SlotRef<T> {
  pub fn steal(self) -> SlotState<T> {
    match ArcMutexGuard::<RawMutex, Option<T>>::mutex(&self.0).try_lock_arc() {
      Some(mut guard) => match guard.take() {
        Some(v) => SlotState::Value(v),
        None => SlotState::Empty,
      },
      None => SlotState::Locked,
    }
  }

  pub fn as_deref(&self) -> &<T as Deref>::Target
  where
    T: Deref,
  {
    // the presence of a value in the slot must be checked in the fn that generate the SlotRef
    self.0.as_deref().expect("BUG: call deref on empty slot")
  }

  pub fn as_deref_mut(&mut self) -> &mut <T as Deref>::Target
  where
    T: DerefMut,
  {
    // the presence of a value in the slot must be checked in the fn that generate the SlotRef
    self
      .0
      .as_deref_mut()
      .expect("BUG: call deref on empty slot")
  }
}

impl<T> Deref for SlotRef<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    // the presence of a value in the slot must be checked in the fn that generate the SlotRef
    self.0.as_ref().expect("BUG: call deref on empty slot")
  }
}

impl<T> DerefMut for SlotRef<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    // the presence of a value in the slot must be checked in the fn that generate the SlotRef
    self.0.as_mut().expect("BUG: call deref mut on empty slot")
  }
}

#[derive(Debug)]
pub struct Slot<T>(Arc<Mutex<Option<T>>>);

impl<T> Slot<T> {
  pub fn new(value: T) -> Self {
    Self(Arc::new(Mutex::new(Some(value))))
  }

  pub fn take(&self) -> SlotState<SlotRef<T>> {
    match self.0.try_lock_arc() {
      Some(v) => match v.as_ref() {
        Some(_) => SlotState::Value(SlotRef(v)),
        None => SlotState::Empty,
      },
      None => SlotState::Locked,
    }
  }

  pub fn steal(self) -> SlotState<T> {
    match self.0.try_lock_arc() {
      Some(mut guard) => match guard.take() {
        Some(v) => SlotState::Value(v),
        None => SlotState::Empty,
      },
      None => SlotState::Locked,
    }
  }
}

impl<T> Clone for Slot<T> {
  fn clone(&self) -> Self {
    Self(Arc::clone(&self.0))
  }
}
