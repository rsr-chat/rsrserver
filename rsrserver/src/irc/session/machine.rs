use crate::irc::{IrcContext, state};

/// Marker trait describing a next possible valid
/// state transition from the current state.
pub trait StateInto<T> {}

/// Unconditionally no transition.
pub struct Old<T>(pub T);

/// Unconditionally transition.
pub struct New<U>(pub U);

/// Conditionally transition at runtime.
/// Any initial state `T` must implement [StateInto<U>]
/// for some `U` next state.
pub enum MaybeTransition<T, U>
where
    T: StateInto<U>,
{
    Old(T),
    New(U),
}

pub trait IntoTransition<T, U>
where
    T: StateInto<U>,
{
    fn apply(self) -> MaybeTransition<T, U>;
}

impl<T, U> IntoTransition<T, U> for MaybeTransition<T, U>
where
    T: StateInto<U>,
{
    fn apply(self) -> MaybeTransition<T, U> {
        self
    }
}

impl<T> From<T> for Old<T> {
    fn from(value: T) -> Self {
        Old(value)
    }
}

impl<T> From<T> for New<T> {
    fn from(value: T) -> Self {
        New(value)
    }
}

impl<T, U> From<Old<T>> for MaybeTransition<T, U>
where
    T: StateInto<U>,
{
    fn from(value: Old<T>) -> Self {
        MaybeTransition::Old(value.0)
    }
}

impl<T, U> From<New<U>> for MaybeTransition<T, U>
where
    T: StateInto<U>,
{
    fn from(value: New<U>) -> Self {
        MaybeTransition::New(value.0)
    }
}

impl<T, U, S> From<Old<IrcContext<'_, T, S>>> for MaybeTransition<T, U>
where
    T: StateInto<U>,
{
    fn from(value: Old<IrcContext<'_, T, S>>) -> Self {
        MaybeTransition::Old(value.0.apply())
    }
}

impl<T, U, S> From<New<IrcContext<'_, U, S>>> for MaybeTransition<T, U>
where
    T: StateInto<U>,
{
    fn from(value: New<IrcContext<'_, U, S>>) -> Self {
        MaybeTransition::New(value.0.apply())
    }
}

macro_rules! impl_from_transitions {
    ($t:ty) => {
        impl<T> From<$t> for MaybeTransition<T, $t> where T: StateInto<$t> {
            fn from(value: $t) -> Self {
                MaybeTransition::New(value)
            }
        }

        impl<U> From<$t> for MaybeTransition<$t, U> where $t: StateInto<U> {
            fn from(value: $t) -> Self {
                MaybeTransition::Old(value)
            }
        }
    };
}

impl_from_transitions!(state::Anonymous);
impl_from_transitions!(state::Registered);
impl_from_transitions!(state::Authenticated);