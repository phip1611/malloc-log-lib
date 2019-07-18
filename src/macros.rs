/// Wraps Code that has mallocs/frees inside, that should be delegated IMMEDIATELY to
/// the original implementation. There are two edge-cases when we want to do this:
/// 1) we have code inside malloc/free that needs malloc/free itself (prevent endless recursion)
/// 2) we have initialization-code that needs mallocs/frees and we don't want to log these calls
/// Only does the action for the very top macro if macros are chained
#[macro_export]
macro_rules! malloc_no_conflict {
    ($code: block) => {
        crate::internal_malloc_no_conflict!($code);
    };
    ($code: stmt) => {
        // making Statement to Block
        crate::internal_malloc_no_conflict!({$code;});
    };
}
#[macro_export]
macro_rules! internal_malloc_no_conflict {
    ($code: block) => {
        // Thread-Local Value
        let tl_is_master = !malloc_recur_protection::get_is_in_macro_chain();

        if tl_is_master {
            malloc_recur_protection::truify_is_in_macro_chain();
            malloc_recur_protection::enable_return_immediately();
        }

        $code;

        if tl_is_master {
            malloc_recur_protection::falsify_is_in_macro_chain();
            malloc_recur_protection::disable_return_immediately();
        }
    }
}
