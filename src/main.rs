use async_std::io;
use std::error::Error;
use libp2p::gossipsub::{
    GossipsubEvent, 
    IdentTopic as Topic, 
    MessageAuthenticity,
};
use libp2p::{
    gossipsub, 
    identity, 
    swarm::SwarmEvent, 
    Multiaddr, 
    PeerId
};
use env_logger::{
    Builder, 
    Env
};
use futures::{
    prelude::*, 
    select
};

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    println!("GOSSIP SUB");

    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {:?}", local_peer_id);

    let transport = libp2p::development_transport(local_key.clone()).await?;

    let topic = Topic::new("local-test-network");

    let mut swarm = {
        let gossipsub_config = gossipsub::GossipsubConfigBuilder::default()
            .build()
            .expect("Failed building the GOSSIPSUB config");
        
        let mut gossipsub: gossipsub::Gossipsub =
            gossipsub::Gossipsub::new(
                MessageAuthenticity::Signed(local_key), gossipsub_config)
                    .expect("Failed creating an instance of the GOSSIPSUB");

        gossipsub.subscribe(&topic).unwrap();

        // Connects to a Peer
        if let Some(explicit) = std::env::args().nth(2) {
            let explicit = explicit.clone();
            match explicit.parse() {
                Ok(id) => gossipsub.add_explicit_peer(&id),
                Err(err) => println!("Invalid peer id: {:?}", err),
            }
        }

        libp2p::Swarm::new(transport, gossipsub, local_peer_id)
    };

    // Listens across all OS assinged interfaces
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap()).unwrap();

    // Connects to another peer (if one was specified)
    if let Some(to_dial) = std::env::args().nth(1) {
        let address: Multiaddr = to_dial.parse().unwrap();
        match swarm.dial(address.clone()) {
            Ok(_) => println!("CONNECTED TO {:?}", address),
            Err(e) => println!("Dial {:?} failed: {:?}", address, e),
        };
    }

    // Read full lines from stdin
    let mut stdin = io::BufReader::new(io::stdin()).lines().fuse();

    // Kick it off
    loop {
        select! {
            line = stdin.select_next_some() => {
                if let Err(e) = swarm
                    .behaviour_mut()
                    .publish(
                        topic.clone(), 
                        line.unwrap()
                        .as_bytes())
                {
                    println!("Publish error: {:?}", e);
                }
            },
            event = swarm.select_next_some() => match event {
                SwarmEvent::Behaviour(GossipsubEvent::Message {
                    propagation_source: peer_id,
                    message_id: id,
                    message,
                }) => println!(
                    "MESSAGE RECEIVED FROM {:?} [ID: {}]: {}",
                    peer_id,
                    id,
                    String::from_utf8_lossy(&message.data)
                ),
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Listening on {:?}", address);
                }
                _ => {}
            }
        }
    }
}