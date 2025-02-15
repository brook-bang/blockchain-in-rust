use super::*;
use crate::block::*;
use crate::transaction::*;
use crate::utxoset::*;
use bincode::{deserialize, serialize};
use failure::format_err;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::*;
use std::thread;
use std::time::Duration;
use log::{debug, info};


#[derive(Serialize, Deserialize, Debug, Clone)]
enum Message {
    Addr(Vec<String>),
    Version(Versionmsg),
    Tx(Txmsg),
    GetData(GetDatamsg),
    GetBlock(GetBlocksmsg),
    Inv(Invmasg),
    Block(Blockmsg),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Blockmsg {
    addr_from: String,
    block: Block,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GetBlocksmsg {
    addr_from: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GetDatamsg {
    addr_from: String,
    kind: String,
    id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Invmasg {
    addr_from: String,
    kind: String,
    items: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Txmsg {
    addr_from: String,
    transaction: Transaction,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Versionmsg {
    addr_from: String,
    version: i32,
    best_height: i32,
}

pub struct Server {
    node_address: String,
    mining_address: String,
    inner: Arc<Mutex<ServerInner>>,
}

struct ServerInner {
    known_nodes: HashSet<String>,
    utxo: UTXOSet,
    blocks_in_transit: Vec<String>,
    mempool: HashMap<String, Transaction>,
}

const KNOWN_NODE1: &str = "localhost: 3000";
const CMD_LEN: usize = 12;
const VERSION: i32 = 1;

impl Server {
    pub fn new(port: &str, miner_address: &str, utxo: UTXOSet) -> Result<Server> {
        let mut node_set = HashSet::new();
        Ok(Server {
            node_address: String::from("localhost:") + port,
            mining_address: miner_address.to_string(),
            inner: Arc::new(Mutex::new(ServerInner {
                known_nodes: node_set,
                utxo,
                blocks_in_transit: Vec::new(),
                mempool: HashMap::new(),
            })),
        })
    }

    pub fn start_server(&self) -> Result<()> {
        let server1 = Server {
            node_address: self.node_address.clone(),
            mining_address: self.mining_address.clone(),
            inner: Arc::clone(&self.inner),
        };
        info!(
            "Start server as {},minning address: {}",
            &self.node_address, &self.mining_address
        );

        thread::spawn(move || {
            thread::sleep(Duration::from_millis(1000));
            if server1.get_best_height()? == -1 {
                server1.request_blocks()
            } else {
                server1.send_version(KNOWN_NODE1)
            }
        });

        let listener = TcpListener::bind(&self.node_address).unwrap();
        info!("Server listen...");

        for stream in listener.incoming() {
            let stream = stream?;
            let server1 = Server {
                node_address: self.node_address.clone(),
                mining_address: self.mining_address.clone(),
                inner: Arc::clone(&self.inner),
            };
            thread::spawn(move || server1.handle_connection(stream));
        }
        Ok(())
    }



    fn get_best_height(&self) -> Result<i32> {
        self.inner.lock().unwrap().utxo.blockchain.get_best_height()
    }

    fn send_data(&self, addr: &str, data: &[u8]) -> Result<()> {
        if addr == &self.node_address {
            return Ok(());
        }

        let mut stream = match TcpStream::connect(addr) {
            Ok(s) => s,
            Err(_) => {
                self.remove_node(addr);
                return Ok(());
            }
        };

        stream.write(data)?;
        info!("data send successfully");
        Ok(())
    }

    fn request_blocks(&self) -> Result<()> {
        for node in self.get_known_nodes() {
            self.send_get_blocks(&node)?
        }
        Ok(())
    }

    fn send_get_blocks(&self,addr: &str) -> Result<()> {
        info!("send get blocks message to: {}", addr);
        let data = GetBlocksmsg{
            addr_from: self.node_address.clone(),
        };

        let data = serialize(&(cmd_to_bytes("getblocks"),data))?;
        self.send_get_data(addr, kind, id)
    }

    fn add_nodes(&self,addr: &str) {
        self.inner.lock().unwrap().known_nodes.insert(String::from(addr));
    }

    fn remove_node(&self,addr: &str) {
        self.inner.lock().unwrap().known_nodes.remove(addr);
    }

    fn get_known_nodes(&self) -> HashSet<String> {
        self.inner.lock().unwrap().known_nodes.clone()
    }

    fn send_version(&self,addr: &str) -> Result<()> {
        info!("send version info to: {}",addr);
        let data = Versionmsg {
            addr_from: self.node_address.clone(),
            best_height: self.get_best_height()?,
            version: VERSION,
        };
        let data = serialize(&(cmd_to_bytes("version"),data))?;
        self.send_data(addr, &data)
    }

    fn handle_connection(&self,mut stream: TcpStream) -> Result<()> {
        let mut buffer = Vec::new();

        let count = stream.read_to_end(&mut buffer)?;

        let cmd = bytes_to_cmd(&buffer)?;

        match cmd {
            Message::Addr(data) => self.handle_addr(data)?,
            Message::Version(data) => self.handle_block(msg),
            Message::Tx(data) => todo!(),
            Message::GetData(data) => todo!(),
            Message::GetBlock(data) => todo!(),
            Message::Inv(data) => todo!(),
            Message::Block(data) => todo!(),
        }

        Ok(())

    }

    fn handle_addr(&self,msg: Vec<String>) -> Result<()> {
        info!("receive address msg: {:#?}",msg);
        for node in msg {
            self.add_nodes(&node);
        }
        Ok(())
    }

    fn handle_block(&self, msg: Blockmsg) -> Result<()> {
        info!(
            "receive block msg: {},{}",
            msg.addr_from,
            msg.block.get_hash()
        );
        self.add_block(msg.block)?;
        let mut in_transit = self.get_in_transit();
        if in_transit.len() > 0 {
            let block_hash = &in_transit[0];
            self.send_get_data(&msg.addr_from, "block", block_hash)?;
            in_transit.remove(0);
            self.re

        }
        Ok(())
    }

    fn replace_in_transit(&self,hashs: Vec<String>) {
        let bit = &mut self.inner.lock().unwrap().blocks_in_transit;
        bit.clone
    }

    fn send_get_data(&self,addr: &str,kind: &str,id: &str) -> Result<()> {
        info!(
            "send get data message to: {} kind: {} id: {}",
            addr,
            kind,
            id
        );
        let data = GetDatamsg {
            addr_from: self.node_address.clone(),
            kind:kind.to_string(),
            id: id.to_string(),
        };
        let data = serialize(&(cmd_to_bytes("getdata"),data))?;
        self.send_data(addr, &data)

    }

    fn add_block(&self,block: Block) -> Result<()>{
        self.inner.lock().unwrap().utxo.blockchain.add_block(block)
    }

    fn get_in_transit(&self) -> Vec<String> {
        self.inner.lock().unwrap().blocks_in_transit.clone()
    }
    
    
}
fn cmd_to_bytes(cmd: &str) -> [u8;CMD_LEN] {
    let mut data = [0;CMD_LEN];
    for (i,d) in cmd.as_bytes().iter().enumerate() {
        data[i] = *d;
    }
    data
}
 
fn bytes_to_cmd(bytes: &[u8]) -> Result<Message> {
    let mut cmd = Vec::new();
    let cmd_bytes = &bytes[..CMD_LEN];
    let data = &bytes[CMD_LEN..];
    for b in cmd_bytes {
        if 0 as u8 != *b {
            cmd.push(*b);
        }
    }

    info!("cmd:{}",String::from_utf8(cmd.clone())?);

    if cmd == "addr".as_bytes() {
        let data: Vec<String> = deserialize(data)?;
        Ok(Message::Addr(data))
    } else if cmd == "block".as_bytes() {
        let data: Blockmsg = deserialize(data)?;
        Ok(Message::Block(data))
    } else if cmd == "inv".as_bytes() {
        let data: Invmasg = deserialize(data)?;
        Ok(Message::Inv(data))
    } else if cmd == "getblocks".as_bytes() {
        let data: GetBlocksmsg = deserialize(data)?;
        Ok(Message::GetBlock(data))
    } else if cmd == "getdata".as_bytes() {
        let data: GetDatamsg = deserialize(data)?;
        Ok(Message::GetData(data))
    } else if cmd == "version".as_bytes() {
        let data: Versionmsg = deserialize(data)?;
        Ok(Message::Version(data))
    } else {
        Err(format_err!("Unknown command in the server"))
    }

}