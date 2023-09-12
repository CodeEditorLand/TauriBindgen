#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod simple_lists {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
    #[::tauri_bindgen_host::async_trait]
    pub trait SimpleLists: Sized {
        async fn simple_list1(&self, l: Vec<u32>);
        async fn simple_list2(&self) -> Vec<u32>;
        async fn simple_list3(&self, a: Vec<u32>, b: Vec<u32>) -> (Vec<u32>, Vec<u32>);
        async fn simple_list4(&self, l: Vec<Vec<u32>>) -> Vec<Vec<u32>>;
    }
    pub fn add_to_router<T, U, R>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T, R>,
        get_cx: impl Fn(&T) -> &U + Send + Sync + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        T: Send + Sync + 'static,
        U: SimpleLists + Send + Sync + 'static,
        R: ::tauri_bindgen_host::tauri::Runtime,
    {
        let wrapped_get_cx = ::std::sync::Arc::new(get_cx);
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "simple_lists",
                "simple_list1",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: Vec<u32>| {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        Ok(ctx.simple_list1(p).await)
                    })
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "simple_lists",
                "simple_list2",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        Ok(ctx.simple_list2().await)
                    })
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "simple_lists",
                "simple_list3",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: (Vec<u32>, Vec<u32>)|
                {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        Ok(ctx.simple_list3(p.0, p.1).await)
                    })
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "simple_lists",
                "simple_list4",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: Vec<Vec<u32>>|
                {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        Ok(ctx.simple_list4(p).await)
                    })
                },
            )?;
        Ok(())
    }
}
