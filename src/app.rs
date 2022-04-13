use bdk::blockchain::esplora::EsploraBlockchain;
use bdk::database::MemoryDatabase;
use bdk::wallet::AddressIndex;
use bdk::{bitcoin, SyncOptions, Wallet};
use log::{debug, info};
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <WasmWallet />
        </main>
    }
}

pub struct WasmWallet {
    wallet: Rc<Wallet<MemoryDatabase>>,
    blockchain: Rc<EsploraBlockchain>,
    address: String,
}

// Define the possible messages which can be sent to the component
pub enum WasmWalletMsg {
    NewAddress,
    Sync,
}

impl WasmWallet {
    fn sync(&self) {
        let wallet = Rc::clone(&self.wallet);
        let blockchain = Rc::clone(&self.blockchain);
        spawn_local(async move {
            debug!("before sync");
            wallet
                .as_ref()
                .sync(blockchain.as_ref(), SyncOptions::default())
                .await
                .expect("wallet.sync");
            debug!("after sync");
        });
    }

    fn new_address(&mut self) {
        let wallet = Rc::clone(&self.wallet);
        let address = wallet
            .as_ref()
            .get_address(AddressIndex::New)
            .expect("address")
            .address
            .to_string();
        info!("new address: {}", &address);
        self.address = address;
    }
}

impl Component for WasmWallet {
    type Message = WasmWalletMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> WasmWallet {
        let blockchain = EsploraBlockchain::new("https://blockstream.info/testnet/api", 20);
        let wallet = Wallet::new(
            "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)",
            Some("wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)"),
            bitcoin::Network::Testnet,
            MemoryDatabase::default(),
        ).expect("Wallet::new");

        let address = wallet
            .get_address(AddressIndex::New)
            .expect("address")
            .address
            .to_string();

        let wallet = Rc::new(wallet);
        let blockchain = Rc::new(blockchain);
        WasmWallet {
            wallet,
            blockchain,
            address,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            WasmWalletMsg::NewAddress => {
                self.new_address();
                true // Return true to cause the displayed change to update
            }
            WasmWalletMsg::Sync => {
                self.sync();
                true // Return true to cause the displayed change to update
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_sync = ctx.link().callback(|_| WasmWalletMsg::Sync);
        let on_new_address = ctx.link().callback(|_| WasmWalletMsg::NewAddress);
        html! {
            <div>
                <div>
                    <button onclick={on_sync}>{ "Sync" }</button>
                </div>
                <div>
                    <button onclick={on_new_address}>{ "New Address" }</button>
                </div>
                <div>
                    { self.address.clone() }
                </div>
            </div>
        }
    }
}
