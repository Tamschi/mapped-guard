use {
    core::{mem::transmute, ops::Deref},
    std::sync::{MutexGuard, RwLockReadGuard, RwLockWriteGuard},
};

// There are a few requirements that must hold for this library to be sound:
// I: G is movable independently from R (base premise)
// II: It is impossible to separate G and R, except if that transformation is guarateed to keep R valid which is enabled by III.
// III: Consumers can't acquire a direct reference to `target` while consuming `self`.
#[derive(Debug)]
pub struct MappedGuard<G, R> {
    guard: G,
    target: R,
}

impl<G, R> MappedGuard<G, R> {
    pub fn new(guard: G, target: R) -> Self {
        Self { guard, target }
    }
}

impl<G, R: Deref> Deref for MappedGuard<G, R> {
    // Safety: III only holds if the deref defers to R's Target because MappedGuard is UnpinDereferenced!
    type Target = R::Target;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.target
    }
}

/// Marker types for guards that are mapped by first being boxed (because they make no guarantees that their target reference can be detached from their location in memory.)
/// Technically this could be anything, but the operation only really makes sense for guards, in order to return the mapping from a function.
pub trait BoxedMapped {}
impl<'a, T> BoxedMapped for MutexGuard<'a, T> {}
impl<'a, T> BoxedMapped for RwLockReadGuard<'a, T> {}
impl<'a, T> BoxedMapped for RwLockWriteGuard<'a, T> {}

// TODO: This is a bit inefficient. Provide another implementation that uses a flatter mapping if R is Deref.
impl<G, R> BoxedMapped for MappedGuard<G, R> {}

//TODO: Mention this in the struct documentation!
impl<G, R1, R> From<MappedGuard<MappedGuard<G, R1>, R>> for MappedGuard<G, R> {
    /// Flattens nested MappedGuard instances.
    #[inline]
    fn from(from: MappedGuard<MappedGuard<G, R1>, R>) -> Self {
        Self {
            guard: from.guard.guard,
            target: from.target,
        }
    }
}

pub trait MapGuard<'a, G, R1, R2: 'a> {
    fn map_guard(self, map: impl FnOnce(R1) -> R2) -> MappedGuard<G, R2>;

    //TODO: What's the naming convention for this?
    fn maybe_map_guard(
        self,
        maybe_map: impl FnOnce(R1) -> Option<R2>,
    ) -> Option<MappedGuard<G, R2>>;
}
pub trait TryMapGuard<'a, G, R1, R2: 'a, E: 'a> {
    fn try_map_guard(
        self,
        try_map: impl FnOnce(R1) -> Result<R2, E>,
    ) -> Result<MappedGuard<G, R2>, E>;
}

//TODO: These should be default implementations and associated types instead.
impl<'a, G: BoxedMapped + 'a, R: 'a> MapGuard<'a, Box<G>, &'a G, R> for G {
    #[inline]
    fn map_guard(self, map: impl FnOnce(&'a G) -> R) -> MappedGuard<Box<G>, R> {
        let boxed = Box::new(self);
        MappedGuard {
            target: map(unsafe {
                //SAFETY: `guard` can't be dropped independently from `target`.
                detach_borrow(&*boxed)
            }),
            guard: boxed,
        }
    }

    #[inline]
    fn maybe_map_guard(
        self,
        maybe_map: impl FnOnce(&'a G) -> Option<R>,
    ) -> Option<MappedGuard<Box<G>, R>> {
        let boxed = Box::new(self);
        maybe_map(unsafe {
            //SAFETY: `guard` can't be dropped independently from `target`.
            detach_borrow(&*boxed)
        })
        .map(|target| MappedGuard {
            target,
            guard: boxed,
        })
    }
}
impl<'a, G: BoxedMapped + 'a, R: 'a, E: 'a> TryMapGuard<'a, Box<G>, &'a G, R, E> for G {
    #[inline]
    fn try_map_guard(
        self,
        try_map: impl FnOnce(&'a G) -> Result<R, E>,
    ) -> Result<MappedGuard<Box<G>, R>, E> {
        let boxed = Box::new(self);
        Ok(MappedGuard {
            target: try_map(unsafe {
                //SAFETY: `guard` can't be dropped independently from `target`.
                detach_borrow(&*boxed)
            })?,
            guard: boxed,
        })
    }
}

#[inline]
unsafe fn detach_borrow<'a, 'b, T>(reference: &'a T) -> &'b T {
    transmute(reference)
}
