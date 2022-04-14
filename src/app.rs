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
    balance: u64,
    address: String,
}

// Define the possible messages which can be sent to the component
pub enum WasmWalletMsg {
    Balance,
    NewAddress,
    Sync,
}

impl WasmWallet {
    fn sync(&mut self) {
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

    fn balance(&mut self) {
        let wallet = Rc::clone(&self.wallet);
        let balance = wallet.as_ref().get_balance().expect("balance");
        info!("balance: {}", &balance);
        self.balance = balance;
    }

    fn new_address(&mut self) {
        let wallet = Rc::clone(&self.wallet);
        let address = wallet
            .as_ref()
            .get_address(AddressIndex::New)
            .expect("new address")
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

        let balance = 0;
        let address = "".to_string();

        let wallet = Rc::new(wallet);
        let blockchain = Rc::new(blockchain);
        WasmWallet {
            wallet,
            blockchain,
            balance,
            address,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            WasmWalletMsg::Balance => {
                self.balance();
                true // Return true to cause the displayed change to update
            }
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
        let on_balance = ctx.link().callback(|_| WasmWalletMsg::Balance);
        let on_new_address = ctx.link().callback(|_| WasmWalletMsg::NewAddress);
        html! {
            <div>
                <div>
                    <button onclick={on_sync}>{ "Sync" }</button>
                </div>
                <div>
                    <button onclick={on_balance}>{ "Balance" }</button>
                </div>
                <div>
                    { self.balance }
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
