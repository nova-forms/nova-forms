use leptos::*;

/// Create a read signal that applies a unary operation on the write signal.
pub fn use_unop<F, I1, O>(cond: F) -> (Signal<O>, WriteSignal<I1>)
where
    I1: Clone + Default,
    O: Clone,
    F: Fn(I1) -> O + 'static,
{
    let (i1_signal, set_i1_signal) = create_signal(I1::default());

    (Signal::derive(move || cond(i1_signal.get())), set_i1_signal)
}

/// Create a read signal that applies a binary operation on the write signals.
pub fn use_binop<F, I1, I2, O>(cond: F) -> (Signal<O>, WriteSignal<I1>, WriteSignal<I2>)
where
    I1: Clone + Default,
    I2: Clone + Default,
    O: Clone,
    F: Fn(I1, I2) -> O + 'static,
{
    let (i1_signal, set_i1_signal) = create_signal(I1::default());
    let (i2_signal, set_i2_signal) = create_signal(I2::default());

    (Signal::derive(move || cond(i1_signal.get(), i2_signal.get())), set_i1_signal, set_i2_signal)
}