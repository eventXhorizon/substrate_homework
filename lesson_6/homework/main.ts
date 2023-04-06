import { ApiPromise, WsProvider } from "@polkadot/api";
import '@polkadot/api-augment';


// const WEB_SOCKET = 'ws://localhost:9944';
const WEB_SOCKET = 'wss://rpc.polkadot.io';

const connectSubstrate = async () => {
    const wsProvider = new WsProvider(WEB_SOCKET);
    const api = await ApiPromise.create({ provider: wsProvider, types: {} });
    await api.isReady;
    console.log('connection to substrate is OK');
    return api;
}

const subscribeEvent = async (api: ApiPromise) => {
    api.query.system.events((events) => {
        console.log(`\nReceived ${events.length} events:`);

        events.forEach((record) => {
            const { event, phase } = record;
            const types = event.typeDef;

            // 只对自己感兴趣的事件进行处理，这里只针对 balances 事件进行处理
            if (event.section == 'balances') {
                console.log(`${event.section}:${event.method}::(phase=${phase.toString()})`)
                event.data.forEach((data, index) => {
                    console.log("types: ", types);
                    console.log(`--${types[index].type}:${data.toString()}`);
                })
            }
        })
    })
}

const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

const main = async () => {
    const api = await connectSubstrate();
    await subscribeEvent(api);

    await sleep(600000);
}

main().then(() => {
    console.log('done');
    process.exit(0);
}).catch(err => {
    console.log('error: ', err);
    process.exit(1);
})