use inet2_addr::ServiceAddr;
use internet2::session::LocalSession;
use internet2::{SendRecvMessage, ZmqSocketType};

#[test]
fn main() {
    let node_addr1: ServiceAddr = "inproc://zmq-test-1".parse().unwrap();
    let node_addr2 = node_addr1.clone();
    let ctx = zmq::Context::new();

    let mut session = LocalSession::connect(
        ZmqSocketType::RouterBind,
        &node_addr1,
        None,
        Some(b"rx"),
        &ctx,
    )
    .unwrap();

    let tx = std::thread::spawn(move || {
        let mut session = LocalSession::connect(
            ZmqSocketType::RouterConnect,
            &node_addr2,
            None,
            Some(b"tx"),
            &ctx,
        )
        .unwrap();
        session
            .send_routed_message(b"tx", b"rx", b"rx", b"ignored")
            .unwrap();
        let frame = session.recv_routed_message().unwrap();
        assert_eq!(frame.msg, b"hello");
        session.set_identity(&"tx_new", &ctx).unwrap();
        session
            .send_routed_message(b"tx_new", b"rx", b"rx", b"ignored")
            .unwrap();
        let frame = session.recv_routed_message().unwrap();
        assert_eq!(frame.msg, b"world");
        session
            .send_routed_message(b"tx_new", b"rx", b"rx", &[0; 1024 * 1024])
            .unwrap();
        let frame = session.recv_routed_message().unwrap();
        assert_eq!(frame.msg, [0; 1024 * 1024]);
    });

    session.recv_routed_message().unwrap();
    session
        .send_routed_message(b"rx", b"tx", b"tx", b"hello")
        .unwrap();
    session.recv_routed_message().unwrap();
    session
        .send_routed_message(b"rx", b"tx_new", b"tx_new", b"world")
        .unwrap();
    session.recv_routed_message().unwrap();
    session
        .send_routed_message(b"rx", b"tx_new", b"tx_new", &[0; 1024 * 1024])
        .unwrap();

    tx.join().unwrap();
}
