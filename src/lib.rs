use crate::signalr_uri::{InternalUri, NoUri};

mod signalr_uri;

pub struct HubConnectionBuilder<T, P> {
    uri: T,
    protocols: P,
}

struct Protocols<W, S, L> {
    web_sockets: W,
    server_sent_events: S,
    long_polling: L,
}

impl Protocols<NoProtocol, NoProtocol, NoProtocol> {
    fn new() -> Self {
        Self {
            long_polling: NoProtocol,
            web_sockets: NoProtocol,
            server_sent_events: NoProtocol,
        }
    }
}

impl<U> HubConnectionBuilder<U, Protocols<NoProtocol, NoProtocol, NoProtocol>> {
    fn with_protocols<F,W,S,L>(self, func: F) -> HubConnectionBuilder<U, Protocols<W, S, L>>
    where
        F: FnOnce(Protocols<NoProtocol, NoProtocol, NoProtocol>) -> Protocols<W,S,L>,
    {
        HubConnectionBuilder {
            uri: self.uri,
            protocols: func(self.protocols)
        }
    }
}

impl<S, L> Protocols<NoProtocol, S, L> {
    fn use_websockets(self) -> Protocols<WebSockets, S, L> {
        Protocols {
            web_sockets: WebSockets,
            server_sent_events: self.server_sent_events,
            long_polling: self.long_polling,
        }
    }
}


impl<W, L> Protocols<W, NoProtocol, L> {
    fn use_sse(self) -> Protocols<W, ServerSentEvents, L> {
        Protocols {
            web_sockets: self.web_sockets,
            server_sent_events: ServerSentEvents,
            long_polling: self.long_polling,
        }
    }
}

impl<W, S> Protocols<W, S, NoProtocol> {
    fn use_long_polling(self) -> Protocols<W, S, LongPolling> {
        Protocols {
            web_sockets: self.web_sockets,
            server_sent_events: self.server_sent_events,
            long_polling: LongPolling,
        }
    }
}

struct NoProtocol;
struct WebSockets;
struct LongPolling;
struct ServerSentEvents;

impl HubConnectionBuilder<NoUri, Protocols<NoProtocol, NoProtocol, NoProtocol>> {
    pub fn new() -> Self {
        Self {
            uri: NoUri,
            protocols: Protocols::new(),
        }
    }
}

pub struct HubConnection<P> {
    protocols: P,
}

impl<W,S,L> HubConnection<Protocols<W,S,L>> {
    pub async fn connect(& self) {}
}

impl<'a, P> HubConnectionBuilder<NoUri, P> {
    pub fn with_uri<T>(self, uri: T) -> HubConnectionBuilder<InternalUri<'a>, P>
    where
        T: Into<InternalUri<'a>>,
    {
        HubConnectionBuilder {
            uri: uri.into(),
            protocols: self.protocols,
        }
    }
}

impl<W, S, L> HubConnectionBuilder<InternalUri<'_>, Protocols<W, S, L>> {
    pub fn build(self) -> HubConnection<Protocols<W,S,L>> {
        HubConnection{
            protocols: self.protocols
        }
    }
}

async fn bla() {
    let a = HubConnectionBuilder::new()
        .with_uri("https://www.google.com/")
        .with_protocols(|p| p
            .use_long_polling()
            .use_websockets()
            .use_sse())
        .build();

    a.connect().await;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(true);
    }
}
