use actix::{Actor, AsyncContext, WrapFuture};
use futures::Future;
use actix::ContextFutureSpawner;

pub trait FutureSpawnExt {
    fn spawn<A: Actor + Actor<Context = C>, C: AsyncContext<A>>(self, actor: &A, ctx: &mut C);
}

impl<F: Future + 'static> FutureSpawnExt for F {
    fn spawn<A: Actor + Actor<Context=C>, C: AsyncContext<A>>(self, actor: &A, ctx: &mut C) {
        async move {
            let _ = self.await;
        }
            .into_actor(actor)
            .spawn(ctx);
    }
}

