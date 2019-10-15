use actix::prelude::*;
use actix_raft::{RaftNetwork, messages};

use crate::network::{Network, SendRaftMessage};
use crate::raft::{
    storage::{
        MemoryStorageData as Data
    }
};


const ERR_ROUTING_FAILURE: &str = "Routing failures are not allowed in tests.";

impl RaftNetwork<Data> for Network {}

impl Handler<messages::AppendEntriesRequest<Data>> for Network {
    type Result = ResponseActFuture<Self, messages::AppendEntriesResponse, ()>;

    fn handle(&mut self, msg: messages::AppendEntriesRequest<Data>, ctx: &mut Context<Self>) -> Self::Result {
        let node = self.get_node(msg.target).unwrap();
        let req = node.send(SendRaftMessage(msg));

        Box::new(fut::wrap_future(req)
            .map_err(|_, _, _| panic!(ERR_ROUTING_FAILURE))
            .and_then(|res, _, _| fut::result(res)))
    }
}

impl Handler<messages::VoteRequest> for Network {
    type Result = ResponseActFuture<Self, messages::VoteResponse, ()>;

    fn handle(&mut self, msg: messages::VoteRequest, ctx: &mut Context<Self>) -> Self::Result {
        let node = self.get_node(msg.target).unwrap();
        let req = node.send(SendRaftMessage(msg));

        Box::new(fut::wrap_future(req)
            .map_err(|_, _, _| panic!(ERR_ROUTING_FAILURE))
            .and_then(|res, _, _| fut::result(res)))
    }
}

impl Handler<messages::InstallSnapshotRequest> for Network {
    type Result = ResponseActFuture<Self, messages::InstallSnapshotResponse, ()>;

    fn handle(&mut self, msg: messages::InstallSnapshotRequest, ctx: &mut Context<Self>) -> Self::Result {
        let node = self.get_node(msg.target).unwrap();
        let req = node.send(SendRaftMessage(msg));

        Box::new(fut::wrap_future(req)
            .map_err(|_, _, _| panic!(ERR_ROUTING_FAILURE))
            .and_then(|res, _, _| fut::result(res)))
    }
}