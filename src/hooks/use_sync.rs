use leptos::*;

pub fn use_unop<F, I1, O>(cond: F) -> (WriteSignal<I1>, Signal<O>)
where
    I1: Clone + Default,
    O: Clone,
    F: Fn(I1) -> O + 'static,
{
    let (i1_signal, set_i1_signal) = create_signal(I1::default());

    (set_i1_signal, Signal::derive(move || cond(i1_signal.get())))
}

pub fn use_binop<F, I1, I2, O>(cond: F) -> (WriteSignal<I1>, WriteSignal<I2>, Signal<O>)
where
    I1: Clone + Default,
    I2: Clone + Default,
    O: Clone,
    F: Fn(I1, I2) -> O + 'static,
{
    let (i1_signal, set_i1_signal) = create_signal(I1::default());
    let (i2_signal, set_i2_signal) = create_signal(I2::default());

    (set_i1_signal, set_i2_signal, Signal::derive(move || cond(i1_signal.get(), i2_signal.get())))
}