use leptos::*;

/// Wires multiple input signals to an output signal using the given function.
#[macro_export]
macro_rules! wire {
    ( move | $($input:ident),* | $($t:tt)* ) => {
        {
            $(
                let $input = leptos::create_signal(Default::default());
            )*

            let output = leptos::Signal::derive(move || {
                (|$($input),*| {
                    $($t)*
                })($($input.0.get()),*)
            });

            (output, $( $input.1 ),*)
        }  
    };
}

/// Calls the input function only if the returned functions result is ok.
pub fn on_ok<F, T, E>(f: F) -> impl Fn(Result<T, E>)
where
    F: Fn(T) + 'static,
{
    move |result| {
        if let Ok(t) = result {
            f(t)
        }
    }   
}

/// Calls the input function only if the returned functions result is an error.
pub fn on_err<F, T, E>(f: F) -> impl Fn(Result<T, E>)
where
    F: Fn(E) + 'static,
{
    move |result| {
        if let Err(err) = result {
            f(err)
        }
    }   
}

/// Returns a function that sets a signal.
pub fn set<T, S>(signal: S) -> impl Fn(T)
where
    S: SignalSet<Value = T>,
{
    move |t| {
        signal.set(t);
    }   
}

