use leptos::*;

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


pub fn set<T, S>(signal: S) -> impl Fn(T)
where
    S: SignalSet<Value = T>,
{
    move |t| {
        signal.set(t);
    }   
}

