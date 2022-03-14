# p2p-chat-network
A peer-to-peer chatting network using libp2p.rs and gossipsub.

## Instructions

Clone the code repository locally and open a terminal window.
Navigate to the project directory and run it using:

``` cargo run ```

It will print the [`PeerId`] and the listening addresses. 
e.g. `Listening on //! "/ip4/127.0.0.1/tcp/12345"`
( Notice that multiple messages may appear depending on how many network interfaces your OS is assigning. )

Now, open a new terminal, and execute the project using:

```cargo run -- /ip4/127.0.0.1/tcp/12345 ```

In each prompt, type a message and you should see the result appear on every other terminal session running other peers.