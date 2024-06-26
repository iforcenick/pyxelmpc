use libp2p::gossipsub::MessageId;
use libp2p::gossipsub::{
    Gossipsub, GossipsubEvent, GossipsubMessage, IdentTopic as Topic, MessageAuthenticity,
    ValidationMode,
};
use libp2p::swarm::keep_alive;
use libp2p::{
    gossipsub, identity, mdns, swarm::NetworkBehaviour, swarm::SwarmEvent, PeerId, Swarm,
};
use tokio::sync::Notify;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Duration;

use anyhow::{Result};
use futures::{Sink, Stream, StreamExt};
use std::{cell::RefCell, rc::Rc};

#[derive(NetworkBehaviour)]
pub struct MpcPubsubBahavior {
    pub gossipsub: Gossipsub,
    pub mdns: mdns::async_io::Behaviour,
    pub keep_alive: keep_alive::Behaviour,
}

pub struct MpcPubsub {
    node: Rc<RefCell<Swarm<MpcPubsubBahavior>>>,

    notifier: Notify,
}

impl MpcPubsub {
    pub async fn new() -> Result<Self> {
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        println!("Local peer id: {local_peer_id}");

        let transport = libp2p::development_transport(local_key.clone()).await?;

        // Placeholder
        let message_id_fn = |message: &GossipsubMessage| {
            let mut s = DefaultHasher::new();
            message.data.hash(&mut s);
            MessageId::from(s.finish().to_string())
        };

        let gossipsub_config = gossipsub::GossipsubConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(10)) // This is set to aid debugging by not cluttering the log space
            .validation_mode(ValidationMode::Strict) // This sets the kind of message validation. The default is Strict (enforce message signing)
            .message_id_fn(message_id_fn) // content-address messages. No two messages of the same content will be propagated.
            .build()
            .expect("Valid config");

        // let topic = Topic::new(topic);
        let gossipsub = Gossipsub::new(MessageAuthenticity::Signed(local_key.clone()), gossipsub_config)
            .expect("Correct configuration");
        // gossipsub.subscribe(&topic)?;

        let swarm = {
            let mdns = mdns::async_io::Behaviour::new(mdns::Config::default())?;
            let behaviour = MpcPubsubBahavior { gossipsub, mdns, keep_alive: keep_alive::Behaviour::default() };
            Swarm::with_async_std_executor(transport, behaviour, local_peer_id)
        };

        Ok(Self {
            node: Rc::new(RefCell::new(swarm)),
            notifier: Notify::new(),
        })
    }

    pub fn start(&mut self, port: i32) -> Result<()> {
        self.node
            .borrow_mut()
            .listen_on(format!("/ip4/0.0.0.0/tcp/{}", port).parse()?)?;
        Ok(())
    }

    pub fn listen_topic(&mut self, topic: &str) -> Result<()> {
        self.node
            .borrow_mut()
            .behaviour_mut()
            .gossipsub
            .subscribe(&Topic::new(topic))?;
        Ok(())
    }

    pub fn process (&mut self, topic: &str) -> Result<(
        impl Stream<Item = Option<Vec<u8>> > + '_, //incoming
        impl Sink<Vec<u8>, Error = anyhow::Error> + '_, // outgoing
    )> {

        let mut original_stream = self.node.borrow_mut();
        original_stream.behaviour_mut().gossipsub.subscribe(&Topic::new(topic))?;

        let notifier = &self.notifier;

        let q = Rc::new(RefCell::new(Vec::new()));
        let outgoing = futures::sink::unfold(
            q.clone(),
            |v, message: Vec<u8> | {
                eprintln!("New Message {:?}", message);
                v.borrow_mut().push(message);
                self.notifier.notify_waiters();
                futures::future::ready(Ok(v))
            }
        );

        let incoming = async_stream::stream!{
            loop {
                tokio::select! {
                    event = original_stream.select_next_some() => {
                        match event {
                            SwarmEvent::Behaviour(MpcPubsubBahaviorEvent::Mdns(mdns::Event::Discovered(list))) => {
                                for (peer_id, _multiaddr) in list {
                                    println!("mDNS discovered a new peer: {peer_id}");
                                    original_stream.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                                }
                                
                                yield None
                            },
                            SwarmEvent::Behaviour(MpcPubsubBahaviorEvent::Mdns(mdns::Event::Expired(list))) => {
                                println!("Left {:?}", list);
                                for (peer_id, _multiaddr) in list {
                                    println!("mDNS discover peer has expired: {peer_id}");
                                    original_stream.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                                }
        
                                yield None
                            },
                            SwarmEvent::Behaviour(MpcPubsubBahaviorEvent::Gossipsub(GossipsubEvent::Message {
                                propagation_source: peer_id,
                                message_id: id,
                                message,
                            })) => {
                                eprintln!("Got message From {:?}, with ID {:?}", peer_id, id);
                                yield Some(message.data.clone())
                            },
                            _ => {
                                yield None
                            }
                        }
                    }
                    _ = notifier.notified() => {
                        // eprintln!("Sending Message {:?}", q.clone());
                        for msg in q.borrow().iter() {
                            let res = original_stream
                                .behaviour_mut()
                                .gossipsub
                                .publish(Topic::new("test"), msg.to_owned());
                            eprintln!("Publishing Result: {:?}", res);
                        }

                        *q.borrow_mut() = Vec::new();
                    }
                }
            }
        };

        Ok((incoming, outgoing))
    }

    pub fn outgoing_sink(
        &self,
    ) -> Result<
        impl Sink<Vec<u8>, Error = anyhow::Error> + '_, // outgoing
    > {
        let outgoing = futures::sink::unfold(
            Vec::new(),
            |mut v, message: Vec<u8> | {
                v.push(message);
                self.notifier.notify_waiters();
                futures::future::ready(Ok(v))
            }
        );

        Ok(outgoing)
    }

}
