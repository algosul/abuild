use std::{any::Any, error::Error};
pub mod modules;
pub trait Module<Parent> {
    type D0: Module<Parent> = ();
    type D1: Module<Parent> = ();
    type D2: Module<Parent> = ();
    fn do_it(
        &mut self, parent: &mut Parent,
        dependencies: (&Self::D0, &Self::D1, &Self::D2),
    ) -> Result<(), Box<dyn Error>>;
    fn redo_it(
        &mut self, parent: &mut Parent,
        dependencies: (&Self::D0, &Self::D1, &Self::D2),
    ) -> Result<(), Box<dyn Error>>;
    fn undo_it(
        &mut self, parent: &mut Parent,
        dependencies: (&Self::D0, &Self::D1, &Self::D2),
    ) -> Result<(), Box<dyn Error>>;
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynModule<Parent> {
    inner: Box<dyn DynModuleImpl<Parent>>,
}
pub(crate) trait DynModuleImpl<Parent> {
    fn dyn_do_it(
        &mut self, parent: &mut Parent,
        dependencies: (&dyn Any, &dyn Any, &dyn Any),
    ) -> Result<(), Box<dyn Error>>;
    fn dyn_redo_it(
        &mut self, parent: &mut Parent,
        dependencies: (&dyn Any, &dyn Any, &dyn Any),
    ) -> Result<(), Box<dyn Error>>;
    fn dyn_undo_it(
        &mut self, parent: &mut Parent,
        dependencies: (&dyn Any, &dyn Any, &dyn Any),
    ) -> Result<(), Box<dyn Error>>;
}
impl<T: Module<Parent> + Any, Parent> DynModuleImpl<Parent> for T
where
    T::D0: 'static,
    T::D1: 'static,
    T::D2: 'static,
{
    fn dyn_do_it(
        &mut self, parent: &mut Parent,
        dependencies: (&dyn Any, &dyn Any, &dyn Any),
    ) -> Result<(), Box<dyn Error>> {
        self.do_it(
            parent,
            (
                dependencies.0.downcast_ref::<T::D0>().unwrap(),
                dependencies.1.downcast_ref::<T::D1>().unwrap(),
                dependencies.2.downcast_ref::<T::D2>().unwrap(),
            ),
        )
    }

    fn dyn_redo_it(
        &mut self, parent: &mut Parent,
        dependencies: (&dyn Any, &dyn Any, &dyn Any),
    ) -> Result<(), Box<dyn Error>> {
        self.redo_it(
            parent,
            (
                dependencies.0.downcast_ref::<T::D0>().unwrap(),
                dependencies.1.downcast_ref::<T::D1>().unwrap(),
                dependencies.2.downcast_ref::<T::D2>().unwrap(),
            ),
        )
    }

    fn dyn_undo_it(
        &mut self, parent: &mut Parent,
        dependencies: (&dyn Any, &dyn Any, &dyn Any),
    ) -> Result<(), Box<dyn Error>> {
        self.undo_it(
            parent,
            (
                dependencies.0.downcast_ref::<T::D0>().unwrap(),
                dependencies.1.downcast_ref::<T::D1>().unwrap(),
                dependencies.2.downcast_ref::<T::D2>().unwrap(),
            ),
        )
    }
}
impl<Parent> Module<Parent> for () {
    type D0 = ();
    type D1 = ();
    type D2 = ();

    fn do_it(
        &mut self, _parent: &mut Parent,
        _dependencies: (&Self::D0, &Self::D1, &Self::D2),
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn redo_it(
        &mut self, _parent: &mut Parent,
        _dependencies: (&Self::D0, &Self::D1, &Self::D2),
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn undo_it(
        &mut self, _parent: &mut Parent,
        _dependencies: (&Self::D0, &Self::D1, &Self::D2),
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
impl<Parent> Module<Parent> for DynModule<Parent> {
    type D0 = dyn DynModuleImpl<Parent>;
    type D1 = dyn DynModuleImpl<Parent>;
    type D2 = dyn DynModuleImpl<Parent>;

    fn do_it(
        &mut self, parent: &mut Parent,
        dependencies: (&Self::D0, &Self::D1, &Self::D2),
    ) -> Result<(), Box<dyn Error>> {
        self.inner.dyn_do_it(parent, dependencies)
    }

    fn redo_it(
        &mut self, parent: &mut Parent,
        dependencies: (&Self::D0, &Self::D1, &Self::D2),
    ) -> Result<(), Box<dyn Error>> {
        self.inner.dyn_redo_it(parent, dependencies)
    }

    fn undo_it(
        &mut self, parent: &mut Parent,
        dependencies: (&Self::D0, &Self::D1, &Self::D2),
    ) -> Result<(), Box<dyn Error>> {
        self.inner.dyn_undo_it(parent, dependencies)
    }
}
