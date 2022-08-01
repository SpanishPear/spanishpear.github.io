#[macro_export]
macro_rules! cloned_closure {
    ($($vars: ident),*; |$($param:ident),*| $body:expr) => {
        {
            $(let $vars = $vars.clone();)*
            move |$($param),*| $body
        }
    };
}