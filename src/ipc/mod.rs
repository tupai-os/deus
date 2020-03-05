use core::{
    mem::MaybeUninit,
    marker::PhantomData,
};
use alloc::{
    vec::Vec,
    string::String,
    collections::{VecDeque, BTreeMap},
};
use serde::{Serialize, de::DeserializeOwned};
use crate::util::{
    irq_lock::IrqLockGuard,
    IrqLock,
    Id, Store,
};

#[derive(Debug)]
pub enum IpcError {
    NoSuchChannel,
    AlreadyTaken,
}

pub struct Channel {
    ring: VecDeque<Vec<u8>>,
    txs: usize,
    rxs: usize,
}

impl Channel {
    pub fn new(txs: usize, rxs: usize) -> Self {
        Self {
            ring: VecDeque::new(),
            txs,
            rxs,
        }
    }

    pub fn send(&mut self, bytes: Vec<u8>) {
        if self.rxs > 0 { // No need to send anything if nobody is listening
            self.ring.push_back(bytes);
        }
    }
}

#[derive(Default)]
pub struct Exchange {
    channels: Store<Channel>,
    index: BTreeMap<String, Id<Channel>>,
}

impl Exchange {
    pub fn register_tx<T>(&mut self, name: String) -> Tx<T> {
        let id = match self.index.get(&name).copied() {
            Some(id) => {
                self.channels.get_mut(id).txs += 1;
                id
            },
            None => {
                let id = self.channels.insert(Channel::new(1, 0));
                self.index.insert(name, id);
                id
            },
        };
        Tx(id, PhantomData)
    }

    pub fn register_rx<T>(&mut self, name: String) -> Rx<T> {
        let id = match self.index.get(&name).copied() {
            Some(id) => {
                self.channels.get_mut(id).rxs += 1;
                id
            },
            None => {
                let id = self.channels.insert(Channel::new(0, 1));
                self.index.insert(name, id);
                id
            },
        };
        Rx(id, PhantomData)
    }

    pub fn connect_tx<T>(&mut self, name: &str) -> Result<Tx<T>, IpcError> {
        let id = *self.index
            .get(name)
            .ok_or(IpcError::NoSuchChannel)?;
        self.channels.get_mut(id).txs += 1;
        Ok(Tx(id, PhantomData))
    }

    pub fn connect_rx<T>(&mut self, name: &str) -> Result<Rx<T>, IpcError> {
        let id = *self.index
            .get(name)
            .ok_or(IpcError::NoSuchChannel)?;
        self.channels.get_mut(id).rxs += 1;
        Ok(Rx(id, PhantomData))
    }

    pub fn list(&self) -> impl Iterator<Item=&str> {
        self.index.keys().map(|s| s.as_str())
    }
}

static mut EXCHANGE: MaybeUninit<IrqLock<Exchange>> = MaybeUninit::uninit();

pub fn init_exchange() {
    unsafe { EXCHANGE.write(IrqLock::new(Exchange::default())); }
}

pub fn exchange<'a>() -> IrqLockGuard<'a, Exchange> {
    unsafe { EXCHANGE.get_mut() }.lock()
}

#[derive(Clone)]
pub struct Tx<T>(Id<Channel>, PhantomData<T>);

impl<T> Tx<T> {
    pub fn send(&self, msg: T) -> Result<(), IpcError> where T: Serialize {
        let mut exchange = unsafe { EXCHANGE.get_mut() }.lock();

        exchange
            .channels
            .get_mut(self.0)
            .send(pinecone::to_vec(&msg).unwrap());

        Ok(())
    }
}

impl<T> Drop for Tx<T> {
    fn drop(&mut self) {
        exchange()
            .channels
            .get_mut(self.0);
    }
}

pub struct Rx<T>(Id<Channel>, PhantomData<T>);

impl<T> Rx<T> {
    pub fn recv(&self) -> Result<T, IpcError> where T: DeserializeOwned {
        let mut exchange = unsafe { EXCHANGE.get_mut() }.lock();

        loop {
            if let Some(bytes) = exchange
                .channels
                .get_mut(self.0)
                .ring
                .pop_front()
            {
                return Ok(pinecone::from_bytes(&bytes).unwrap())
            }
        }
    }
}
