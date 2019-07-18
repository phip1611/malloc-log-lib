/// Wraps Code that has mallocs/frees inside, that should be delegated IMMEDIATELY to
/// the original implementation. There are two edge-cases when we want to do this:
/// 1) we have code inside malloc/free that needs malloc/free itself (prevent endless recursion)
/// 2) we have initialization-code that needs mallocs/frees and we don't want to log these calls
#[macro_export]
macro_rules! malloc_no_conflict {
    ($code: stmt) => {{
        if !endless_recur_protection::get_return_immediately() {
            endless_recur_protection::enable_return_immediately();
            $code;
            endless_recur_protection::disable_return_immediately();
        }
    }};
    ($code: block) => {{
        if !endless_recur_protection::get_return_immediately() {
            endless_recur_protection::enable_return_immediately();
            $code;
            endless_recur_protection::disable_return_immediately();
        }
    }}
}
